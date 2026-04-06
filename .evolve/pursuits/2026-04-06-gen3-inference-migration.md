# Pursuit: Gen 3 — All Inference Blueprints Migrate to tangle-inference-core + vllm Rename

Generation: 3
Date: 2026-04-06
Status: designing

## Thesis

Gen 2 proved the shared-crate pattern by migrating vllm-inference-blueprint with a 50% LOC reduction. Gen 3 applies that same pattern to the other 6 inference blueprints, renames `vllm-inference-blueprint` to `llm-inference-blueprint` to reflect it's a service-type blueprint (vLLM is a backend choice, not the service), and publishes a migration path document so future inference blueprints can follow a repeatable recipe.

## System Audit

### What exists and works (Gen 2 baseline)
- `tangle-inference-core` v2: `AppStateBuilder`, `Arc<dyn Any>` backend injection, 6 concrete `CostModel` impls, integration test, example, adoption README
- `vllm-inference-blueprint` migrated (Gen 2 pilot): 3 files deleted, 1,821 LOC removed, 5 lib tests + 26 server tests pass
- 19 cloud providers (12 traditional + 7 GPU/decentralized)

### The 6 remaining inference blueprints (to migrate in Gen 3)

| Blueprint | LOC | Has billing.rs? | Has metrics.rs? | Has health.rs? | Cost model fit | Unique code |
|---|---|---|---|---|---|---|
| voice-inference-blueprint | 2,553 | Yes (428) | Yes (138) | Yes | `PerCharCostModel` | voice_engine.rs (vLLM-Omni subprocess) |
| embedding-inference-blueprint | 2,169 | **No** | **No** | Yes | `PerTokenCostModel` (per-1K-token) | embedding.rs (TEI/OpenAI HTTP proxy) |
| modal-inference-blueprint | 3,011 | Yes (428+) | Yes (200+) | **No** (uses proxy) | `TaskTypeCostModel` | proxy.rs, idle.rs, registry.rs, engine/ |
| image-gen-inference-blueprint | 1,999 | **No** | **No** | Yes | `PerImageCostModel` | diffusion.rs (ComfyUI/SD proxy) |
| video-gen-inference-blueprint | 2,606 | **No** | **No** | Yes | `PerSecondCostModel` | video.rs (Hunyuan/LTX) |
| distributed-inference-blueprint | 2,166 | **No** | **No** | Yes | `PerTokenCostModel` | pipeline.rs, network.rs, shard.rs |
| **Total** | **14,504** | 2 of 6 | 2 of 6 | 5 of 6 | | |

### Divergence observations
- **voice is closest to vllm**: has full billing/metrics, uses per-char pricing — should use `PerCharCostModel`
- **modal is most complex**: multi-task pricing, idle manager, model registry — needs `TaskTypeCostModel`
- **embedding/image-gen/video-gen have NO billing code**: they were built as stubs without on-chain settlement. Gen 3 adds billing via core.
- **distributed has NO billing but has unique networking code**: pipeline.rs, network.rs, shard.rs for peer activation forwarding. These stay untouched. Billing integration via core.

### Rename question
The user wants `vllm-inference-blueprint` renamed. Options:
- `llm-inference-blueprint` (service-type name) — **chosen**
- Keep `vllm-inference-blueprint` (too specific, implies vLLM is the only backend)

vLLM becomes an operator-level config choice within `llm-inference-blueprint`. Other operators could run the same blueprint with a different LLM backend (SGLang, TGI, Ollama) if they want.

### Migration blockers from Gen 2 experience
- Wire-format breaking change: pricing moved from `billing.*` to `vllm.*` in vllm. Gen 3 will do the same per-blueprint (pricing becomes backend-config).
- Test count changes: ~10 tests moved from vllm to core. Same will happen per-blueprint.
- Config rewrite: each blueprint's config.rs shrinks significantly.

