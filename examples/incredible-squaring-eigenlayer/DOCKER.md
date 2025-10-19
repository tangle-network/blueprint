# Docker & Kubernetes Testing Guide

This guide explains how to build, test, and run the incredible-squaring-eigenlayer blueprint in containers using Docker and Kubernetes (via Kind).

## Quick Start

```bash
# Build Docker image
./build-docker.sh

# Build and load into Kind cluster
./build-docker.sh --load-kind my-cluster

# Run container locally
docker run --rm -it incredible-squaring-blueprint-eigenlayer:latest --help
```

## Prerequisites

### For Local Docker Testing
- Docker Desktop (or Docker Engine)
- macOS/Linux/Windows with Docker support

### For Kubernetes Testing
- **Kind** (Kubernetes in Docker): `brew install kind` (macOS) or see [Kind installation](https://kind.sigs.k8s.io/docs/user/quick-start/#installation)
- Docker Desktop running
- kubectl (optional, for debugging): `brew install kubectl`

## Docker Image

### Architecture Note

The binary MUST be built for Linux inside Docker, not copied from macOS:
- ❌ **macOS binary**: Built for `aarch64-apple-darwin` (won't run in Linux containers)
- ✅ **Docker binary**: Built for `aarch64-unknown-linux-gnu` (correct for containers)

### Multi-Stage Build with Layer Caching

The Dockerfile uses an optimized multi-stage build:

**Build Performance:**
- **First build**: ~5-10 minutes (compiles all dependencies + code)
- **Subsequent builds**: Docker may cache some layers, but Rust compilation takes most of the time

**Build stages:**
1. **Builder Stage**: Compiles the Rust binary from source
   - Base: `rust:1.86-bookworm` (Linux Rust toolchain)
   - Installs build dependencies (protobuf, SSL, pkg-config)
   - Compiles the binary in release mode for Linux

2. **Runtime Stage**: Minimal Debian image with just the binary
   - Base: `debian:bookworm-slim`
   - Non-root user (`blueprint:1000`)
   - Mounted volumes for keystore and data
   - Health check configured
   - Final image size: ~150MB

### Image Details

- **Name**: `incredible-squaring-blueprint-eigenlayer`
- **Tag**: `latest` (override with `IMAGE_TAG=v1.0.0`)
- **Size**: ~150MB (runtime only, no build tools)
- **User**: Non-root user `blueprint` (UID 1000)
- **Entrypoint**: `/usr/local/bin/blueprint`
- **Default CMD**: `run`

### Volume Mounts

The container expects these volumes:

- `/mnt/keystore` - Keystore directory (read-only)
- `/mnt/data` - Data directory (read-write)

### Environment Variables

Standard blueprint environment variables (see main docs):

```bash
HTTP_RPC_URL=http://...
WS_RPC_URL=ws://...
KEYSTORE_URI=/mnt/keystore
DATA_DIR=/mnt/data
BLUEPRINT_ID=...
SERVICE_ID=...
PROTOCOL=eigenlayer
CHAIN=...
BOOTNODES=...
```

## Building the Image

### From Source (Default)

```bash
# Build from workspace root
./build-docker.sh

# The script will:
# 1. Build the entire workspace context
# 2. Compile the blueprint in release mode
# 3. Create minimal runtime image
```

### Custom Tag

```bash
export IMAGE_TAG=v1.0.0
./build-docker.sh
```

## Running Locally with Docker

### Basic Run

```bash
docker run --rm -it \
  incredible-squaring-blueprint-eigenlayer:latest \
  --help
```

### With Volumes and Environment

```bash
docker run --rm -it \
  -v ./keystore:/mnt/keystore:ro \
  -v ./data:/mnt/data \
  -e HTTP_RPC_URL=http://localhost:8545 \
  -e WS_RPC_URL=ws://localhost:8545 \
  -e KEYSTORE_URI=/mnt/keystore \
  incredible-squaring-blueprint-eigenlayer:latest \
  run
```

## Kubernetes Testing with Kind

### 1. Create Kind Cluster

```bash
kind create cluster --name blueprint-test
```

### 2. Build and Load Image

```bash
# Build and load in one command
./build-docker.sh --load-kind blueprint-test
```

### 3. Verify Image is Available

```bash
# List images in Kind cluster
docker exec -it blueprint-test-control-plane crictl images | grep incredible
```

### 4. Deploy to Kubernetes

Create a pod manifest (`pod.yaml`):

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: incredible-squaring-eigenlayer
  namespace: blueprint-manager
spec:
  containers:
  - name: blueprint
    image: incredible-squaring-blueprint-eigenlayer:latest
    imagePullPolicy: Never  # Use local image from Kind
    env:
    - name: HTTP_RPC_URL
      value: "http://host.docker.internal:8545"
    - name: WS_RPC_URL
      value: "ws://host.docker.internal:8545"
    - name: KEYSTORE_URI
      value: "/mnt/keystore"
    - name: DATA_DIR
      value: "/mnt/data"
    - name: BLUEPRINT_ID
      value: "1"
    - name: SERVICE_ID
      value: "0"
    - name: PROTOCOL
      value: "eigenlayer"
    volumeMounts:
    - name: keystore
      mountPath: /mnt/keystore
      readOnly: true
    - name: data
      mountPath: /mnt/data
  volumes:
  - name: keystore
    hostPath:
      path: /path/to/keystore
      type: Directory
  - name: data
    emptyDir: {}
```

Deploy:

```bash
kubectl create namespace blueprint-manager
kubectl apply -f pod.yaml
kubectl logs -f incredible-squaring-eigenlayer -n blueprint-manager
```

### 5. Cleanup

```bash
# Delete pod
kubectl delete pod incredible-squaring-eigenlayer -n blueprint-manager

# Delete cluster
kind delete cluster --name blueprint-test
```

## Running the Container Lifecycle Test

The test suite includes a comprehensive end-to-end test that:
- Creates a Kind cluster
- Builds the Docker image
- Loads it into Kind
- Runs the container
- Verifies it works
- Cleans up automatically

### Run the Test

```bash
# Must have containers feature enabled
cargo test --test runtime_target_test \
  --features containers \
  -- test_container_runtime_full_lifecycle_with_kind \
  --ignored --nocapture
```

The test will:
1. ✅ Check Kind is installed
2. ✅ Check Docker is running
3. 🔧 Create temporary Kind cluster
4. 🐳 Build Docker image from source
5. 📦 Load image into Kind
6. ⚙️  Register AVS with container runtime
7. 🧹 Cleanup (even on failure)

## Troubleshooting

### Image Not Found in Kind

```bash
# Rebuild and reload
./build-docker.sh --load-kind <cluster-name>

# Verify
docker exec -it <cluster-name>-control-plane crictl images
```

### Permission Denied

The container runs as non-root user `blueprint` (UID 1000). Ensure volumes have correct permissions:

```bash
sudo chown -R 1000:1000 /path/to/keystore
```

### Build Fails

```bash
# Clean and rebuild
docker system prune -f
./build-docker.sh
```

### Pod Stays in Pending

Check events:

```bash
kubectl describe pod <pod-name> -n blueprint-manager
```

Common issues:
- Image pull policy (should be `Never` for local images)
- Volume mount paths don't exist
- Resource limits too high

## Advanced Usage

### Using Kata Containers (VM Isolation)

If your cluster has Kata Containers installed:

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: incredible-squaring-eigenlayer
spec:
  runtimeClassName: kata  # Use Kata runtime for VM isolation
  containers:
  - name: blueprint
    # ... rest of spec
```

### Multi-Node Cluster

```bash
kind create cluster --name blueprint-test --config - <<EOF
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
- role: worker
- role: worker
EOF
```

### Custom Registry

Push to a registry accessible from Kind:

```bash
# Tag for registry
docker tag incredible-squaring-blueprint-eigenlayer:latest \
  localhost:5000/incredible-squaring-blueprint-eigenlayer:latest

# Push
docker push localhost:5000/incredible-squaring-blueprint-eigenlayer:latest

# Update imagePullPolicy to Always in pod.yaml
```

## Testing Checklist

- [ ] Docker image builds successfully
- [ ] Image loads into Kind cluster
- [ ] Pod starts and stays running
- [ ] Logs show no errors
- [ ] Environment variables are passed correctly
- [ ] Volumes are mounted correctly
- [ ] Health check passes
- [ ] Blueprint connects to RPC endpoints
- [ ] Cleanup removes all resources

## See Also

- [Kind Documentation](https://kind.sigs.k8s.io/)
- [Kata Containers](https://katacontainers.io/)
- [Kubernetes Pods](https://kubernetes.io/docs/concepts/workloads/pods/)
- [Blueprint Manager Docs](../../crates/manager/README.md)
