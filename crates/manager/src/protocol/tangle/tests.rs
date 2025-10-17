/// Tests for Tangle protocol handler to ensure backwards compatibility

#[cfg(test)]
mod tests {
    use super::super::event_handler::TangleEventHandler;
    use crate::protocol::types::{EigenlayerProtocolEvent, ProtocolEvent};

    /// Test that handler can be created and is Send + Sync
    #[test]
    fn test_handler_creation() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<TangleEventHandler>();
        assert_sync::<TangleEventHandler>();

        let _handler = TangleEventHandler::new();
    }

    /// Test that protocol events correctly identify their type
    #[test]
    fn test_protocol_event_type_checking() {
        // Create a mock EigenLayer event
        let eigenlayer_event = ProtocolEvent::Eigenlayer(EigenlayerProtocolEvent {
            block_number: 100,
            block_hash: vec![0u8; 32],
            logs: vec![],
        });

        // Verify type checking works correctly
        assert!(eigenlayer_event.as_tangle().is_none());
        assert!(eigenlayer_event.as_eigenlayer().is_some());
        assert_eq!(eigenlayer_event.block_number(), 100);
    }

    /// Test that protocol event block numbers are preserved
    #[test]
    fn test_protocol_event_block_number() {
        let eigenlayer_event = ProtocolEvent::Eigenlayer(EigenlayerProtocolEvent {
            block_number: 12345,
            block_hash: vec![0u8; 32],
            logs: vec![],
        });

        assert_eq!(eigenlayer_event.block_number(), 12345);
    }
}
