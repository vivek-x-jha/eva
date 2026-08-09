<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->

# Installing eva

`eva` is a personal fork of `eza`.

## Prebuilt binary

Eva v0.4.0 provides an unsigned Apple Silicon macOS binary on the GitHub release page:

- `eva_aarch64-apple-darwin.tar.gz`
- `eva_aarch64-apple-darwin.tar.gz.sha256`

Extract the archive and move `eva` somewhere on your `PATH`.

## Install with Cargo

Cargo users can install the latest Eva directly from GitHub:

```sh
cargo install --locked --git https://github.com/vivek-x-jha/eva
```

This installs the `eva` binary in your Cargo bin directory and lets `cargo install-update --git` update it later.

## Build from a checkout

```sh
git clone https://github.com/vivek-x-jha/eva.git
cd eva
cargo install --locked --path .
```

## Try without installing

```sh
cargo run -- --version
cargo run -- --icons always
```

## Shell completions and man pages

Completion files are in `completions/` and are named for `eva`:

- Bash: `completions/bash/eva`
- Fish: `completions/fish/eva.fish`
- Zsh: `completions/zsh/_eva`

Man page Markdown sources are in `man/`:

- `man/eva.1.md`
- `man/eva_colors.5.md`
- `man/eva_colors-explanation.5.md`
