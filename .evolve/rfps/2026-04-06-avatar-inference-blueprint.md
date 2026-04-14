---
type: rfp
status: review
date: 2026-04-06
author: claude
---

# RFP: Avatar Inference Blueprint

## Context

6 inference blueprints exist: LLM, voice, image, video-gen, embedding, distributed. None produce talking-head/avatar content. The video-gen blueprint does generative video (Hunyuan/LTX via ComfyUI), which is text/image-to-cinematic-video, not face animation.

An avatar blueprint would complete the "full UGC pipeline on Tangle" story:
```
LLM (script) → Voice (narration) → Image (scenes) → Avatar (talking head) → Video-Gen (composite)
```

## What an Avatar Blueprint Does

Accepts: audio file + face image (or avatar preset)
Returns: video of the face speaking the audio with lip-sync

This is what HeyGen, D-ID, Hedra do. The blueprint wraps this capability for Tangle operators.

## Architecture Options

### Option A: API Proxy Blueprint (ship fast)

Operator wraps a commercial API (HeyGen, D-ID, or Hedra).

```
Client → Tangle Router → Avatar Blueprint Operator → HeyGen API → video returned
                          ↑
                   Operator pays HeyGen upstream
                   Client pays operator via x402
```

**Pros:** Ships in days. Best-in-class quality (HeyGen Avatar IV). Same pattern as existing blueprints.
**Cons:** Operator needs their own HeyGen API key. Margin depends on HeyGen pricing. Not truly decentralized.

### Option B: Self-Hosted Open Source (ship slower, true decentralization)

Operator runs open-source lip-sync models on GPU.

SOTA open-source options (April 2026):
- **ByteDance OmniHuman-1** — best open-source avatar quality, any reference image, but ~48GB VRAM
- **SadTalker** — mature, lower VRAM (~8GB), audio-to-face animation
- **Wav2Lip** — oldest, most stable, lowest quality
- **MuseTalk** — real-time lip-sync, ~12GB VRAM
- **LivePortrait** — expression transfer, Hugging Face spaces available

Could run via ComfyUI (same backend as video-gen-blueprint) with custom nodes for lip-sync.

**Pros:** No API dependency. Truly decentralized. Operators keep full margin.
**Cons:** Quality gap vs HeyGen. Higher VRAM requirements. More complex operator setup.

### Option C: Dual Mode (recommended, matches video-gen-blueprint pattern)

The video-gen-blueprint already supports both ComfyUI (self-hosted) and API (Modal/Replicate). Same pattern:

```rust
enum AvatarBackend {
    ComfyUI { workflow: String },     // Self-hosted: SadTalker/MuseTalk via ComfyUI
    HeyGen { api_key: String },       // Commercial proxy
    DID { api_key: String },          // Commercial proxy
    Replicate { api_token: String },  // Hosted open-source models
}
```

Operator chooses their backend. Clients don't care which — they get the same API.

## Endpoints

Following the pattern of existing inference blueprints:

```
POST /v1/avatar/generate
{
  "audio_url": "https://...",     // narration audio
  "image_url": "https://...",     // face image (or avatar_id for presets)
  "avatar_id": "preset-1",       // optional: use a preset avatar
  "duration_seconds": 30,
  "output_format": "mp4",
  "resolution": "1080p"
}

→ 202 Accepted (async job)
{
  "job_id": "...",
  "status": "processing",
  "poll_url": "/v1/avatar/jobs/{job_id}"
}

GET /v1/avatar/jobs/{job_id}
→ { "status": "completed", "video_url": "https://...", "duration": 28.5 }
```

## Contract (VideoGenBSM pattern)

Same pattern as `video-gen-inference-blueprint/contracts/`:
- VRAM validation for self-hosted operators
- Per-second pricing
- Duration limits
- Result hash verification

## Relationship to Router

The Router's model list would include avatar models:
```
heygen/avatar-iv          (via operator running HeyGen proxy)
sadtalker/v2              (via operator running ComfyUI self-hosted)
musetalk/realtime         (via operator running self-hosted)
```

Clients call `POST router.tangle.tools/v1/avatar/generate`, Router selects best operator.

## Effort Estimate

- Option A (API proxy only): ~2-3 days (copy video-gen-blueprint pattern, swap endpoints)
- Option B (self-hosted only): ~1-2 weeks (ComfyUI workflow authoring, VRAM testing)
- Option C (dual mode): ~1 week for proxy, +1 week for self-hosted

## Decision Needed

1. Start with Option A (fastest) or Option C (most complete)?
2. Which commercial API to target first: HeyGen (best quality) or D-ID (cheapest, most API-native)?
3. Priority relative to other blueprint work?
