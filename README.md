<!--
SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: EUPL-1.2
-->

<div align="center">

# eva

A personal fork of [eza](https://github.com/eza-community/eza), a modern replacement for `ls`.

[![Built with Nix](https://img.shields.io/badge/Built_With-Nix-5277C3.svg?logo=nixos&labelColor=73C3D5)](https://nixos.org)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

</div>

![eva demo screenshot](docs/images/screenshots.png)

---

**eva** is a personal fork of **eza**, a modern alternative for the venerable file-listing command-line program `ls` that ships with Unix and Linux operating systems, giving it more features and better defaults.
It uses colours to distinguish file types and metadata.
It knows about symlinks, extended attributes, and Git.
And it’s **small**, **fast**, and just **one single binary**.

By deliberately making some decisions differently, eva aims to stay a featureful, user-friendly version of `ls` while keeping this fork easy to update from upstream eza.

---

Upstream **eza** features not in exa (non-exhaustive):

- Fixes [“The Grid Bug”](https://github.com/eza-community/eza/issues/66#issuecomment-1656758327) introduced in exa 2021.
- Hyperlink support.
- Mount point details.
- Selinux context output.
- Git repo status output.
- Human readable relative dates.
- Several security fixes.
- Support for `bright` terminal colours.
- Many smaller bug fixes/changes!
- Configuration `theme.yml` file for customization of colors and icons.

...and like, so much more that it became exhausting to update this all the time.
Like seriously, we have a lot of good stuff.

---

<a id="try-it">
<h1>Try it!</h1>
</a>

### Nix ❄️

If you already have Nix setup with flake support, you can try out eva with the `nix run` command:

    nix run github:vivek-x-jha/eva

Nix will build eva and run it.

If you want to pass arguments this way, use e.g. `nix run github:vivek-x-jha/eva -- -ol`.

# Installation

eva is available for Windows, macOS and Linux. Platform and distribution
specific installation instructions can be adapted from [INSTALL.md](INSTALL.md).

---

<a id="options">
<h1>Command-line options</h1>
</a>

eva’s options are almost, but not quite, entirely unlike `ls`’s. Quick overview:

## Display options

<details>
<summary>Click to expand</summary>

- **-1**, **--oneline**: display one entry per line
- **-G**, **--grid**: display entries as a grid (default)
- **-l**, **--long**: display extended details and attributes
- **-R**, **--recurse**: recurse into directories
- **-T**, **--tree**: recurse into directories as a tree
- **-x**, **--across**: sort the grid across, rather than downwards
- **-F**, **--classify=(when)**: display type indicator by file names (always, auto, never)
- **--colo[u]r=(when)**: when to use terminal colours (always, auto, never)
- **--colo[u]r-scale=(field)**: highlight levels of `field` distinctly(all, age, size)
- **--color-scale-mode=(mode)**: use gradient or fixed colors in --color-scale. valid options are `fixed` or `gradient`
- **--icons=(when)**: when to display icons (always, auto, never)
- **--hyperlink=(when)**: when to display entries as hyperlinks (always, auto, never)
- **--absolute=(mode)**: display entries with their absolute path (on, follow, off)
- **-w**, **--width=(columns)**: set screen width in columns

</details>

## Filtering options

<details>
<summary>Click to expand</summary>

- **-a**, **--all**: show hidden and 'dot' files
- **-d**, **--treat-dirs-as-files**: list directories like regular files
- **-L**, **--level=(depth)**: limit the depth of recursion
- **-r**, **--reverse**: reverse the sort order
- **-s**, **--sort=(field)**: which field to sort by
- **--group-directories-first**: list directories before other files
- **--group-directories-last**: list directories after other files
- **-D**, **--only-dirs**: list only directories
- **-f**, **--only-files**: list only files
- **--no-symlinks**: don't show symbolic links
- **--show-symlinks**: explicitly show links (with `--only-dirs`, `--only-files`, to show symlinks that match the filter)
- **--git-ignore**: ignore files mentioned in `.gitignore`
- **-I**, **--ignore-glob=(globs)**: glob patterns (pipe-separated) of files to ignore

Pass the `--all` option twice to also show the `.` and `..` directories.

</details>

## Long view options

<details>
<summary>Click to expand</summary>

These options are available when running with `--long` (`-l`):

- **-b**, **--binary**: list file sizes with binary prefixes
- **-B**, **--bytes**: list file sizes in bytes, without any prefixes
- **-g**, **--group**: list each file’s group
- **--smart-group**: only show group if it has a different name from owner
- **-h**, **--header**: add a header row to each column
- **-H**, **--links**: list each file’s number of hard links
- **-i**, **--inode**: list each file’s inode number
- **-m**, **--modified**: use the modified timestamp field
- **-M**, **--mounts**: Show mount details (Linux and MacOS only).
- **-S**, **--blocksize**: show size of allocated file system blocks
- **-t**, **--time=(field)**: which timestamp field to use
- **-u**, **--accessed**: use the accessed timestamp field
- **-U**, **--created**: use the created timestamp field
- **-X**, **--dereference**: dereference symlinks for file information
- **-Z**, **--context**: list each file’s security context
- **-@**, **--extended**: list each file’s extended attributes and sizes
- **--changed**: use the changed timestamp field
- **--git**: list each file’s Git status, if tracked or ignored
- **--git-repos**: list each directory’s Git status, if tracked
- **--git-repos-no-status**: list whether a directory is a Git repository, but not its status (faster)
- **--no-git**: suppress Git status (always overrides `--git`, `--git-repos`, `--git-repos-no-status`)
- **--time-style**: how to format timestamps. valid timestamp styles are ‘`default`’, ‘`iso`’, ‘`long-iso`’, ‘`full-iso`’, ‘`relative`’, or a custom style ‘`+<FORMAT>`’ (E.g., ‘`+%Y-%m-%d %H:%M`’ => ‘`2023-09-30 13:00`’. For more specifications on the format string, see the _`eva(1)` manual page_ and [chrono documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).).
- **--total-size**: show recursive directory size
- **--no-permissions**: suppress the permissions field
- **-o**, **--octal-permissions**: list each file's permission in octal format
- **--no-filesize**: suppress the filesize field
- **--no-user**: suppress the user field
- **--no-time**: suppress the time field
- **--stdin**: read file names from stdin

Some of the options accept parameters:

- Valid **--colo\[u\]r** options are **always**, **automatic** (or **auto** for short), and **never**.
- Valid sort fields are **accessed**, **changed**, **created**, **extension**, **Extension**, **inode**, **modified**, **name**, **Name**, **size**, **type**, and **none**. Fields starting with a capital letter sort uppercase before lowercase. The modified field has the aliases **date**, **time**, and **newest**, while its reverse has the aliases **age** and **oldest**.
- Valid time fields are **modified**, **changed**, **accessed**, and **created**.
- Valid time styles are **default**, **iso**, **long-iso**, **full-iso**, and **relative**.



See the `man` pages for further documentation of usage. They are available
- online [in the repo](man)
- in your terminal via `man eva` when installed with man pages
</details>


## Custom Themes
<details>
<summary>Click to expand</summary>

**Eva** supports a `theme.yml` file, where you can specify all of the existing theming options
available for the `LS_COLORS` and `EVA_COLORS` environment variables, as well as the option to specify different icons
for different file types and extensions. Legacy `EZA_*` and `EXA_*` environment variables continue to work as fallbacks.

#### **New** Pre-made themes
Check out the themes available in the official [eza-themes](https://github.com/eza-community/eza-themes) repository, or contribute your own.

An example theme file is available in `docs/theme.yml`, and needs to either be placed in a directory specified by the 
environment variable `EVA_CONFIG_DIR`, or will be looked for by default in `$XDG_CONFIG_HOME/eva` with a `$HOME/.config/eva` fallback. If no eva theme is found, eva also checks legacy `EZA_CONFIG_DIR`, `$XDG_CONFIG_HOME/eza`, and `$HOME/.config/eza` locations.

Sparse icon defaults can be set without copying the full built-in icon map. Directory icons are semantic: all non-empty directories use `folder`, and empty directories use `empty_folder` when set, otherwise `folder`. Filename and extension overrides apply to files only, so names such as `.config` or `.cache` cannot steal directory icon/color precedence.

```yaml
icons:
  folder: { glyph: "" }
  empty_folder: { glyph: "" }
  file: { glyph: "" }
  unknown_file: { glyph: "" }
```

Git status markers can also be customized independently from their colors:

```yaml
git:
  staged: { foreground: brightgreen }
  new: { foreground: red }
  modified: { foreground: yellow }
  ignored: { foreground: brightblack }
  conflicted: { foreground: brightred }
git_markers:
  staged: "+"
  new: "?"
  modified: "~"
  ignored: "I"
  conflicted: "!"
```

Full details are available on the [man page](man/eva_colors-explanation.5.md) and an example theme file is included [here](docs/theme.yml)

</details>


# Hacking on eva

This fork tracks upstream [eza](https://github.com/eza-community/eza). Keep fork-specific changes narrow and easy to rebase.
