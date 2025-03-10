//! Simple protocol in which parties cooperate to generate randomness

mod common;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, digest::Output};

use round_based::rounds_router::{
    CompleteRoundError, RoundsRouter,
    simple_store::{RoundInput, RoundInputError},
};
use round_based::{Delivery, Mpc, MpcParty, MsgId, Outgoing, PartyIndex, ProtocolMessage, SinkExt};

/// Protocol message
#[derive(Clone, Debug, PartialEq, ProtocolMessage, Serialize, Deserialize)]
pub enum Msg {
    /// Round 1
    CommitMsg(CommitMsg),
    /// Round 2
    DecommitMsg(DecommitMsg),
}

/// Message from round 1
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitMsg {
    /// Party commitment
    pub commitment: Output<Sha256>,
}

/// Message from round 2
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecommitMsg {
    /// Randomness generated by party
    pub randomness: [u8; 32],
}

/// Carries out the randomness generation protocol
#[tracing::instrument(skip(party, rng))]
pub async fn protocol_of_random_generation<R, M>(
    party: M,
    i: PartyIndex,
    n: u16,
    mut rng: R,
) -> Result<[u8; 32], Error<M::ReceiveError, M::SendError>>
where
    M: Mpc<ProtocolMessage = Msg>,
    R: rand::RngCore,
{
    let MpcParty { delivery, .. } = party.into_party();
    let (incoming, mut outgoing) = delivery.split();

    // Define rounds
    let mut rounds = RoundsRouter::<Msg>::builder();
    let round1 = rounds.add_round(RoundInput::<CommitMsg>::broadcast(i, n));
    let round2 = rounds.add_round(RoundInput::<DecommitMsg>::broadcast(i, n));
    let mut rounds = rounds.listen(incoming);

    // --- The Protocol ---

    // 1. Generate local randomness
    let mut local_randomness = [0u8; 32];
    rng.fill_bytes(&mut local_randomness);

    tracing::debug!(local_randomness = %hex::encode(local_randomness), "Generated local randomness");

    // 2. Commit local randomness (broadcast m=sha256(randomness))
    let commitment = Sha256::digest(local_randomness);
    tracing::debug!(commitment = %hex::encode(commitment), "Committed local randomness");
    outgoing
        .send(Outgoing::broadcast(Msg::CommitMsg(CommitMsg {
            commitment,
        })))
        .await
        .map_err(Error::Round1Send)?;

    tracing::debug!("Sent commitment and waiting for others to send theirs");

    // 3. Receive committed randomness from other parties
    let commitments = rounds
        .complete(round1)
        .await
        .map_err(Error::Round1Receive)?;

    tracing::debug!("Received commitments from all parties");

    // 4. Open local randomness
    tracing::debug!("Opening local randomness");
    outgoing
        .send(Outgoing::broadcast(Msg::DecommitMsg(DecommitMsg {
            randomness: local_randomness,
        })))
        .await
        .map_err(Error::Round2Send)?;

    tracing::debug!("Sent decommitment and waiting for others to send theirs");

    // 5. Receive opened local randomness from other parties, verify them, and output protocol randomness
    let randomness = rounds
        .complete(round2)
        .await
        .map_err(Error::Round2Receive)?;

    tracing::debug!("Received decommitments from all parties");

    let mut guilty_parties = vec![];
    let mut output = local_randomness;
    for ((party_i, com_msg_id, commit), (_, decom_msg_id, decommit)) in commitments
        .into_iter_indexed()
        .zip(randomness.into_iter_indexed())
    {
        let commitment_expected = Sha256::digest(decommit.randomness);
        if commit.commitment != commitment_expected {
            guilty_parties.push(Blame {
                guilty_party: party_i,
                commitment_msg: com_msg_id,
                decommitment_msg: decom_msg_id,
            });
            continue;
        }

        output
            .iter_mut()
            .zip(decommit.randomness)
            .for_each(|(x, r)| *x ^= r);
    }

    if guilty_parties.is_empty() {
        tracing::debug!(output = %hex::encode(output), "Generated randomness");
        tracing::info!("Randomness generation protocol completed successfully.");
        Ok(output)
    } else {
        tracing::error!(guilty_parties = ?guilty_parties, "Some parties cheated");
        Err(Error::PartiesOpenedRandomnessDoesntMatchCommitment { guilty_parties })
    }
}

