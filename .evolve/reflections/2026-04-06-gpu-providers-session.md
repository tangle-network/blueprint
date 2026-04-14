# Reflect: GPU Providers + Inference Core Session
Date: 2026-04-06

## Run Grade: 8/10

| Dimension | Score | Evidence |
|---|---|---|
| **Goal achievement** | 9/10 | All stated goals met: 17 providers, inference core extraction, 7 blueprint migrations, vllm→llm rename, audit fixes, PR merged. Only gap: 5 blueprints at 7-22% LOC reduction vs 50% target. |
| **Code quality** | 8/10 | 33 core tests, 250 provider tests, clippy clean, feature-gated. Audit found and fixed CRITICAL nonce TOCTOU, credential exposure, integer overflow. Remaining: no live API integration tests. |
| **Efficiency** | 7/10 | ~40 background agents dispatched. Many hit rate limits and delivered partial work. Shared file edits were lost 2-3 times during the session (cargo fmt overwrites, stash accidents). Significant rework re-applying wiring. |
| **Self-correction** | 9/10 | Caught and corrected: "backends should be separate blueprints" → "backends are operator config within service-type blueprints". Caught RFP-GPU-SUPPORT.md was outdated. Caught that manager GPU flow was scaffolded but not wired. Critical audit findings fixed same-session. |
| **Learning** | 8/10 | Produced 3 pursuit specs, MIGRATION.md, architecture docs. But .evolve state was inconsistent — current.json was rewritten multiple times without stable baselines. |
| **Overall** | 8/10 | Massive scope delivered. PR merged. Production-ready path established. Efficiency drag from file-loss incidents and rate-limited agents. |

## Session Flow Analysis

### Flow 1: Audit → Discover gap → Fix
```
TRIGGER: User asks "check this, is it complete?"
STEPS: Read existing code → find gap → propose fix → user refines intent → implement
OUTCOME: RFP rewritten, PLAN updated, GPU flow documented
Frequency: 3x (RFP, PLAN, GPU-SUPPORT.md)
Automation: None needed — this is discovery work
```

### Flow 2: Parallel agent dispatch → compile → fix stragglers
```
TRIGGER: Large implementation scope ("do all of it")
STEPS: Write shared skeleton → dispatch 5-7 background agents → wait → fix compile errors from agent output → verify
OUTCOME: Code lands but with inconsistencies (some agents use retry, others don't; some handle 404, others don't)
Frequency: 5x (Gen 1 adapters, Gen 2 decentralized, Gen 2 hardening, Gen 3 migrations, audit fixes)
Automation potential: HIGH — a "dispatch + verify + fix" meta-skill would prevent the inconsistency problem
```

### Flow 3: Shared file collision
```
TRIGGER: Multiple agents or cargo fmt modifying the same tracked file
STEPS: Agent A writes to config.rs → cargo fmt runs → Agent B writes to config.rs → one version wins, other lost
OUTCOME: Wiring had to be re-applied 3 times (providers/mod.rs, pricing-engine enum, config.rs)
Frequency: 3x
LESSON: Pre-populate ALL shared file edits before dispatching agents. Agents should ONLY create new files in their own directories.
```

### Flow 4: Architecture pivot
```
TRIGGER: User corrects a wrong assumption
STEPS: I propose X → user says "that's not the intent, it's Y" → I adjust
OUTCOME: Better architecture (blueprints = services, backends = operator choice, manager = infrastructure)
Frequency: 3x (backend-as-blueprint → backend-as-config, manager should handle provisioning, Vllm types are correctly named)
LESSON: Ask clarifying questions before proposing architecture changes. The user's existing design was more thoughtful than I initially assumed.
```

## Project Health

### blueprint SDK (~/code/blueprint)
- **Trajectory**: Improving — 17 providers merged, hardened, documented
- **Test coverage**: 250 provider tests (JSON parsing + retry helpers). Zero e2e API tests. ~60% meaningful coverage.
- **Architecture**: Clean — adapter pattern scales well. Config sprawl improved with helpers. Enum exhaustiveness is the right tradeoff.
- **Next action**: Live integration test with one real provider (RunPod or Lambda Labs — cheapest to validate)

