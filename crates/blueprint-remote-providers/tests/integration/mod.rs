//! Integration tests for Blueprint Remote Providers
//!
//! These tests verify end-to-end functionality across modules

// pub mod auth_integration; // DISABLED: missing dependencies
pub mod blueprint_ssh_deployment_tests;
// pub mod chaos_engineering_tests; // DISABLED: compiler cycle error
// pub mod core_functionality; // DISABLED: missing dependencies
// pub mod critical_flows; // DISABLED: missing dependencies
// pub mod manager_bridge; // DISABLED: missing dependencies
// pub mod observability; // DISABLED: missing dependencies
// pub mod property_tests; // DISABLED: missing dependencies
// pub mod qos_integration; // DISABLED: missing dependencies
pub mod real_blueprint_tests;
pub mod ssh_container_tests;
pub mod ssh_deployment_integration;
