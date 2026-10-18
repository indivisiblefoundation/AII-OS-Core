# AII-OS-Core — DNA Framework
**Thesis: We don't draw buildings. We simulate 100 futures for them.**
**Formula: `(Border + Motion) x 100 Channels x Brain ^ On-Demand Recall`**
### DNA is not a layer. It is the structure.
Like human DNA, 4 bases = 4 protocols in every cell:
- **A = Audit (WORM)** — every event immutable
- **T = Trust (wasmtime + Fuel 100M)** — all agents sandboxed
- **C = Compliance (Edge-Case 100)** — 100 edge-cases must pass
- **G = Glue (NATS + CRDT + Temporal)** — message, state, time
> No engine can run without `dna::check()` — kill DNA, entire body dies. Intentional. Brain is the enforcer.
### 7 Organs
1.  **Architecture (Skeletal)** + `BorderAgent` — borders, zoning
2.  **Kinetic (Muscular)** + `MotionAgent` (WASI-NN) — flow, egress
3.  **GuerillaNet (Nervous)** + `SwarmAgent` (RL) — 100 channels
4.  **Brain (Cerebral)** + `BrainAgent` — **NEW** — JetStream scheduler, pull consumer `brain` durable, reasons over FILTERED, calls `dna::check()` before any action. If brain dies, no organ fires.
5.  **Flywheel (Memory)** + `RecallAgent` (CLIP + reward) — Qdrant:6333 on-demand recall
6.  **Compliance is DNA itself** — bloodstream, not an organ
7.  **Mainframe (Heart)** + `OrchestratorAgent` — `dna-framework:8080`, pumps blood
**Interdependence: Circular.** DNA needs organs for data, organs need DNA + Brain to start. Enforced via docker healthcheck + `depends_on: service_healthy`.
### Runtime
Producers -> raw.> -> [RAW: 100k msgs] -> filter-engine (Dedupe/RateLimit/PII) -> filtered.> -> [FILTERED] -> brain-engine -> qdrant / mainframe
^ ^ ^
NATS:4222 + 8222 (-js) Brain is gatekeeper here dna::check()

**brain-engine today:**
```rust
get_or_create_stream(RAW: raw.>, FILTERED: filtered.>)
create_consumer(durable: brain)
loop { msg = fetch().await; dna::check(); reason(); ack() }
Quick Start
Bash
sudo docker compose up -d nats qdrant
sudo docker compose up --build -d brain-engine dna-framework
sudo docker compose logs -f brain-engine
# BRAIN ENGINE running - waiting for messages
Stack: Rust 1.85+ (edition2024), async-nats 0.34 (no jetstream feature), docker compose v2.

Indivisible Foundation — Border + Motion x 100 x Brain ^ Recall
MD
