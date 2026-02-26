# blueprint-producers-extra

Protocol-agnostic producer implementations for Blueprint runtimes.

This crate currently includes extra producer implementations that are not tied to a specific chain protocol.

## Available producers

- `cron` feature: cron-scheduled job producer (`cron::CronJob`)

## Feature flags

- `cron`: enables cron-based scheduling producer support

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/producers-extra
- Trigger docs: https://docs.tangle.tools/developers/blueprint-runner/job-triggers
- Producer docs: https://docs.tangle.tools/developers/blueprint-runner/producers
