//! TEE middleware for the Blueprint job pipeline.
//!
//! Provides [`TeeLayer`] for attaching attestation metadata to job results
//! and [`TeeContext`] as an extractor for job handlers.

pub mod tee_context;
pub mod tee_layer;

pub use tee_context::TeeContext;
pub use tee_layer::TeeLayer;
