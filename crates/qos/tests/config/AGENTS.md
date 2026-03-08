# config

## Purpose
Contains configuration files used by QoS integration tests, specifically Prometheus scrape configuration for test environments.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `prometheus.yml` - Prometheus configuration with 10-second scrape/evaluation intervals. Defines two scrape jobs: `prometheus` targeting itself at `localhost:9090`, and `blueprint_test_app_metrics` scraping the `/metrics` endpoint from the test application at `host.docker.internal:9091` (for Docker-to-host connectivity).

## Key APIs (no snippets)
- (not applicable -- configuration file only)

## Relationships
- Used by QoS integration tests that spin up a Docker-based Prometheus server via `crate::servers::prometheus`.
- The `host.docker.internal:9091` target assumes the test application exposes an embedded Prometheus metrics endpoint on port 9091.
