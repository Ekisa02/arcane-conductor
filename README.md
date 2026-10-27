# Arcane Conductor

Arcane Conductor is a modular AI agent framework written in Rust, built on top of Actix (HTTP) and Tokio (async runtime). It provides components for planning, executing, memory management, tool/connectors, and an HTTP API to orchestrate agents and long-running tasks.

Key goals:
- Clear modular architecture for agent components (planner, executor, memory, tools).
- Async-first design leveraging Tokio and Actix for concurrency and web serving.
- Small, extensible core suitable for production or experimentation.

## Architecture

- src/
  - agent/: Planner, Executor and agent orchestration helper types.
  - memory.rs: In-memory memory store for agent state and context.
  - tools.rs: Tool interface and mock implementations (search, webhook, LLM connector stubs).
  - api.rs: Actix-web server endpoints to create and query agent tasks.
  - task.rs: Task model and simple registry.
  - transport.rs: Mock transport/connector helpers.
  - utils.rs: Shared small utilities and types.
  - main.rs: Starts the HTTP server and wires components.

Sequence (simplified):
1. Client POST /agent/run { goal }
2. Planner generates a list of actions (Plan).
3. Executor runs actions sequentially (or concurrently), using Tools/Connectors.
4. Results and intermediate data are persisted to MemoryStore.
5. Client polls GET /agent/status/{id} for progress and results.

## Installation

Prerequisites:
- Rust toolchain (1.60+ recommended)
- Cargo

Clone and build:

```bash
git clone https://github.com/your-org/arcane-conductor.git
cd arcane-conductor
cargo build --release
```

Run the server (default port 8080):

```bash
cargo run --release --bin arcane_conductor_server
```

## Usage

Start the server, then create an agent task with a goal:

```bash
curl -X POST http://127.0.0.1:8080/agent/run -H "Content-Type: application/json" -d '{"goal":"Research Rust async patterns. Summarize in bullet points."}'
```

Response contains a `task_id`. Check status:

```bash
curl http://127.0.0.1:8080/agent/status/<task_id>
```

Example responses are JSON with task status, plan, and results.

## Extending

- Implement a real Planner that queries an LLM and returns structured plans.
- Add real Tool connectors (OpenAI, search engines, databases) by implementing the Tool trait in `src/tools.rs`.
- Swap MemoryStore for Redis or a persistent DB by implementing the same store API.

## Project Layout and Notes

- The code aims to be small and idiomatic.
- Tests include unit tests for planner/memory and an integration test for the HTTP API.

## License

MIT
