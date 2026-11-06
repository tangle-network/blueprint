# Check remote provider behavior

Run the public GPU requests against local provider HTTP peers:

```bash
cargo test -p blueprint-remote-providers --no-default-features --features std \
  --test native_gpu_http -- --test-threads=1
cargo clippy -p blueprint-remote-providers --no-default-features --features std \
  --lib --test native_gpu_http --no-deps
```

The GPU cases exercise native request fields, inventory, allocation, status,
listing and ownership during retirement. HTTP peers replace provider processes.
Recorded responses retain their capture time; synthetic responses are labelled
in the test. These checks create no billable cloud resources.

Read each target and the [manifest](../Cargo.toml) for its external prerequisites.
Kubernetes and SSH targets may require a cluster, Docker or a reachable host.
A credential skip or an expected connection failure does not prove deployment.
Live allocation follows the [rental design](../../../docs/gpu-marketplace-strategy.md)
with an authorized budget and provider-confirmed retirement receipt.
