#!/usr/bin/env bash
#
# Start the full Tangle local stack:
#   anvil + contracts → operator (modal-inference w/ Qwen 3.5) → router → ready
#
# Usage:
#   ./scripts/start-local-stack.sh
#
# Prerequisites:
#   - cargo-tangle built: cargo build --manifest-path cli/Cargo.toml --bin cargo-tangle
#   - modal-operator built: cd ~/code/modal-inference-blueprint && cargo build --manifest-path operator/Cargo.toml
#   - router deps: cd ~/code/tangle-router && pnpm install
#   - Qwen 3.5 deployed on Modal: cd ~/code/phony && modal deploy infra/modal-gpu/obliteratus_service.py
#
# To stop: Ctrl+C (kills all background processes)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BLUEPRINT_DIR="$(dirname "$SCRIPT_DIR")"
ROUTER_DIR="${HOME}/code/tangle-router"
MODAL_BLUEPRINT_DIR="${HOME}/code/modal-inference-blueprint"
OPERATOR_PORT=19001
ROUTER_PORT=3000

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log() { echo -e "${CYAN}[stack]${NC} $*"; }
ok()  { echo -e "${GREEN}[stack]${NC} $*"; }
err() { echo -e "${RED}[stack]${NC} $*"; }
warn() { echo -e "${YELLOW}[stack]${NC} $*"; }

PIDS=()

cleanup() {
  log "Shutting down..."
  for pid in "${PIDS[@]}"; do
    kill "$pid" 2>/dev/null || true
  done
  # Kill anything on our ports
  lsof -i :${OPERATOR_PORT} -t 2>/dev/null | xargs kill -9 2>/dev/null || true
  lsof -i :${ROUTER_PORT} -t 2>/dev/null | xargs kill -9 2>/dev/null || true
  ok "Stack stopped."
  exit 0
}

trap cleanup INT TERM EXIT

# ── Preflight checks ──────────────────────────────────────────────────

log "Checking prerequisites..."

if ! command -v cargo-tangle &>/dev/null && [ ! -f "${BLUEPRINT_DIR}/target/debug/cargo-tangle" ]; then
  err "cargo-tangle not found. Build it:"
  err "  cargo build --manifest-path cli/Cargo.toml --bin cargo-tangle"
  exit 1
fi

CARGO_TANGLE="${BLUEPRINT_DIR}/target/debug/cargo-tangle"
if [ ! -f "$CARGO_TANGLE" ]; then
  CARGO_TANGLE="cargo-tangle"
fi

if [ ! -f "${MODAL_BLUEPRINT_DIR}/target/debug/modal-operator" ]; then
  err "modal-operator not found. Build it:"
  err "  cd ${MODAL_BLUEPRINT_DIR} && cargo build --manifest-path operator/Cargo.toml"
  exit 1
fi

if [ ! -d "${ROUTER_DIR}/node_modules" ]; then
  warn "Router node_modules missing. Installing..."
  (cd "$ROUTER_DIR" && pnpm install --silent)
fi

# ── Kill old processes on our ports ─────────────────────────────────

for port in $OPERATOR_PORT $ROUTER_PORT; do
  pid=$(lsof -i :${port} -t 2>/dev/null || true)
  if [ -n "$pid" ]; then
    warn "Killing old process on port $port (PID $pid)"
    kill -9 "$pid" 2>/dev/null || true
    sleep 1
  fi
done

# ── Write harness config ───────────────────────────────────────────

HARNESS_CONFIG=$(mktemp /tmp/harness-XXXXXX.toml)
cat > "$HARNESS_CONFIG" <<TOML
[chain]
anvil = true
include_anvil_logs = false

[[blueprint]]
name = "obliteratus"
path = "${MODAL_BLUEPRINT_DIR}"
binary = "${MODAL_BLUEPRINT_DIR}/target/debug/modal-operator"
port = ${OPERATOR_PORT}
health_path = "/health"
startup_timeout_secs = 30
blueprint_type = "llm"
models = [
  { id = "qwen-3.5-27b-abliterated", input_price = 0.001, output_price = 0.002 },
]

[blueprint.env]
RUST_LOG = "info"
TOML

# ── Start harness (anvil + operator) ───────────────────────────────

log "Starting harness (anvil + contracts + operator)..."
cd "$BLUEPRINT_DIR"
"$CARGO_TANGLE" tangle harness up --config "$HARNESS_CONFIG" > /tmp/harness.log 2>&1 &
HARNESS_PID=$!
PIDS+=("$HARNESS_PID")
log "Harness PID: $HARNESS_PID"

# Wait for operator to be healthy
log "Waiting for operator on :${OPERATOR_PORT}..."
for i in $(seq 1 60); do
  if curl -sS --max-time 2 http://localhost:${OPERATOR_PORT}/health > /dev/null 2>&1; then
    ok "Operator healthy after ${i}s"
    break
  fi
  if ! kill -0 "$HARNESS_PID" 2>/dev/null; then
    err "Harness died! Last 20 lines:"
    tail -20 /tmp/harness.log
    exit 1
  fi
  sleep 1
