# Pricing Engine Testing Plan

This document outlines the testing strategy for the blueprint-pricing-engine crate, along with guidelines for adding and maintaining tests.

## Testing Philosophy

The testing approach follows these key principles:

1. **Test Production Flows**: Tests should use the same code paths as production code. This means avoiding test-specific methods and focusing on testing the actual behavior users will experience.

2. **Real Implementation Testing**: Integration tests should use actual implementations rather than mocks where possible. This ensures that our tests validate behavior in conditions similar to real usage.

3. **Types and Generics**: Tests should respect the generic patterns used in production code, using appropriate type parameters rather than concrete types.

## Test Categories

### Unit Tests

Unit tests focus on testing individual components in isolation. These are particularly useful for testing complex algorithms or business logic.

Key unit test files:

-   `src/tests/rfq.rs`: Tests for RFQ message handling and quote generation
-   `src/tests/service.rs`: Tests for the Service lifecycle management
-   `src/tests/models.rs`: Tests for pricing model logic (pricing calculations, etc.)

### Integration Tests

Integration tests validate the interaction between multiple components. We have several types of integration tests:

1. **End-to-End Tests**: Full service lifecycle with multiple components

    - `src/tests/integration.rs`: Contains tests that verify the full flow from service initialization to RFQ handling

2. **Multi-Node Tests**: Tests involving multiple nodes in a real network
    - `test_multi_node_rfq_flow_real_network`: Tests the full RFQ flow with multiple operator nodes using actual networking

## Test Utilities

We provide several utility functions in `src/tests/utils.rs` to make writing tests easier:

-   `create_test_key_pair<K: KeyType>()`: Creates key pairs using production key generation mechanisms
-   `create_test_pricing_models()`: Creates standard pricing models for testing
-   `create_test_service_config<K: KeyType>()`: Creates service configuration
-   `create_test_rfq_processor<K: KeyType>()`: Creates an RFQ processor with test configuration

## Running Tests

To run all tests in the pricing engine crate:

```bash
cargo test -p blueprint-pricing-engine
```

To run a specific test:

```bash
cargo test -p blueprint-pricing-engine -- test_name
```

For tests that use real networking, it's recommended to run them with the `--nocapture` flag to see output:

```bash
cargo test -p blueprint-pricing-engine -- test_multi_node_rfq_flow_real_network --nocapture
```

## Adding New Tests

When adding new tests, follow these guidelines:

1. **Use Production Flows**: Test the same code paths used in production. Avoid adding test-specific methods to production code.

2. **Handle Generic Types**: Use generic type parameters correctly, as defined by the production code. For example:

    ```rust
    let processor = create_test_rfq_processor::<SpSr25519>(&signing_key, models);
    ```

3. **Test Real Networking**: For integration tests, prefer testing with real networking rather than simulated channels where possible.

4. **Make Tests Deterministic**: Ensure tests don't depend on timing or external services that might make them flaky.

5. **Use Mocks Carefully**: When mocks are necessary, ensure they accurately represent the behavior of the components they replace.

## Common Patterns

### Testing RFQ Flows

When testing RFQ message processing, follow this pattern:

```rust
// 1. Create key pairs and processor
let signing_key = create_test_key_pair::<SpSr25519>();
let models = create_test_pricing_models();
let processor = create_test_rfq_processor::<SpSr25519>(&signing_key, models);

// 2. Create a requester key and request
let requester_key = create_test_key_pair::<SpSr25519>();
let requester_id = SpSr25519::public_from_secret(&requester_key).to_bytes();
let request = create_test_quote_request(&requester_id);

// 3. Create an RFQ message (as would happen in production)
let timestamp = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis() as u64;

let request_message = RfqMessage {
    version: 1,
    timestamp,
    message_type: RfqMessageType::QuoteRequest(request.clone()),
};

// 4. Process the message through the standard flow
let result = processor.process_incoming_message(request_message, Some(PeerId::random())).await;
assert!(result.is_ok());
```

### Testing Network Communication

For testing real network communication:

1. Use `blueprint_networking::test_utils::create_whitelisted_nodes` to create test nodes
2. Start nodes and wait for handshakes with `wait_for_all_handshakes`
3. Set up Services with RFQ processors that use the network handles
4. Send messages through one node and verify they propagate to other nodes

## Current Test Status and TODOs

As of the current implementation:

-   ✅ Basic unit tests for RFQ handling
-   ✅ Service lifecycle tests
-   ✅ Integration tests with simulated network
-   ✅ Real network multi-node test structure
-   ❌ Price calculation tests (need implementation of `calculate_price` method)
-   ❌ Comprehensive error handling tests
-   ❌ Performance tests for large numbers of quotes

## Conclusion

Testing is an integral part of the pricing engine development process. By following the patterns and practices in this document, we ensure high-quality, reliable code that behaves correctly in production scenarios.
