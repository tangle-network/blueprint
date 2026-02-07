use alloy_sol_types::{SolValue, sol};

sol! {
    struct MetricPair {
        string name;
        uint256 value;
    }
}

/// ABI-encode a list of metric key-value pairs into bytes that match
/// Solidity `abi.decode(data, (MetricPair[]))`.
pub fn encode_metric_pairs(metrics: &[(String, u64)]) -> Vec<u8> {
    let pairs: Vec<MetricPair> = metrics
        .iter()
        .map(|(name, value)| MetricPair {
            name: name.clone(),
            value: alloy_primitives::U256::from(*value),
        })
        .collect();
    pairs.abi_encode()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_sol_types::SolValue;

    #[test]
    fn test_encode_decode_metric_pairs() {
        let metrics = vec![
            ("response_time_ms".to_string(), 150u64),
            ("uptime_percent".to_string(), 99u64),
        ];

        let encoded = encode_metric_pairs(&metrics);
        assert!(!encoded.is_empty());

        let decoded = Vec::<MetricPair>::abi_decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].name, "response_time_ms");
        assert_eq!(decoded[0].value, alloy_primitives::U256::from(150u64));
        assert_eq!(decoded[1].name, "uptime_percent");
        assert_eq!(decoded[1].value, alloy_primitives::U256::from(99u64));
    }

    #[test]
    fn test_empty_metrics_encode() {
        let metrics: Vec<(String, u64)> = vec![];
        let encoded = encode_metric_pairs(&metrics);
        assert!(!encoded.is_empty()); // ABI encoding of empty array is still valid bytes

        let decoded = Vec::<MetricPair>::abi_decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 0);
    }

    #[test]
    fn test_single_metric() {
        let metrics = vec![("cpu_usage".to_string(), 42u64)];
        let encoded = encode_metric_pairs(&metrics);

        let decoded = Vec::<MetricPair>::abi_decode(&encoded).unwrap();
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0].name, "cpu_usage");
        assert_eq!(decoded[0].value, alloy_primitives::U256::from(42u64));
    }
}
