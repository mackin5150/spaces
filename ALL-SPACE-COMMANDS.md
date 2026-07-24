# All Available Workspace Commands

## Core Language Commands (with stack selection)

### Python Variations
```bash
spaces python <path>              # Basic Python workspace (default: basic, uv package manager)
spaces py <path>                  # Alias for "python"
spaces pyspace <path>             # Alias for "python"
spaces pyworkspace <path>         # Alias for "python"

# Python stacks (via --stack flag):
spaces python --stack django <path>     # Django
spaces python --stack fastapi <path>    # FastAPI  
spaces python --stack flask <path>      # Flask
spaces python --stack huggingface <path>  # HuggingFace
```

### Node.js Variations
```bash
spaces node <path>                 # Basic Node workspace (default: hardhat, npm)
spaces njs <path>                  # Alias for "node"
spaces nodespace <path>            # Alias for "node"

# Node stacks:
spaces node --stack express <path>    # Express
spaces node --stack hardhat <path>    # Hardhat (smart contracts)
spaces node --stack nextjs <path>     # Next.js
spaces node --stack react <path>      # React
spaces node --stack sveltekit <path>  # SvelteKit
spaces node --stack vite <path>       # Vite
```

### Other Languages
```bash
# Go (default: basic, go module)
spaces go <path>
spaces goworkspace <path>
spaces gospace <path>

# Lua (default: basic, makefile)
spaces lua <path>
spaces luaworkspace <path>
spaces luaspace <path>

# C++ llama.cpp (default: llama-cpp, makefile)
spaces cpp <path>
```

## Framework-Specific Shortcuts

| Command | Full Name | Default Stack | Package Manager |
|---------|-----------|---------------|-----------------|
| `hf`    | huggingface  | uv             | npm/pnpm/yarn/uv/pip/go/make/cargo/forge |
| `fl`    | flask        | uv             | same as above |
| `dj`    | django       | uv             | same as above |
| `fapi`  | fastapi      | uv             | same as above |
| `hh`    | hardhat      | npm            | same as above |
| `exp`   | express      | npm            | same as above |
| `nx`/`next` | nextjs     | npm            | same as above |
| `sk`/`sv` | sveltekit   | npm            | same as above |

## Inference Interface (Special Command)

```bash
# Creates a chat UI for any text model
spaces iface <model> [output-path]

Examples:
  spaces iface meta-llama/Llama-3.2-1B                    # HF repo, scaffold in cwd
  spaces iface ./models/mistral/                          # Local path
  spaces iface Qwen/Qwen2.5-7B-Instruct --cuda -i         # CUDA + auto-install
```

## Common Options (All Commands)

| Flag | Description |
|------|-------------|
| `--name <NAME>` | Custom project name (default: derived from path) |
| `--stack <STACK>` | Framework preset (see stacks above) |
| `--pm <MANAGER>` | Package manager choice |
| `-i` / `--install` | Run package install after scaffold |
| `--docker` | Generate Dockerfile (+ CUDA base if `--cuda`) |
| `--git` | Initialize git repo |
| `--tests` | Include test scaffolding |
| `--cuda` | Enable CUDA support (Python/C++ only) |

## Utility Commands

```bash
spaces list           # Show all workspace types and abbreviations
spaces upgrade        # Update to latest version from GitHub Releases
spaces up             # Alias for upgrade
spaces doctor         # Diagnose current workspace (run inside a project)
```

## Quick Examples

```bash
# Scaffold in current directory:
spaces hf                  # HuggingFace workspace here
spaces py --stack flask    # Flask here
spaces nx                  # Next.js here

# Scaffold into specific directory:
spaces hf ./my-ai-project
spaces dj my-django-app
spaces fapi services/api --name api --pm uv --install
```

