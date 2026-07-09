# spaces

Universal tuned developer workspaces — scaffold a ready-to-code project in seconds.

## Quick Start

```bash
# Scaffold in current directory (no path needed)
spaces hf
spaces py --stack flask
spaces nx

# Scaffold into a specific directory
spaces hf ./my-ai-project
spaces dj my-django-app
spaces hh ./smart-contract
```

## Supported Workspaces

| Command | Abbreviations | Language | Stack |
|---------|---------------|----------|-------|
| `python` | `py`, `pyspace`, `pyworkspace` | Python | configurable |
| `node` | `njs`, `nodespace` | Node.js | configurable |
| `go` | `goworkspace`, `gospace` | Go | configurable |
| `lua` | `luaworkspace`, `luaspace` | Lua | configurable |
| `hardhat` | `hh` | Node.js | Hardhat |
| `express` | `exp` | Node.js | Express |
| `flask` | `fl` | Python | Flask |
| `django` | `dj` | Python | Django |
| `fastapi` | `fapi` | Python | FastAPI |
| `huggingface` | `hf` | Python | HuggingFace |
| `vite` | — | Node.js | Vite |
| `nextjs` | `nx`, `next` | Node.js | Next.js |
| `sveltekit` | `sk`, `sv` | Node.js | SvelteKit |
| `llama-cpp` | `llamacpp`, `llm` | C++ | llama.cpp |

## Options

All workspace commands accept these optional flags:

| Flag | Description |
|------|-------------|
| `--name <NAME>` | Project name (default: derived from path) |
| `--stack <STACK>` | Framework/stack preset (where applicable) |
| `--pm <MANAGER>` | Package manager (`npm`, `pnpm`, `yarn`, `uv`, `pip`, `go`, `make`) |
| `--install` | Run package install after scaffold |
| `--docker` | Generate Dockerfile + docker-compose |
| `--git` | Initialize git repository |
| `--tests` | Include test scaffolding |
| `--cuda` | Enable CUDA support (Python/C++ workspaces) |

## Utility Commands

```bash
spaces list      # Show all available workspace types and abbreviations
spaces upgrade   # Update to latest version from GitHub Releases
spaces up        # Alias for upgrade
spaces doctor    # Diagnose current workspace (run inside a project)
```

## Examples

```bash
# HuggingFace workspace with CUDA, in current dir
spaces hf --cuda --docker --git --install

# FastAPI project named "api" in ./services/api/
spaces fapi services/api --name api --stack fastapi --pm uv --install

# Next.js app in a subdirectory
spaces nx ./web --pm pnpm --docker

# Hardhat smart contract project here, with tests
cd contracts && spaces hh --tests --git
```

## Installation

From source (development):

```bash
git clone https://github.com/mackin5150/spaces.git
cd spaces
cargo install --path .
```

Or build locally:

```bash
cargo build --release
./target/release/spaces --help
```

## Updating

Once installed, update with a single command. Requires the binary to have been published as a GitHub Release on [`mackin5150/spaces`](https://github.com/mackin5150/spaces):

```bash
spaces upgrade
# or: spaces up
```