## Current Baselines
- 1 of 7 inference blueprints using tangle-inference-core (vllm pilot)
- ~14,500 LOC duplicated across 6 unmigrated blueprints
- 0 operators can easily run multiple inference blueprints against the same core version

## Diagnosis
**Divergence compounds linearly**. Each unmigrated blueprint copies the closest neighbor's billing/metrics/server code and adds its own variations. Without forcing adoption, the 14,500 LOC of duplication will grow with every new blueprint, and per-blueprint security fixes will have to be applied N times. The shared crate is the only scalable path.

**The rename is overdue**: `vllm-inference-blueprint` implies vLLM is the blueprint, when really it's a backend *inside* an LLM chat blueprint. Renaming now, before 6 more blueprints depend on the name, minimizes churn.

## Generation 3 Design

### Thesis
Apply the Gen 2 vllm migration pattern to all 6 remaining blueprints in parallel, rename vllm → llm for semantic correctness, and publish a migration path document that captures the repeatable recipe.

### Changes (ordered by dependency)

#### Rename (must ship first — it's a cross-repo churn and references must be consistent)
1. **Rename `vllm-inference-blueprint` directory** → `llm-inference-blueprint`
2. **Rename operator crate** `vllm-inference` → `llm-inference`
3. **Update Cargo.toml package name + workspace references**
4. **Update CLAUDE.md, PLAN.md, README.md** to reflect new name
5. **Update docs/gpu-provisioning-flows.md** and other docs referencing vllm-inference-blueprint

#### Migration path document (must ship — the reference for future blueprints)
6. **`~/code/tangle-inference-core/MIGRATION.md`** — step-by-step recipe with before/after code snippets from the llm-inference-blueprint reference

#### 6 blueprint migrations (parallelizable)
7. **voice-inference-blueprint** → core (PerCharCostModel)
8. **embedding-inference-blueprint** → core (PerTokenCostModel, adds billing where missing)
9. **modal-inference-blueprint** → core (TaskTypeCostModel composition)
10. **image-gen-inference-blueprint** → core (PerImageCostModel, adds billing)
11. **video-gen-inference-blueprint** → core (PerSecondCostModel, adds billing)
12. **distributed-inference-blueprint** → core (PerTokenCostModel, adds billing)

### Alternatives Considered
- **Migrate sequentially**: rejected — parallel migration via background agents is faster and each blueprint is independent
- **Keep vllm name**: rejected — user explicitly asked for rename, and the semantic confusion compounds
- **Skip embedding/image-gen/video-gen (no billing)**: rejected — they need billing to be production-ready inference blueprints; Gen 3 adds it via core

### Risk Assessment
- **Biggest risk**: agents migrate blueprints inconsistently if they don't see the vllm reference. Mitigation: each migration agent gets an explicit pointer to the vllm (renamed to llm) migration as the reference.
- **Second risk**: rename breaks `operator/Cargo.toml` path references. Mitigation: do rename first, then launch parallel agents on the stable new name.
- **Third risk**: blueprints with no current billing code (embedding, image-gen, video-gen, distributed) need billing *added*, not just refactored. Agents need clear instructions for this.
- **Rollback**: each blueprint is independently revertable. Rename is a `git mv` and can be reversed.

### Success Criteria
- `llm-inference-blueprint` compiles, tests pass, clippy clean under new name
- All 6 remaining blueprints depend on tangle-inference-core
- All 6 blueprints compile + tests pass + clippy clean
- Total LOC reduction across 6 blueprints: target ~6,000-8,000 LOC deleted
- `MIGRATION.md` exists with complete recipe, before/after examples, and a checklist
- No regressions in tangle-inference-core tests (19 still pass)
- No regressions in blueprint-remote-providers (270 still pass)

### Non-Goals (explicit)
- Changing the `llm-inference-blueprint` (previously vllm) reference implementation — it's the canonical pattern, leave it alone
- Adding new backends (SGLang, TGI, Ollama) — that's a follow-up
- Renaming other blueprints (voice, embedding, etc.) — only vllm needs the rename because only vllm had a backend-specific name
