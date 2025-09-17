#!/bin/bash
# Setup script for remote-providers test environment

set -e

echo "Setting up test environment for blueprint-remote-providers..."

# Check OS
OS="$(uname -s)"
ARCH="$(uname -m)"

# Install kind for Kubernetes testing
install_kind() {
    if command -v kind &> /dev/null; then
        echo "✓ kind already installed"
    else
        echo "Installing kind..."
        if [[ "$OS" == "Darwin" ]]; then
            brew install kind
        elif [[ "$OS" == "Linux" ]]; then
            curl -Lo ./kind "https://kind.sigs.k8s.io/dl/v0.20.0/kind-linux-${ARCH}"
            chmod +x ./kind
            sudo mv ./kind /usr/local/bin/kind
        fi
    fi
}

# Install kubectl
install_kubectl() {
    if command -v kubectl &> /dev/null; then
        echo "✓ kubectl already installed"
    else
        echo "Installing kubectl..."
        if [[ "$OS" == "Darwin" ]]; then
            brew install kubectl
        elif [[ "$OS" == "Linux" ]]; then
            curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/${ARCH}/kubectl"
            chmod +x kubectl
            sudo mv kubectl /usr/local/bin/
        fi
    fi
}

# Install Docker (required for Kind)
check_docker() {
    if command -v docker &> /dev/null; then
        echo "✓ Docker already installed"
    else
        echo "ERROR: Docker is required but not installed"
        echo "Please install Docker Desktop from https://www.docker.com/products/docker-desktop"
        exit 1
    fi
}

# Install sshpass for SSH testing
install_sshpass() {
    if command -v sshpass &> /dev/null; then
        echo "✓ sshpass already installed"
    else
        echo "Installing sshpass..."
        if [[ "$OS" == "Darwin" ]]; then
            brew install hudochenkov/sshpass/sshpass
        elif [[ "$OS" == "Linux" ]]; then
            sudo apt-get update && sudo apt-get install -y sshpass
        fi
    fi
}

# Create test Kind cluster
create_test_cluster() {
    if kind get clusters | grep -q "blueprint-test"; then
        echo "✓ Kind cluster 'blueprint-test' already exists"
    else
        echo "Creating Kind cluster 'blueprint-test'..."
        cat <<EOF | kind create cluster --name blueprint-test --config=-
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraPortMappings:
  - containerPort: 30000
    hostPort: 30000
    protocol: TCP
EOF
    fi
}

# Main installation
echo "1. Checking Docker..."
check_docker

echo "2. Installing kind..."
install_kind

echo "3. Installing kubectl..."
install_kubectl

echo "4. Installing sshpass..."
install_sshpass

echo "5. Creating test cluster..."
create_test_cluster

echo ""
echo "✅ Test environment setup complete!"
echo ""
echo "To run Kubernetes tests:"
echo "  cargo test -p blueprint-remote-providers --test kubernetes_deployment -- --ignored"
echo ""
echo "To run SSH tests:"
echo "  cargo test -p blueprint-remote-providers --test ssh_deployment"
echo ""
echo "To cleanup test cluster:"
echo "  kind delete cluster --name blueprint-test"