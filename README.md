<div align="center">

```
 ██████╗ ██████╗ ███████╗███╗   ██╗██████╗ ██████╗ ██╗███╗   ███╗███████╗
██╔═══██╗██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔══██╗██║████╗ ████║██╔════╝
██║   ██║██████╔╝█████╗  ██╔██╗ ██║██████╔╝██████╔╝██║██╔████╔██║█████╗  
██║   ██║██╔═══╝ ██╔══╝  ██║╚██╗██║██╔═══╝ ██╔══██╗██║██║╚██╔╝██║██╔══╝  
╚██████╔╝██║     ███████╗██║ ╚████║██║     ██║  ██║██║██║ ╚═╝ ██║███████╗
 ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝╚═╝     ╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚══════╝
```

### **The legendary open agentic OS.**
### Self-improving. Parallel. Unstoppable.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust-orange?style=flat-square)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/status-building_legend-gold?style=flat-square)]()

</div>

---

## What is OpenPRIME?

OpenPRIME is not a chatbot. It is not a wrapper around an LLM. It is not another Python agent framework.

**OpenPRIME is a full agentic operating system** — built in Rust from the ground up — that thinks, learns, and acts autonomously on your behalf, 24 hours a day, without you having to ask.

It combines three proven ideas into one legendary system:

| Inspiration | What we took |
|---|---|
| **Hermes Agent** | Self-improving skills, cross-session memory, closed learning loop |
| **OpenFang** | Rust-native OS architecture, secure sandboxed runtime, channel adapters |
| **OpenAI SWARM** | Multi-agent parallelism, task routing, result merging |

The result is something none of them are alone: **an OS that gets smarter every time it runs.**

---

## Core principles

**1. It learns.** After every complex task, OpenPRIME writes or improves a `SKILL.md` — domain expertise that gets injected into every future agent that needs it. Your system gets smarter the more you use it.

**2. It parallelizes.** The SWARM coordinator spawns isolated sub-agents for complex tasks, runs them in parallel, and merges the results. One instruction can become ten agents working simultaneously.

**3. It remembers.** Cross-session SQLite + vector memory means OpenPRIME builds a deepening model of you, your preferences, your work, and your goals — across every conversation, forever.

**4. It runs anywhere.** A single compiled Rust binary. No Python environment. No Docker required. Drop it on a $5 VPS, a GPU server, or your laptop. It wakes when needed, sleeps when idle.

**5. It's open.** MIT licensed. No lock-in. No cloud required. Yours completely.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│              SWARM Coordinator                       │
│   Researcher · Coder · Analyst · Custom · ...        │
├─────────────────────────────────────────────────────┤
│           Hermes Skill Layer                         │
│   SKILL.md · Memory loop · User profile · FTS5      │
├─────────────────────────────────────────────────────┤
│              OpenPRIME Kernel                        │
│   Scheduler · RBAC · Budget · Lifecycle · Audit      │
├──────────────┬──────────────────────────────────────┤
│   Runtime    │  53 tools · MCP · A2A · WASM sandbox  │
├──────────────┴──────────────────────────────────────┤
│   Channels   │  Telegram · Discord · Slack · CLI...  │
├──────────────┴──────────────────────────────────────┤
│   Memory     │  SQLite · Vector embeddings · Search  │
└─────────────────────────────────────────────────────┘
```

### Crates

| Crate | Purpose |
|---|---|
| `prime-kernel` | Orchestration, scheduling, RBAC, budget, agent lifecycle |
| `prime-runtime` | Agent loop, LLM drivers, tool execution, WASM sandbox |
| `prime-swarm` | SWARM multi-agent coordinator, task routing, result merger |
| `prime-skills` | Self-improving SKILL.md engine, learner loop, skills hub |
| `prime-memory` | SQLite persistence, vector embeddings, session search |
| `prime-channels` | Messaging adapters (Telegram, Discord, Slack, WhatsApp...) |
| `prime-api` | REST/WS/SSE endpoints, OpenAI-compatible API |
| `prime-cli` | CLI, TUI dashboard, daemon management |

---

## Quick start

```bash
# Install
curl -fsSL https://openprime.sh/install | sh

# Initialize your workspace
prime init

# Start the daemon
prime start

# Chat with your first agent
prime chat

# Spawn a SWARM task
prime swarm spawn "Research the top 10 AI agent frameworks and write a report"

# Check your skills (grows over time)
prime skills list
```

---

## The skill loop

This is what makes OpenPRIME legendary. Every time an agent completes a complex task:

```
Agent completes task
        ↓
prime-skills evaluates complexity
        ↓
If new knowledge detected → writes SKILL.md
        ↓
Next agent doing similar work gets that skill injected
        ↓
Skills improve themselves over time
        ↓
Your system gets smarter. Forever.
```

Skills live in `~/.openprime/skills/` — human-readable Markdown files you can read, edit, or share.

---

## SWARM parallelism

One instruction. Many agents.

```bash
prime swarm spawn "Build me a full competitive analysis of the top 5 AI coding tools"
```

OpenPRIME breaks this into parallel sub-agents:

```
Coordinator
├── Agent A → researches Cursor
├── Agent B → researches GitHub Copilot  
├── Agent C → researches Windsurf
├── Agent D → researches Aider
└── Agent E → researches Continue
         ↓
    Merger combines results
         ↓
    Final report delivered
```

---

## Roadmap

- [x] Project scaffold and architecture
- [ ] `prime-kernel` — full orchestrator
- [ ] `prime-runtime` — agent loop + LLM drivers
- [ ] `prime-swarm` — parallel coordinator
- [ ] `prime-skills` — learning loop
- [ ] `prime-memory` — SQLite + vector store
- [ ] `prime-channels` — Telegram + Discord + CLI
- [ ] `prime-api` — REST API + dashboard
- [ ] `prime-cli` — full TUI
- [ ] First public release (v0.1.0)

---

## Contributing

OpenPRIME is being built in the open. If you want to be part of building something legendary:

```bash
git clone https://github.com/your-org/OpenPRIME
cd OpenPRIME
cargo build --workspace
cargo test --workspace
```

---

## License

MIT — use it however you want. Build something legendary.

---

<div align="center">
<strong>Open. Prime. Unstoppable.</strong>
</div>
