# blueprint-qos

Quality-of-Service instrumentation and health surfaces for Blueprint services.

`blueprint-qos` provides heartbeat, metrics, and logging integrations for operator observability. It can run with managed local observability components or point to externally managed stacks.

## What it provides

- Heartbeat service and consumers
- Metrics and logging integrations
- QoS service builder and config helpers
- Optional managed Grafana/Loki/Prometheus server configs

## Example entry point

```rust,ignore
use blueprint_qos::default_qos_config;

let qos_config = default_qos_config();
```

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/qos
- QoS docs: https://docs.tangle.tools/developers/blueprint-qos
- Operator QoS docs: https://docs.tangle.tools/operators/quality-of-service
