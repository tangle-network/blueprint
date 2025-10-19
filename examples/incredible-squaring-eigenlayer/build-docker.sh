#!/bin/bash
# Build and optionally load Docker image for incredible-squaring-eigenlayer blueprint
#
# Usage:
#   ./build-docker.sh                    # Build only
#   ./build-docker.sh --load-kind        # Build and load into Kind cluster
#   ./build-docker.sh --load-kind <cluster-name>  # Load into specific cluster

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

IMAGE_NAME="incredible-squaring-blueprint-eigenlayer"
IMAGE_TAG="${IMAGE_TAG:-latest}"
IMAGE_FULL="${IMAGE_NAME}:${IMAGE_TAG}"

# Check if image already exists
if docker images | grep -q "${IMAGE_NAME}.*${IMAGE_TAG}"; then
    echo "✅ Docker image $IMAGE_FULL already exists, skipping build"
else
    echo "🐳 Building Docker image: $IMAGE_FULL"
    echo "   Workspace: $WORKSPACE_ROOT"

    # Build from workspace root to include all dependencies
    cd "$WORKSPACE_ROOT"

    docker build \
        -f examples/incredible-squaring-eigenlayer/Dockerfile \
        -t "$IMAGE_FULL" \
        .

    echo "✅ Built image: $IMAGE_FULL"
fi

# Load into Kind if requested
if [[ "$1" == "--load-kind" ]]; then
    KIND_CLUSTER="${2:-kind}"

    echo "📦 Loading image into Kind cluster: $KIND_CLUSTER"

    # Check if Kind cluster exists
    if ! kind get clusters | grep -q "^${KIND_CLUSTER}$"; then
        echo "❌ Kind cluster '${KIND_CLUSTER}' not found"
        echo "   Available clusters:"
        kind get clusters
        exit 1
    fi

    # Load image into Kind
    kind load docker-image "$IMAGE_FULL" --name "$KIND_CLUSTER"

    echo "✅ Loaded $IMAGE_FULL into Kind cluster: $KIND_CLUSTER"
fi

echo ""
echo "🎉 Done! Image available as: $IMAGE_FULL"
echo ""
echo "To run locally:"
echo "  docker run --rm -it $IMAGE_FULL --help"
echo ""
echo "To load into Kind cluster:"
echo "  ./build-docker.sh --load-kind <cluster-name>"
