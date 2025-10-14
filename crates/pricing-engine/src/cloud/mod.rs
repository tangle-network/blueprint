//! Cloud provider pricing APIs
//!
//! This module provides real pricing data from cloud providers for:
//! - FaaS (Function-as-a-Service) platforms like AWS Lambda, GCP Cloud Functions, Azure Functions
//! - VM (Virtual Machine) instance pricing from AWS, GCP, Azure, DigitalOcean, Vultr, and more
//!
//! All pricing is fetched from real provider APIs - NO HARDCODED VALUES.

pub mod faas;
pub mod vm;

pub use faas::{FaasPricing, FaasPricingFetcher};
pub use vm::{InstanceInfo, PricingFetcher};
