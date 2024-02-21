use crate::protocols::key_refresh::DfnsCGGMP21KeyRefreshExtraParams;
use crate::protocols::key_rotate::DfnsCGGMP21KeyRotateExtraParams;
use crate::protocols::keygen::DfnsCGGMP21KeygenExtraParams;
use crate::protocols::sign::DfnsCGGMP21SigningExtraParams;
use async_trait::async_trait;
use gadget_common::client::{AccountId, ClientWithApi, JobsClient, PalletSubmitter};
use gadget_common::client::{GadgetJobType, JobsApiForGadget};
use gadget_common::debug_logger::DebugLogger;
use gadget_common::full_protocol::SharedOptional;
use gadget_common::gadget::network::Network;
use gadget_common::gadget::JobInitMetadata;
use gadget_common::keystore::{ECDSAKeyStore, KeystoreBackend};
use gadget_common::prelude::*;
use gadget_common::prelude::{FullProtocolConfig, GadgetProtocolMessage, Mutex, WorkManager};
use gadget_common::{generate_protocol, generate_setup_and_run_command, Error};
use gadget_core::job::{BuiltExecutableJobWrapper, JobError};
use gadget_core::job_manager::{ProtocolWorkManager, WorkManagerInterface};
use protocol_macros::protocol;
use sc_client_api::Backend;
use sp_api::ProvideRuntimeApi;
use sp_runtime::traits::Block;
use std::sync::Arc;
use tangle_primitives::roles::{RoleType, ThresholdSignatureRoleType};
use test_utils::generate_signing_and_keygen_tss_tests;
use tokio::sync::mpsc::UnboundedReceiver;

pub mod constants;
pub mod error;
pub mod protocols;
pub mod util;

generate_protocol!(
    "DFNS-Keygen-Protocol",
    DfnsKeygenProtocol,
    DfnsCGGMP21KeygenExtraParams,
    protocols::keygen::generate_protocol_from,
    protocols::keygen::create_next_job,
    GadgetJobType::DKGTSSPhaseOne(_),
    RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
);
generate_protocol!(
    "DFNS-Signing-Protocol",
    DfnsSigningProtocol,
    DfnsCGGMP21SigningExtraParams,
    protocols::sign::generate_protocol_from,
    protocols::sign::create_next_job,
    GadgetJobType::DKGTSSPhaseTwo(_),
    RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
);
generate_protocol!(
    "DFNS-Refresh-Protocol",
    DfnsKeyRefreshProtocol,
    DfnsCGGMP21KeyRefreshExtraParams,
    protocols::key_refresh::generate_protocol_from,
    protocols::key_refresh::create_next_job,
    GadgetJobType::DKGTSSPhaseThree(_),
    RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
);
generate_protocol!(
    "DFNS-Rotate-Protocol",
    DfnsKeyRotateProtocol,
    DfnsCGGMP21KeyRotateExtraParams,
    protocols::key_rotate::generate_protocol_from,
    protocols::key_rotate::create_next_job,
    GadgetJobType::DKGTSSPhaseFour(_),
    RoleType::Tss(ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1)
);

generate_setup_and_run_command!(
    DfnsKeygenProtocol,
    DfnsSigningProtocol,
    DfnsKeyRefreshProtocol,
    DfnsKeyRotateProtocol
);

generate_signing_and_keygen_tss_tests!(2, 3, 4, ThresholdSignatureRoleType::DfnsCGGMP21Secp256k1);
