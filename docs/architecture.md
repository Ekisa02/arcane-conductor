# Arcane Conductor — Architecture Overview

This document outlines the architecture and rationale for Arcane Conductor.

Components

- Planner: Responsible for converting high-level goals into structured Plans.
  - Current: SimplePlanner — a rule-based splitter.
  - Extend: Replace with an LLM-based planner returning structured JSON.

- Executor: Runs Actions. Can call Tools / connectors when an action requires external capabilities.
  - Current: Sequential executor using a single Tool interface.

- Memory: In-memory store of plans and event traces. Provides easy persistence swap.

- Tools / Connectors: Abstracted as the ToolHandle trait. Implementations can wrap external APIs.

- Orchestrator: Coordinates planner + executor + memory + registry; suitable to be run inside async tasks.

- API (Actix): Exposes endpoints to create tasks and get live status. Uses application state for registry and memory.

Concurrency Model

- HTTP handlers create a background tokio task for orchestration so requests return immediately with a task id.
- Memory and registry use async RwLocks for safe concurrent access.

Extensibility

- Swap MemoryStore with Redis/Postgres by implementing an API compatible wrapper.
- Add concurrency/parallel action execution in Executor.
- Add job persistence, retries, and observability hooks.
