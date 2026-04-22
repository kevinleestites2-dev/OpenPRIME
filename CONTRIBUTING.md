# Contributing to OpenPRIME

OpenPRIME is being built in the open. Contributions are welcome.

## Setup

```bash
git clone https://github.com/your-org/OpenPRIME
cd OpenPRIME
cargo build --workspace
cargo test --workspace
```

## Crate guide

| Want to work on | Crate |
|---|---|
| Core orchestration, scheduling | `prime-kernel` |
| Agent loop, LLM drivers, tools | `prime-runtime` |
| Multi-agent parallelism | `prime-swarm` |
| Self-improving skill system | `prime-skills` |
| Persistent memory & search | `prime-memory` |
| Telegram/Discord/Slack/etc | `prime-channels` |
| REST API, dashboard | `prime-api` |
| CLI, TUI, `prime` binary | `prime-cli` |

## Code style

- Run `cargo fmt --all` before committing
- Zero clippy warnings: `cargo clippy --workspace -- -D warnings`
- All public functions must have doc comments
- Every new module needs at least one test

## Pull requests

- One PR per feature or fix
- Describe what changed and why
- Add tests for new functionality

## License

MIT — all contributions are MIT licensed.
