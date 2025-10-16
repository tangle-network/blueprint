#![cfg_attr(not(feature = "std"), no_std)]
pub mod client;
pub mod error;
pub mod services;

#[cfg(not(any(feature = "std", feature = "web")))]
compile_error!("`std` or `web` feature required");

// NOTE: Actual client tests are in blueprint-tangle-testing-utils
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
