# spaces — TODO / Feature Backlog

## Completed ✅

- [x] **`iface` workspace** — one-command inference interface for any text model (HF repo or local path). Generates FastAPI server + web chat UI. Aliases: `chat`, `serve`. (`src/commands/iface.rs`)
- [x] **Help menu updated** — `spaces list` now documents the `iface` command with usage examples. (`src/commands/list.rs`)

## In Progress / Next 🔄

### Abbreviations & Shortcuts

| Goal | Status | Notes |
|------|--------|-------|
| `spaces hf` → HuggingFace | ✅ Done (existing alias) | Already works via `#[command(alias = "hf")]` |
| `spaces py`, `pyspace`, `pyworkspace` → Python | ✅ Done (existing aliases) | — |
| `spaces njs`, `nodespace` → Node | ✅ Done (existing aliases) | — |
| `spaces nx`, `next` → Next.js | ✅ Done (existing aliases) | — |
| `spaces sk`, `sv` → SvelteKit | ✅ Done (existing aliases) | — |
| `spaces llm`, `llamacpp` → llama-cpp | ✅ Done (existing aliases) | — |
| **Add more popular abbreviations** | 🔄 TODO | e.g. `api` for fastapi, `web` for vite/nextjs, `ai/ml` for huggingface/iface |

### Current-Directory Default

| Goal | Status | Notes |
|------|--------|-------|
| `spaces <workspace>` installs in cwd when no path given | ✅ Done (all commands use `default_value = "."`) | — |
| Help menu mentions "omit path to scaffold here" | ✅ Done (`src/commands/list.rs` line 17-19) | — |

### README Updates

| Goal | Status | Notes |
|------|--------|-------|
| Add `iface` workspace to Supported Workspaces table | 🔄 TODO | Include model param, aliases |
| Add examples for `iface` usage | 🔄 TODO | Show HF repo + local path examples |
| Document all abbreviations in README | 🔄 TODO | Ensure parity with CLI help |

## Ideas / Future 🚀

- [ ] **Fuzzy matching** — type `spaces hugg` and it resolves to HuggingFace workspace
- [ ] **Interactive mode** — no args → prompt for language/stack/model selection
- [ ] **Workspace templates marketplace** — fetch community-maintained workspace configs from GitHub/GitLab
- [ ] **Model download progress** — show progress bar when `iface` downloads HF models
- [ ] **Multi-model iface** — scaffold a server that loads multiple models on different routes
