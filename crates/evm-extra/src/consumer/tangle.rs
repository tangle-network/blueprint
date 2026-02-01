//! Tangle-specific job consumer for EVM
//!
//! This module provides a consumer that submits job results to the Tangle Jobs contract
//! by calling the `submitResult` function.

use alloc::collections::VecDeque;
use alloy_primitives::{Address, Bytes as AlloyBytes, TxHash};
use alloy_provider::fillers::{FillProvider, JoinFill, RecommendedFillers, WalletFiller};
use alloy_provider::network::{Ethereum, EthereumWallet, NetworkWallet};
use alloy_provider::{Network, Provider, RootProvider};
use alloy_sol_types::{SolCall, sol};
use alloy_transport::TransportError;
use blueprint_core::JobResult;
use blueprint_core::error::BoxError;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::Sink;

use crate::extract::{CallId, ServiceId};

// Define the submitResult function ABI
sol! {
    /// Submit job result to the Jobs contract
    #[derive(Debug)]
    function submitResult(
        uint64 serviceId,
        uint64 callId,
        bytes output
    ) external;
}

/// A type alias for the recommended fillers of a network.
pub type RecommendedFillersOf<T> = <T as RecommendedFillers>::RecommendedFillers;

/// A type alias for the Alloy provider with wallet.
pub type AlloyProviderWithWallet<N, W> =
    FillProvider<JoinFill<RecommendedFillersOf<N>, WalletFiller<W>>, RootProvider<N>, N>;

/// Represents a parsed job result ready for submission
#[derive(Debug)]
struct DerivedJobResult {
    service_id: u64,
    call_id: u64,
    output: AlloyBytes,
}

enum State {
    WaitingForResult,
    ProcessingTransaction(Pin<Box<dyn Future<Output = Result<TxHash, TransportError>> + Send>>),
}

impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WaitingForResult => write!(f, "WaitingForResult"),
            Self::ProcessingTransaction(_) => f.debug_tuple("ProcessingTransaction").finish(),
        }
    }
}

impl State {
    fn is_waiting(&self) -> bool {
        matches!(self, State::WaitingForResult)
    }
}

/// Tangle Consumer for submitting job results to the Jobs contract
///
/// This consumer receives `JobResult`s from job handlers and submits them to the
/// Tangle Jobs contract by calling `submitResult(serviceId, callId, output)`.
#[derive(Debug)]
pub struct TangleConsumer<W = EthereumWallet, N = Ethereum>
where
    W: NetworkWallet<N> + Clone,
    N: Network + RecommendedFillers,
{
    provider: AlloyProviderWithWallet<N, W>,
    contract_address: Address,
    buffer: VecDeque<DerivedJobResult>,
    state: State,
}

impl<W, N> TangleConsumer<W, N>
where
    N: Network + RecommendedFillers,
    W: NetworkWallet<N> + Clone,
{
    /// Create a new [`TangleConsumer`]
    ///
    /// # Arguments
    /// * `provider` - The EVM provider to use for sending transactions
    /// * `wallet` - The wallet to sign transactions with
    /// * `contract_address` - The address of the Tangle Jobs contract
    pub fn new<P: Provider<N>>(provider: P, wallet: W, contract_address: Address) -> Self {
        let p = FillProvider::new(provider.root().clone(), N::recommended_fillers())
            .join_with(WalletFiller::new(wallet));
        Self {
            provider: p,
            contract_address,
            buffer: VecDeque::new(),
            state: State::WaitingForResult,
        }
    }

    /// Get the contract address being used
    pub fn contract_address(&self) -> Address {
        self.contract_address
    }
}

