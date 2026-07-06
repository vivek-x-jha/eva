# Contributing to eva

`eva` is a personal fork of [`eza`](https://github.com/eza-community/eza).

Keep fork-specific changes narrow, documented, and easy to rebase onto upstream eza. For general project architecture and testing guidance, upstream eza documentation is still useful, but user-facing names, paths, release assets, and primary environment variables in this fork should use `eva`/`EVA_*`.

## Useful local checks

```sh
cargo fmt
cargo test
cargo build --release
```

Use Conventional Commit style for commit messages unless matching a clearer existing pattern.
