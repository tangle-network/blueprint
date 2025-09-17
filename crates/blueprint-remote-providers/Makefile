.PHONY: help setup test test-unit test-k8s test-ssh test-all clean

help:
	@echo "Remote Providers Test Commands:"
	@echo "  make setup       - Install test dependencies (kind, kubectl, etc)"
	@echo "  make test        - Run unit tests only"
	@echo "  make test-unit   - Run unit tests"
	@echo "  make test-k8s    - Run Kubernetes tests (requires kind)"
	@echo "  make test-ssh    - Run SSH deployment tests"
	@echo "  make test-all    - Run all tests including integration"
	@echo "  make clean       - Delete test clusters and cleanup"

setup:
	@./scripts/setup-test-env.sh

test: test-unit

test-unit:
	@echo "Running unit tests..."
	@cargo test --lib

test-k8s: 
	@echo "Running Kubernetes tests (requires kind)..."
	@kind get clusters | grep -q blueprint-test || (echo "Error: Run 'make setup' first" && exit 1)
	@cargo test --test kubernetes_real -- --ignored --nocapture

test-ssh:
	@echo "Running SSH deployment tests..."
	@cargo test --test ssh_deployment

test-integration:
	@echo "Running mock integration tests..."
	@cargo test --test deployment_integration
	@cargo test --test critical_flows

test-pricing:
	@echo "Testing live pricing APIs..."
	@cargo test test_pricing_apis_available -- --nocapture

test-all: test-unit test-integration test-pricing
	@echo "Running all tests (including K8s if available)..."
	@if command -v kind >/dev/null 2>&1; then \
		make test-k8s; \
	else \
		echo "Skipping K8s tests (kind not installed)"; \
	fi

clean:
	@echo "Cleaning up test resources..."
	@kind delete cluster --name blueprint-test 2>/dev/null || true
	@docker rm -f $(shell docker ps -aq --filter name=blueprint-) 2>/dev/null || true

verify-tools:
	@echo "Checking required tools..."
	@command -v docker >/dev/null 2>&1 || (echo "❌ Docker not installed" && exit 1)
	@command -v kind >/dev/null 2>&1 || echo "⚠️  kind not installed (run: make setup)"
	@command -v kubectl >/dev/null 2>&1 || echo "⚠️  kubectl not installed (run: make setup)"
	@echo "✅ Basic tools verified"