done

# Verify with a real chat
log "Testing operator with real inference..."
RESPONSE=$(curl -sS --max-time 60 -X POST http://localhost:${OPERATOR_PORT}/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"say hello briefly"}],"model":"qwen-3.5-27b-abliterated","max_tokens":20}' 2>&1)

MODEL=$(echo "$RESPONSE" | python3 -c "import sys,json; print(json.loads(sys.stdin.read())['model'])" 2>/dev/null || echo "unknown")
CONTENT=$(echo "$RESPONSE" | python3 -c "import sys,json; print(json.loads(sys.stdin.read())['choices'][0]['message']['content'][:100])" 2>/dev/null || echo "parse error")
ok "Model: $MODEL"
ok "Response: $CONTENT"

# ── Start local router ─────────────────────────────────────────────

log "Starting local Tangle Router on :${ROUTER_PORT}..."
cd "$ROUTER_DIR"
PORT=$ROUTER_PORT pnpm dev > /tmp/router.log 2>&1 &
ROUTER_PID=$!
PIDS+=("$ROUTER_PID")
log "Router PID: $ROUTER_PID"

# Wait for router to be healthy
for i in $(seq 1 30); do
  if curl -sS --max-time 2 http://localhost:${ROUTER_PORT}/api/health > /dev/null 2>&1; then
    ok "Router healthy after ${i}s"
    break
  fi
  if ! kill -0 "$ROUTER_PID" 2>/dev/null; then
    err "Router died! Last 20 lines:"
    tail -20 /tmp/router.log
    exit 1
  fi
  sleep 1
done

# ── Register operator with router ──────────────────────────────────

log "Registering operator with local router..."
REG_RESULT=$(curl -sS --max-time 10 -X POST http://localhost:${ROUTER_PORT}/api/operators \
  -H "Content-Type: application/json" \
  -d "{
    \"name\": \"obliteratus-local\",
    \"endpointUrl\": \"http://localhost:${OPERATOR_PORT}\",
    \"blueprintType\": \"llm\",
    \"description\": \"Qwen 3.5-27B abliterated on Modal A100-80GB\",
    \"models\": [
      {\"modelId\": \"qwen-3.5-27b-abliterated\", \"inputPrice\": 0.001, \"outputPrice\": 0.002}
    ]
  }" 2>&1)

if echo "$REG_RESULT" | python3 -c "import sys,json; d=json.loads(sys.stdin.read()); print(d.get('operator',{}).get('id',''))" 2>/dev/null | grep -q .; then
  ok "Operator registered with router"
else
  warn "Registration response: $REG_RESULT"
  warn "May need to register manually or check router DB"
fi

# ── Test through router ────────────────────────────────────────────

log "Testing chat through router..."
ROUTER_RESPONSE=$(curl -sS --max-time 60 -X POST http://localhost:${ROUTER_PORT}/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model":"qwen-3.5-27b-abliterated","messages":[{"role":"user","content":"count to 3"}],"max_tokens":20}' 2>&1)

echo ""
ok "Router response: $(echo "$ROUTER_RESPONSE" | head -c 200)"

# ── Ready ──────────────────────────────────────────────────────────

echo ""
echo "════════════════════════════════════════════════════════════════"
ok "LOCAL TANGLE STACK READY"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "  Operator:  http://localhost:${OPERATOR_PORT}"
echo "  Router:    http://localhost:${ROUTER_PORT}"
echo "  Model:     qwen-3.5-27b-abliterated (Qwen 3.5-27B on Modal A100)"
echo ""
echo "  Test with tcloud:"
echo "    TCLOUD_BASE_URL=http://localhost:${ROUTER_PORT} npx tcloud chat 'hello' --model qwen-3.5-27b-abliterated"
echo ""
echo "  Test with curl:"
echo "    curl -X POST http://localhost:${ROUTER_PORT}/v1/chat/completions \\"
echo "      -H 'Content-Type: application/json' \\"
echo "      -d '{\"model\":\"qwen-3.5-27b-abliterated\",\"messages\":[{\"role\":\"user\",\"content\":\"hello\"}]}'"
echo ""
echo "  Test with OpenAI SDK:"
echo "    import OpenAI from 'openai'"
echo "    const client = new OpenAI({ baseURL: 'http://localhost:${ROUTER_PORT}/v1', apiKey: 'dev' })"
echo "    const res = await client.chat.completions.create({ model: 'qwen-3.5-27b-abliterated', messages: [...] })"
echo ""
echo "  Logs:"
echo "    tail -f /tmp/harness.log    # harness + operator"
echo "    tail -f /tmp/router.log     # router"
echo ""
echo "  Press Ctrl+C to stop everything."
echo ""

# Keep alive
wait
