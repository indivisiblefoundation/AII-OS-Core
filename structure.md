AII-OS-Core/
├── architecture-engine/src/lib.rs + Cargo.toml (ifc-rs, opa)
├── kinetic-engine/simulation.py + requirements.txt (numpy, radiance, energyplus)
├── guerillanet-p2p/agent.rs + Cargo.toml (quinn, libp2p-kad, wasmtime, wasi-nn)
├── flywheel-engine/flywheel.py (pgvector, qdrant)
├── compliance-engine/compliance.py (pydantic rubrics)
├── mainframe-core/workflow.py (temporalio, nats)
├── api/server.py (FastAPI)
├── simulation-lab/visual_3d.html
├── docker-compose.yml (6 services, NATS, Postgres, Qdrant)
├── Dockerfile per engine (WASI isolated)
└──.github/workflows/ci.yml (cargo test + pytest + fuel audit)