impl<W, N> Sink<JobResult> for TangleConsumer<W, N>
where
    N: RecommendedFillers + Unpin + 'static,
    N::TransactionRequest: From<alloy_rpc_types::TransactionRequest>,
    <N as RecommendedFillers>::RecommendedFillers: Unpin + 'static,
    W: NetworkWallet<N> + Clone + Unpin + 'static,
{
    type Error = BoxError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: JobResult) -> Result<(), Self::Error> {
        let this = self.get_mut();

        let JobResult::Ok { head, body } = &item else {
            blueprint_core::trace!(target: "tangle-consumer", "Discarding job result with error");
            return Ok(());
        };

        // Extract service_id and call_id from metadata
        let (Some(service_id_raw), Some(call_id_raw)) = (
            head.metadata.get(ServiceId::METADATA_KEY),
            head.metadata.get(CallId::METADATA_KEY),
        ) else {
            blueprint_core::trace!(
                target: "tangle-consumer",
                "Discarding job result with missing metadata (not a Tangle job)"
            );
            return Ok(());
        };

        let service_id: u64 = service_id_raw.try_into().map_err(|_| {
            blueprint_core::error!(target: "tangle-consumer", "Invalid service_id in metadata");
            "Invalid service_id"
        })?;
        let call_id: u64 = call_id_raw.try_into().map_err(|_| {
            blueprint_core::error!(target: "tangle-consumer", "Invalid call_id in metadata");
            "Invalid call_id"
        })?;

        blueprint_core::debug!(
            target: "tangle-consumer",
            service_id,
            call_id,
            output_len = body.len(),
            "Received job result, queuing for submission"
        );

        this.buffer.push_back(DerivedJobResult {
            service_id,
            call_id,
            output: AlloyBytes::copy_from_slice(body),
        });

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.get_mut();

        if this.buffer.is_empty() && this.state.is_waiting() {
            return Poll::Ready(Ok(()));
        }

        loop {
            match &mut this.state {
                State::WaitingForResult => {
                    let Some(derived) = this.buffer.pop_front() else {
                        return Poll::Ready(Ok(()));
                    };

                    blueprint_core::debug!(
                        target: "tangle-consumer",
                        service_id = derived.service_id,
                        call_id = derived.call_id,
                        "Submitting result to Jobs contract"
                    );

                    let fut = send_submit_result(
                        this.provider.clone(),
                        this.contract_address,
                        derived.service_id,
                        derived.call_id,
                        derived.output,
                    );

                    this.state = State::ProcessingTransaction(Box::pin(fut));
                }
                State::ProcessingTransaction(fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(tx_hash)) => {
                        blueprint_core::info!(
                            target: "tangle-consumer",
                            %tx_hash,
                            "Successfully submitted job result"
                        );
                        this.state = State::WaitingForResult;
                    }
                    Poll::Ready(Err(err)) => {
                        blueprint_core::error!(
                            target: "tangle-consumer",
                            ?err,
                            "Failed to submit job result"
                        );
                        // Reset state to allow retries
                        this.state = State::WaitingForResult;
                        return Poll::Ready(Err(err.into()));
                    }
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.buffer.is_empty() {
            Poll::Ready(Ok(()))
        } else {
            self.poll_flush(cx)
        }
    }
}

/// Send submitResult transaction to the Jobs contract
async fn send_submit_result<N, P>(
    provider: P,
    contract_address: Address,
    service_id: u64,
    call_id: u64,
    output: AlloyBytes,
) -> Result<TxHash, TransportError>
where
    N: Network,
    N::TransactionRequest: From<alloy_rpc_types::TransactionRequest>,
    P: Provider<N>,
{
    use alloy_provider::network::ReceiptResponse;
    use alloy_transport::TransportErrorKind;

    // Encode the submitResult function call
    let call = submitResultCall {
        serviceId: service_id,
        callId: call_id,
        output,
    };
    let calldata = call.abi_encode();

    // Create transaction request
    let tx_request = alloy_rpc_types::TransactionRequest::default()
        .to(contract_address)
        .input(calldata.into());

    blueprint_core::trace!(
        target: "tangle-consumer",
        service_id,
        call_id,
        contract = %contract_address,
        "Sending submitResult transaction"
    );

    let res = provider.send_transaction(tx_request.into()).await;
    let receipt_res = match res {
        Ok(pending_tx) => pending_tx.get_receipt().await,
        Err(err) => {
            blueprint_core::error!(
                target: "tangle-consumer",
                ?err,
                service_id,
                call_id,
                "Failed to send submitResult transaction"
            );
            return Err(err);
        }
    };

    let receipt = match receipt_res {
        Ok(receipt) => receipt,
        Err(err) => {
            blueprint_core::error!(
                target: "tangle-consumer",
                ?err,
                service_id,
                call_id,
                "Pending transaction failed"
            );
            return Err(TransportError::Transport(TransportErrorKind::Custom(
                Box::new(err),
            )));
        }
    };

    blueprint_core::info!(
        target: "tangle-consumer",
        status = %receipt.status(),
        block_hash = ?receipt.block_hash(),
        block_number = ?receipt.block_number(),
        transaction_hash = %receipt.transaction_hash(),
        gas_used = %receipt.gas_used(),
        service_id,
        call_id,
        "submitResult transaction confirmed"
    );

    Ok(receipt.transaction_hash())
}