/// Protocol error
#[derive(Debug, thiserror::Error)]
pub enum Error<RecvErr, SendErr> {
    /// Couldn't send a message in the first round
    #[error("send a message at round 1")]
    Round1Send(#[source] SendErr),
    /// Couldn't receive a message in the first round
    #[error("receive messages at round 1")]
    Round1Receive(#[source] CompleteRoundError<RoundInputError, RecvErr>),
    /// Couldn't send a message in the second round
    #[error("send a message at round 2")]
    Round2Send(#[source] SendErr),
    /// Couldn't receive a message in the second round
    #[error("receive messages at round 2")]
    Round2Receive(#[source] CompleteRoundError<RoundInputError, RecvErr>),

    /// Some of the parties cheated
    #[error("malicious parties: {guilty_parties:?}")]
    PartiesOpenedRandomnessDoesntMatchCommitment {
        /// List of cheated parties
        guilty_parties: Vec<Blame>,
    },
}

/// Blames a party in cheating during the protocol
#[derive(Debug)]
pub struct Blame {
    /// Index of the cheated party
    pub guilty_party: PartyIndex,
    /// ID of the message that party sent in the first round
    pub commitment_msg: MsgId,
    /// ID of the message that party sent in the second round
    pub decommitment_msg: MsgId,
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::time::Duration;

    use super::common::*;
    use gadget_crypto::{KeyType, sp_core::SpEcdsa};
    use gadget_networking::discovery::peers::VerificationIdentifierKey;
    use gadget_networking::service::AllowedKeys;
    use gadget_networking_round_based_extension::RoundBasedNetworkAdapter;
    use round_based::MpcParty;
    use tracing::{debug, info};

    use super::protocol_of_random_generation;

    #[test]
    fn simulation() {
        let mut rng = rand_dev::DevRng::new();

        let n: u16 = 5;

        let randomness = round_based::sim::run_with_setup(
            core::iter::repeat_with(|| rng.fork()).take(n.into()),
            |i, party, rng| protocol_of_random_generation(party, i, n, rng),
        )
        .unwrap()
        .expect_ok()
        .expect_eq();

        std::println!("Output randomness: {}", hex::encode(randomness));
    }

    #[tokio::test]
    async fn simulation_async() {
        let mut rng = rand_dev::DevRng::new();

        let n: u16 = 5;

        let randomness = round_based::sim::async_env::run_with_setup(
            core::iter::repeat_with(|| rng.fork()).take(n.into()),
            |i, party, rng| protocol_of_random_generation(party, i, n, rng),
        )
        .await
        .expect_ok()
        .expect_eq();

        std::println!("Output randomness: {}", hex::encode(randomness));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn p2p_networking() {
        init_tracing();
        let network_name = "rand-test-network";
        let instance_id = "rand-test-instance";

        // Generate node2's key pair first
        let instance_key_pair2 = SpEcdsa::generate_with_seed(None).unwrap();
        let mut allowed_keys1 = HashSet::new();
        allowed_keys1.insert(instance_key_pair2.public());

        // Create node1 with node2's key whitelisted
        let mut node1 = TestNode::<SpEcdsa>::new(
            network_name,
            instance_id,
            AllowedKeys::InstancePublicKeys(allowed_keys1),
            vec![],
            false,
        );

        // Create node2 with node1's key whitelisted and pre-generated key
        let mut allowed_keys2 = HashSet::new();
        allowed_keys2.insert(node1.instance_key_pair.public());
        let mut node2 = TestNode::<SpEcdsa>::new_with_keys(
            network_name,
            instance_id,
            AllowedKeys::InstancePublicKeys(allowed_keys2),
            vec![],
            Some(instance_key_pair2),
            None,
            false,
        );

        info!("Starting nodes");
        // Start both nodes - this should trigger automatic handshake
        let handle1 = node1.start().await.expect("Failed to start node1");
        let handle2 = node2.start().await.expect("Failed to start node2");

        wait_for_peer_discovery(&[&handle1, &handle2], Duration::from_secs(5))
            .await
            .unwrap();

        let parties = HashMap::from_iter([
            (
                0,
                VerificationIdentifierKey::InstancePublicKey(node1.instance_key_pair.public()),
            ),
            (
                1,
                VerificationIdentifierKey::InstancePublicKey(node2.instance_key_pair.public()),
            ),
        ]);

        let node1_network = RoundBasedNetworkAdapter::new(handle1, 0, parties.clone(), instance_id);
        let node2_network = RoundBasedNetworkAdapter::new(handle2, 1, parties, instance_id);

        let mut tasks = vec![];
        tasks.push(tokio::spawn(async move {
            let mut rng = rand_dev::DevRng::new();
            let mpc_party = MpcParty::connected(node1_network);
            let randomness = protocol_of_random_generation(mpc_party, 0, 2, &mut rng)
                .await
                .expect("Failed to generate randomness");
            debug!("Node1 generated randomness: {:?}", randomness);
            randomness
        }));

        tasks.push(tokio::spawn(async move {
            let mut rng = rand_dev::DevRng::new();
            let mpc_party = MpcParty::connected(node2_network);
            let randomness = protocol_of_random_generation(mpc_party, 1, 2, &mut rng)
                .await
                .expect("Failed to generate randomness");
            debug!("Node2 generated randomness: {:?}", randomness);
            randomness
        }));

        let results = futures::future::join_all(tasks).await;

        for result in results {
            match result {
                Ok(randomness) => {
                    debug!("Randomness result: {:?}", randomness);
                }
                Err(e) => {
                    panic!("Error in randomness generation: {:?}", e);
                }
            }
        }
    }
}
