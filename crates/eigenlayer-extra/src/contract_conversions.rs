use eigensdk::crypto_bls::{BlsG1Point, BlsG2Point};
use eigensdk::services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;

/// Trait for contract-specific G1 point type
pub trait ContractG1Point: Sized + Clone + Send + Sync + 'static {
    type X;
    type Y;

    /// Create a new G1 point with the given coordinates
    fn new(x: Self::X, y: Self::Y) -> Self;
}

/// Trait for contract-specific G2 point type
pub trait ContractG2Point: Sized + Clone + Send + Sync + 'static {
    type X;
    type Y;

    /// Create a new G2 point with the given coordinates
    fn new(x: Self::X, y: Self::Y) -> Self;
}

/// Trait for non-signer stakes and signature type used in contract calls
pub trait NonSignerStakesAndSignature<G1, G2>: Sized + Clone + Send + Sync + 'static
where
    G1: ContractG1Point,
    G2: ContractG2Point,
{
    /// Create a new instance with the given parameters
    fn new(
        non_signer_pubkeys: Vec<G1>,
        non_signer_quorum_bitmap_indices: Vec<u32>,
        quorum_apks: Vec<G1>,
        apk_g2: G2,
        sigma: G1,
        quorum_apk_indices: Vec<u32>,
        total_stake_indices: Vec<u32>,
        non_signer_stake_indices: Vec<Vec<u32>>,
    ) -> Self;
}

/// Converts a BlsG1Point to a contract-compatible G1 point
pub fn convert_to_contract_g1<G1: ContractG1Point>(
    point: &BlsG1Point,
    convert_g1: impl FnOnce(&BlsG1Point) -> (G1::X, G1::Y),
) -> G1 {
    let (x, y) = convert_g1(point);
    G1::new(x, y)
}

/// Converts a BlsG2Point to a contract-compatible G2 point
pub fn convert_to_contract_g2<G2: ContractG2Point>(
    point: &BlsG2Point,
    convert_g2: impl FnOnce(&BlsG2Point) -> (G2::X, G2::Y),
) -> G2 {
    let (x, y) = convert_g2(point);
    G2::new(x, y)
}

/// Converts a BlsAggregationServiceResponse to a contract-compatible NonSignerStakesAndSignature
pub fn convert_aggregation_response<G1, G2, NSS>(
    response: &BlsAggregationServiceResponse,
    convert_g1: impl Fn(&BlsG1Point) -> (G1::X, G1::Y) + Copy,
    convert_g2: impl Fn(&BlsG2Point) -> (G2::X, G2::Y),
) -> NSS
where
    G1: ContractG1Point,
    G2: ContractG2Point,
    NSS: NonSignerStakesAndSignature<G1, G2>,
{
    // Convert non-signer pubkeys
    let non_signer_pubkeys = response
        .non_signers_pub_keys_g1
        .iter()
        .map(|pk| convert_to_contract_g1(pk, convert_g1))
        .collect();

    // Convert quorum APKs
    let quorum_apks = response
        .quorum_apks_g1
        .iter()
        .map(|pk| convert_to_contract_g1(pk, convert_g1))
        .collect();

    // Convert APK G2
    let apk_g2 = convert_to_contract_g2(&response.signers_apk_g2, convert_g2);

    // Convert signature G1
    let sigma = convert_to_contract_g1(&response.signers_agg_sig_g1.g1_point(), convert_g1);

    NSS::new(
        non_signer_pubkeys,
        response.non_signer_quorum_bitmap_indices.clone(),
        quorum_apks,
        apk_g2,
        sigma,
        response.quorum_apk_indices.clone(),
        response.total_stake_indices.clone(),
        response.non_signer_stake_indices.clone(),
    )
}
