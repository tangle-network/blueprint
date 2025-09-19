//! ![Tangle Network Banner](https://raw.githubusercontent.com/tangle-network/tangle/refs/heads/main/assets/Tangle%20%20Banner.png)
//!
//! <h1 align="center">Blueprint Manager</h1>
//!
//! ## Features
#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://cdn.prod.website-files.com/6494562b44a28080aafcbad4/65aaf8b0818b1d504cbdf81b_Tnt%20Logo.png"
)]
#![allow(clippy::module_name_repetitions)]

pub mod blueprint;
pub mod config;
pub mod error;
pub mod executor;
pub mod remote;
pub mod rt;
pub mod sdk;
pub mod sources;

pub use executor::run_blueprint_manager;

pub use blueprint_auth;
