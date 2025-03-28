//! Test suite for the Tangle Cloud Pricing Engine
//!
//! This module contains tests for various components of the pricing engine,
//! organized according to user flows and system functionality.
//!
//! # Test Structure
//!
//! The tests are organized into several modules:
//!
//! - `service`: Tests for service lifecycle (startup, shutdown, configuration)
//! - `pricing_models`: Tests for price calculation logic
//! - `rfq`: Tests for the Request for Quote system
//! - `integration`: End-to-end tests that verify complete workflows
//!
//! # What's Missing
//!
//! The following components still need more comprehensive tests:
//!
//! 1. **Blueprint-specific pricing**: We need to implement and test blueprint-specific
//!    pricing logic that can evaluate resource requirements against specific blueprint
//!    capabilities and constraints.
//!
//! 2. **Blockchain integration**: Tests for the blockchain event handling need to be
//!    implemented once the blockchain integration is more complete.
//!
//! 3. **RPC server**: The RPC server functionality should be tested, particularly the
//!    JSON-RPC endpoints for price discovery and quote management.
//!
//! 4. **Mock tests**: More comprehensive mock tests that simulate the network layer
//!    would help verify the full request/response cycles.
//!
//! # Running the Tests
//!
//! Run all tests with:
//! ```bash
//! cargo test -p pricing-engine
//! ```
//!
//! Or run a specific test module:
//! ```bash
//! cargo test -p pricing-engine --test service
//! ```

pub mod integration;
pub mod pricing_models;
pub mod rfq;
pub mod rpc_workflow;
pub mod service;
pub mod utils;
