# AGENTS.md

## Scope

- Eva is a personal Rust fork of `eza`; keep fork-specific changes narrow and easy to replay on upstream.
- Preserve the `eva` binary/package name, `EVA_*` variables, eva config paths, release assets, man pages, and completions while retaining documented `EZA_*`/`EXA_*` fallbacks.

## Entry Points

- Source: `src/main.rs`, `src/lib.rs`; CLI parsing under `src/options`, rendering under `src/output`, filesystem behavior under `src/fs`, and theme handling under `src/theme`.
- Tests: unit tests beside source, CLI snapshots in `tests/cmd`, generated integration snapshots in `tests/gen` and `tests/ptests`.
- Docs: `README.md`, `INSTALL.md`, `TESTING.md`, `man/`, and `docs/theme.yml`.

## Commands

- Install/setup: `rustup show` (the pinned toolchain is read from `rust-toolchain.toml`), or `nix develop`.
- Validate: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Test: `cargo test --workspace`; full Nix integration check: `just itest`.

## Rules

- Preserve user-owned changes and secrets.
- Do not edit generated, vendored, or runtime-owned files unless explicitly requested.
- Keep changes narrow and update docs when behavior changes.

## Memory

- Use this file for repo-specific operating rules only.
- Use `docs/known-issues.md` for recurring bugs, workarounds, reproduction steps, and exit criteria.
- Use `docs/agent-memory.md` for durable project facts that are likely to matter in future sessions.
