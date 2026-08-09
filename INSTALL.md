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

## Build from source

```sh
git clone https://github.com/vivek-x-jha/eva.git
cd eva
cargo install --path .
```

Cargo builds the `eva` binary and installs it in your Cargo bin directory, usually `$HOME/.cargo/bin`.

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
