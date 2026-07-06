# Testing eva

`eva` uses the upstream eza Rust test suite plus fork-specific tests for config and theme behavior.

Useful local checks:

```sh
cargo fmt
cargo test
cargo build --release
```

CLI tests use `trycmd` fixtures under `tests/cmd` and should refer to the `eva` binary.

When updating output fixtures, build first and use `devtools/generate-trycmd-test.sh`.