### tangle-inference-core (~/code/tangle-inference-core)
- **Trajectory**: Healthy — 33 tests, feature-gated, all 7 consumers adopted
- **Test coverage**: Good for billing/cost math. Weak for server helpers (validate_spend_auth tests added but no HTTP-level integration test).
- **Architecture**: Clean — AppStateBuilder + type-erased backend is the right call. Feature gates prevent unnecessary dep weight.
- **Next action**: CI pipeline (GitHub Actions) — currently no automated testing on push

### Inference blueprints (7 repos)
- **Trajectory**: Converging — all on tangle-inference-core, all compile clean
- **Test coverage**: Varies wildly (llm: 5+26 tests, distributed: 11, modal: 0 lib tests)
- **Architecture**: 2 fully migrated (llm, voice at 50%), 5 partially migrated (7-22%)
- **Next action**: Deep server.rs rewrites for the 5 partial migrations

## Key Learnings

### 1. Shared file edits are the #1 source of rework
Every time multiple agents or tools modify the same file, one version wins. The session lost ~2 hours re-applying CloudProvider enum additions, config structs, and factory registrations. **Rule: do all shared-file edits yourself, dispatch agents only for new-file creation.**

### 2. Agent consistency requires explicit reference implementations
The 5 "correct" adapters (akash, io_net, prime_intellect, render, bittensor_lium) all used retry_with_backoff because their prompts were written later and pointed to the earlier adapters as references. The 6 "incorrect" adapters (lambda_labs, runpod, vast_ai, paperspace, fluidstack, tensordock) were written first without a reference and missed retry. **Rule: always include a reference implementation in agent prompts.**

### 3. The user's design intent is load-bearing
Three times I proposed architectural changes (backends-as-blueprints, manager-level backend selection, vllm rename being "not worth it") that the user corrected. Each correction revealed that the existing design had more thought behind it than I assumed. **Rule: audit before proposing. Ask why before suggesting what.**

### 4. Audit-driven development produces the highest-quality improvements
The /critical-audit skill found the nonce TOCTOU race (CRITICAL), credential exposure (HIGH), integer overflow (MEDIUM), and settle_billing fire-and-forget (HIGH) — none of which were visible during normal development. The audit→fix cycle produced the most impactful quality improvements of the entire session. **Rule: audit after every major build phase, not just at the end.**

### 5. Git dep vs path dep matters for production
An agent changed a path dep to a git dep mid-session, which was actually the right call — local paths break for anyone else cloning the repo. But it surfaced that the remote needed to be pushed first. **Rule: always use git deps for cross-repo references. Push before depending.**

## Product Signals

### 1. GPU Cloud Marketplace Abstraction Layer
**Who would pay**: Operators who want to serve GPU workloads without vendor lock-in.
**Evidence**: 17 providers integrated, all following the same trait. The value is the abstraction, not any single provider.
**Signal strength**: Strong — this is the core value proposition of the Blueprint Manager's remote-providers system.

### 2. Shared Inference Operator Infrastructure
**Who would pay**: Blueprint developers building inference services.
**Evidence**: 7 blueprints adopted tangle-inference-core, each deleting 500-1800 LOC of duplicated billing/metrics/auth code.
**Signal strength**: Strong — the duplication was real and growing.

### 3. Settlement Recovery Queue
**Who would pay**: Operators who can't afford to serve free inference when on-chain settlement fails.
**Evidence**: settle_billing was silently dropping errors. The recovery queue ensures failed settlements are retried.
**Signal strength**: Medium — important for production but not a standalone product.

## Action Items (ordered by impact)

1. **CI pipeline for tangle-inference-core** — no automated testing on push. One regression breaks all 7 blueprints.
2. **Live integration test** — prove one provider's REST payloads actually work (RunPod: cheapest, simplest API).
3. **Deep server.rs rewrites** for embedding/modal/image-gen/video-gen/distributed — use core's `billing_gate` and `from_config` to reach 50% reduction.
4. **Memory: save the shared-file-edit rule** — this session's biggest efficiency loss. Future sessions should avoid it.
5. **Ops board tasks** for each remaining item so nothing falls through cracks.
