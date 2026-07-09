# TODO — spaces CLI Improvements

## Completed
- [x] Create this tracking file
- [x] Add abbreviation aliases to all CLI commands in `cli.rs`
  - `huggingface` → also accept `hf`
  - `python` → also accept `py`, `pyspace` (already has `pyworkspace`)
  - `node` → also accept `njs`, `nodespace`
  - `go` → also accept `gospace` (already has `goworkspace`)
  - `lua` → also accept `luaspace` (already has `luaworkspace`)
  - `hardhat` → also accept `hh`
  - `express` → also accept `exp`
  - `flask` → also accept `fl`
  - `django` → also accept `dj`
  - `fastapi` → also accept `fapi`
  - `nextjs` → also accept `nx`, `next`
  - `sveltekit` → also accept `sk`, `sv`
  - `llama-cpp` → already has `llamacpp`; also accept `llm`
- [x] Make `path` argument default to `"."` (current directory) when omitted
- [x] Update help output in `commands/list.rs` with abbreviations + current-dir note
- [x] Rewrite README.md — full docs, usage examples, abbreviation reference table
- [x] Build and verify changes compile (`cargo build`)
- [x] Add `spaces upgrade` / `spaces up` command — uses `self_update` crate to fetch new releases from GitHub Releases (`mackin5150/spaces`)
- [x] Document upgrade flow in README.md

## Remaining / Future
### Medium Priority
- [ ] Publish first GitHub Release on `mackin5150/spaces` (required for `spaces upgrade` to find binaries)
  - Must upload tarball artifacts: `spaces-x86_64-unknown-linux-gnu.tar.gz`, etc.

### Low Priority
- [ ] Test each abbreviation alias works end-to-end (create actual workspaces)
- [ ] Add `--help` to show abbreviations directly in subcommand help text

## Notes
- All commands that create workspaces accept the same optional flags: `--name`, `--stack`, `--pm`, `--install`, `--docker`, `--git`, `--tests`, `--cuda` (where applicable)
- When no path is given, workspace scaffolds into current directory (`.`)
