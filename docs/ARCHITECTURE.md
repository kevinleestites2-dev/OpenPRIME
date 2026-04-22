# OpenPRIME Architecture

## Overview

OpenPRIME is a full agentic operating system built in Rust. It fuses three proven ideas:

- **The Hermes learning loop** — agents create and improve skills from experience
- **The OpenFang OS model** — secure, fast, single-binary Rust runtime  
- **The SWARM parallelism model** — one task spawns many agents, results merge

---

## Layer model

```
Layer 3 — SWARM Coordinator      (prime-swarm)
Layer 2 — Hermes Skill Layer     (prime-skills + prime-memory)
Layer 1 — OpenPRIME Kernel       (prime-kernel + prime-runtime)
Layer 0 — Infrastructure         (prime-channels + prime-api + prime-cli)
```

Each layer depends only on the layer below it.

---

## Crate responsibilities

### prime-kernel
Owns: agent lifecycle, global scheduler, RBAC, budget tracking, main event loop.

### prime-runtime
Owns: agent loop (observe→think→act), LLM drivers, 53 tools, WASM sandbox, A2A protocol.

### prime-swarm
Owns: task decomposition, parallel agent routing, result merging, skill reporting.

The SWARM flow:
```
Task received → decompose into sub-tasks
    → route to typed agents (researcher, coder, analyst...)
    → run all in parallel via tokio
    → merge results → deliver to user
    → report learnings to prime-skills
```

### prime-skills
Owns: SKILL.md loader/writer, learning loop, skills hub with semantic search.

The skill loop:
```
Task completes → learner scores complexity
    → If score > threshold → LLM writes new SKILL.md
    → Saved to ~/.openprime/skills/
    → Future agents get it injected automatically
```

### prime-memory
Owns: SQLite store, vector embeddings, user profile (USER.md), FTS5 session search.

Memory kinds: Fact, UserProfile, SessionSummary, SkillReference.

### prime-channels
Owns: ChannelAdapter trait, PrimeMessage (platform-agnostic), adapters for Telegram/Discord/Slack/WhatsApp/Signal/Webhook/CLI.

### prime-api
Owns: axum REST API, OpenAI-compatible endpoint, WebSocket/SSE streaming, web dashboard at :2357.

### prime-cli
Owns: daemon management, TUI chat, SWARM control, skill management, system health.

---

## The agent loop

```
1. Receive task
2. Load relevant SKILL.md files → inject into context
3. Load relevant memories → inject into context
4. Run LLM
5. Execute tool calls (WASM sandboxed)
6. Stream output to channel
7. Save results to prime-memory
8. Report to prime-skills learner
9. If SWARM task → send result to merger
```

---

## Security layers

| Layer | Mechanism |
|---|---|
| Tool isolation | WASM dual-metered sandbox |
| Audit trail | Merkle hash-chain |
| Secrets | Zeroizing<String> — wiped from memory immediately |
| Network | SSRF guard |
| Permissions | RBAC capability gates |
| Input | Prompt injection scanner |
| Rate limiting | GCRA per-IP token bucket |

---

## Data files

```
~/.openprime/
├── config.toml
├── memory/
│   ├── prime.db         # SQLite
│   └── embeddings/      # Vector index
├── skills/              # Auto-generated, grows over time
├── sessions/            # Compressed logs
└── logs/
```
