// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::options::{Vars, vars};
use crate::theme::ThemeFileType as FileType;
use crate::theme::{
    FileKinds, FileNameStyle, Git, GitMarkers, GitRepo, IconStyle, IconTheme, Links, Permissions,
    SELinuxContext, SecurityContext, Size, UiStyles, Users,
};
use nu_ansi_term::{Color, Style};
use serde::{Deserialize, Deserializer, Serialize};
use serde_norway;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

struct EnvironmentVars;

impl Vars for EnvironmentVars {
    fn get(&self, name: &'static str) -> Option<OsString> {
        std::env::var_os(name)
    }
}

fn non_empty_path(value: Option<OsString>) -> Option<PathBuf> {
    value.filter(|path| !path.is_empty()).map(PathBuf::from)
}

fn default_config_dir(
    xdg_config_home: Option<OsString>,
    home: Option<OsString>,
    app_name: &str,
) -> PathBuf {
    if let Some(path) = non_empty_path(xdg_config_home) {
        return path.join(app_name);
    }

    non_empty_path(home)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join(app_name)
}

fn config_dir_from_vars_and_home<V: Vars>(vars: &V, home: Option<OsString>) -> PathBuf {
    if let Some(path) = non_empty_path(vars.get(vars::EVA_CONFIG_DIR)) {
        return path;
    }

    default_config_dir(vars.get(vars::XDG_CONFIG_HOME), home, "eva")
}

fn push_unique(paths: &mut Vec<PathBuf>, path: PathBuf) {
    if !paths.iter().any(|existing| existing == &path) {
        paths.push(path);
    }
}

fn config_dirs_from_vars_and_home<V: Vars>(vars: &V, home: Option<OsString>) -> Vec<PathBuf> {
    if let Some(path) = non_empty_path(vars.get(vars::EVA_CONFIG_DIR)) {
        return vec![path];
    }

    let xdg_config_home = vars.get(vars::XDG_CONFIG_HOME);
    let primary_default = default_config_dir(xdg_config_home.clone(), home.clone(), "eva");
    let legacy_default = default_config_dir(xdg_config_home, home, "eza");

    let mut paths = Vec::new();
    push_unique(&mut paths, primary_default);

    if let Some(path) = non_empty_path(vars.get(vars::EZA_CONFIG_DIR)) {
        push_unique(&mut paths, path);
    }

    push_unique(&mut paths, legacy_default);
    paths
}

#[must_use]
pub fn config_dir_from_vars<V: Vars>(vars: &V) -> PathBuf {
    config_dir_from_vars_and_home(vars, std::env::var_os("HOME"))
}

#[must_use]
pub fn config_dirs_from_vars<V: Vars>(vars: &V) -> Vec<PathBuf> {
    config_dirs_from_vars_and_home(vars, std::env::var_os("HOME"))
}

#[must_use]
pub fn config_dir() -> PathBuf {
    config_dir_from_vars(&EnvironmentVars)
}

#[derive(Debug, Eq, PartialEq)]
pub struct ThemeConfig {
    // This is rather bare for now, will be expanded with config file
    location: PathBuf,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            location: config_dir().join("theme.yml"),
        }
    }
}

trait FromOverride<T>: Sized {
    fn from(value: T, default: Self) -> Self;
}

impl<S, T> FromOverride<Option<S>> for Option<T>
where
    T: FromOverride<S> + Default,
{
    fn from(value: Option<S>, default: Option<T>) -> Option<T> {
        match (value, default) {
            (Some(value), Some(default)) => Some(FromOverride::from(value, default)),
            (Some(value), None) => Some(FromOverride::from(value, T::default())),
            (None, Some(default)) => Some(default),
            (None, None) => None,
        }
    }
}

#[rustfmt::skip]
fn color_from_str(s: &str) -> Option<Color> {
    use Color::{Black, Blue, Cyan, DarkGray, Default, Fixed, Green, LightBlue, LightCyan, LightGray, LightGreen, LightMagenta, LightPurple, LightRed, LightYellow, Magenta, Purple, Red, Rgb, White, Yellow};
    match s {
        // nothing
        "" | "none"    | "None"         => None,

        // hardcoded colors
        "default"      | "Default"      => Some(Default),
        "black"        | "Black"        => Some(Black),
        "darkgray"     | "DarkGray"     => Some(DarkGray),
        "red"          | "Red"          => Some(Red),
        "lightred"     | "LightRed"     => Some(LightRed),
        "green"        | "Green"        => Some(Green),
        "lightgreen"   | "LightGreen"   => Some(LightGreen),
        "yellow"       | "Yellow"       => Some(Yellow),
        "lightyellow"  | "LightYellow"  => Some(LightYellow),
        "blue"         | "Blue"         => Some(Blue),
        "lightblue"    | "LightBlue"    => Some(LightBlue),
        "purple"       | "Purple"       => Some(Purple),
        "lightpurple"  | "LightPurple"  => Some(LightPurple),
        "magenta"      | "Magenta"      => Some(Magenta),
        "lightmagenta" | "LightMagenta" => Some(LightMagenta),
        "cyan"         | "Cyan"         => Some(Cyan),
        "lightcyan"    | "LightCyan"    => Some(LightCyan),
        "white"        | "White"        => Some(White),
        "lightgray"    | "LightGray"    => Some(LightGray),

        // 16-colour palette aliases. These are fixed ANSI palette indexes so
        // theme files can use names such as `brightyellow` instead of `11`.
        "base00" | "Base00" | "color0"  | "Color0"  => Some(Fixed(0)),
        "base01" | "Base01" | "color1"  | "Color1"  => Some(Fixed(1)),
        "base02" | "Base02" | "color2"  | "Color2"  => Some(Fixed(2)),
        "base03" | "Base03" | "color3"  | "Color3"  => Some(Fixed(3)),
        "base04" | "Base04" | "color4"  | "Color4"  => Some(Fixed(4)),
        "base05" | "Base05" | "color5"  | "Color5"  => Some(Fixed(5)),
        "base06" | "Base06" | "color6"  | "Color6"  => Some(Fixed(6)),
        "base07" | "Base07" | "color7"  | "Color7"  => Some(Fixed(7)),
        "base08" | "Base08" | "color8"  | "Color8"  | "brightblack"  | "BrightBlack"  => Some(Fixed(8)),
        "base09" | "Base09" | "color9"  | "Color9"  | "brightred"    | "BrightRed"    => Some(Fixed(9)),
        "base0A" | "Base0A" | "base0a" | "Base0a" | "color10" | "Color10" | "brightgreen"  | "BrightGreen"  => Some(Fixed(10)),
        "base0B" | "Base0B" | "base0b" | "Base0b" | "color11" | "Color11" | "brightyellow" | "BrightYellow" => Some(Fixed(11)),
        "base0C" | "Base0C" | "base0c" | "Base0c" | "color12" | "Color12" | "brightblue"   | "BrightBlue"   => Some(Fixed(12)),
        "base0D" | "Base0D" | "base0d" | "Base0d" | "color13" | "Color13" | "brightmagenta"| "BrightMagenta"=> Some(Fixed(13)),
        "base0E" | "Base0E" | "base0e" | "Base0e" | "color14" | "Color14" | "brightcyan"   | "BrightCyan"   => Some(Fixed(14)),
        "base0F" | "Base0F" | "base0f" | "Base0f" | "color15" | "Color15" | "brightwhite"  | "BrightWhite"  => Some(Fixed(15)),

        // some other string
        s => match s.chars().collect::<Vec<_>>()[..] {
            // #rrggbb hex color
            ['#', r1, r2, g1, g2, b1, b2] => {
                let Ok(r) = u8::from_str_radix(&format!("{r1}{r2}"), 16)
                    else { return None };
                let Ok(g) = u8::from_str_radix(&format!("{g1}{g2}"), 16)
                    else { return None };
                let Ok(b) = u8::from_str_radix(&format!("{b1}{b2}"), 16)
                    else { return None };
                Some(Rgb(r, g, b))
            },
            // #rgb shorthand hex color
            ['#', r, g, b] => {
                let Ok(r) = u8::from_str_radix(&format!("{r}{r}"), 16)
                    else { return None };
                let Ok(g) = u8::from_str_radix(&format!("{g}{g}"), 16)
                    else { return None };
                let Ok(b) = u8::from_str_radix(&format!("{b}{b}"), 16)
                    else { return None };
                Some(Rgb(r, g, b))
            },
            // 0-255 color code (1-3 digits)
            [c1] => {
                let Ok(c) = str::parse::<u8>(&format!("{c1}"))
                    else { return None };
                Some(Fixed(c))
            },
            [c1, c2] => {
                let Ok(c) = str::parse::<u8>(&format!("{c1}{c2}"))
                    else { return None };
                Some(Fixed(c))
            },
            [c1, c2, c3] => {
                let Ok(c) = str::parse::<u8>(&format!("{c1}{c2}{c3}"))
                    else { return None };
                Some(Fixed(c))
            },
            // unknown format
            _ => None,
        }
    }
}

#[rustfmt::skip]
fn deserialize_color<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
where D: Deserializer<'de> {
    Ok(color_from_str(&String::deserialize(deserializer)?))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct StyleOverride {
    /// The style's foreground color, if it has one.
    #[serde(alias = "fg", deserialize_with = "deserialize_color", default)]
    pub foreground: Option<Color>,

    /// The style's background color, if it has one.
    #[serde(alias = "bg", deserialize_with = "deserialize_color", default)]
    pub background: Option<Color>,

    /// Whether this style is bold.
    #[serde(alias = "bold")]
    pub is_bold: Option<bool>,

    /// Whether this style is dimmed.
    #[serde(alias = "dimmed")]
    pub is_dimmed: Option<bool>,

    /// Whether this style is italic.
    #[serde(alias = "italic")]
    pub is_italic: Option<bool>,

    /// Whether this style is underlined.
    #[serde(alias = "underline")]
    pub is_underline: Option<bool>,

    /// Whether this style is blinking.
    #[serde(alias = "blink")]
    pub is_blink: Option<bool>,

    /// Whether this style has reverse colors.
    #[serde(alias = "reverse")]
    pub is_reverse: Option<bool>,

    /// Whether this style is hidden.
    #[serde(alias = "hidden")]
    pub is_hidden: Option<bool>,

    /// Whether this style is struckthrough.
    #[serde(alias = "strikethrough")]
    pub is_strikethrough: Option<bool>,

    /// Wether this style is always displayed starting with a reset code to clear any remaining style artifacts
    #[serde(alias = "prefix_reset")]
    pub prefix_with_reset: Option<bool>,
}

impl FromOverride<StyleOverride> for Style {
    fn from(value: StyleOverride, default: Self) -> Self {
        let mut style = default;
        if value.foreground.is_some() {
            style.foreground = value.foreground;
        }
        if value.background.is_some() {
            style.background = value.background;
        }
        if let Some(bold) = value.is_bold {
            style.is_bold = bold;
        }
        if let Some(dimmed) = value.is_dimmed {
            style.is_dimmed = dimmed;
        }
        if let Some(italic) = value.is_italic {
            style.is_italic = italic;
        }
        if let Some(underline) = value.is_underline {
            style.is_underline = underline;
        }
        if let Some(blink) = value.is_blink {
            style.is_blink = blink;
        }
        if let Some(reverse) = value.is_reverse {
            style.is_reverse = reverse;
        }
        if let Some(hidden) = value.is_hidden {
            style.is_hidden = hidden;
        }
        if let Some(strikethrough) = value.is_strikethrough {
            style.is_strikethrough = strikethrough;
        }
        if let Some(reset) = value.prefix_with_reset {
            style.prefix_with_reset = reset;
        }
        style
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct IconStyleOverride {
    pub glyph: Option<char>,
    pub style: Option<StyleOverride>,
}

impl FromOverride<char> for char {
    fn from(value: char, _default: char) -> char {
        value
    }
}

impl FromOverride<IconStyleOverride> for IconStyle {
    fn from(value: IconStyleOverride, default: Self) -> Self {
        IconStyle {
            glyph: FromOverride::from(value.glyph, default.glyph),
            style: FromOverride::from(value.style, default.style),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct FileNameStyleOverride {
    pub icon: Option<IconStyleOverride>,
    pub filename: Option<StyleOverride>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct IconThemeOverride {
    pub folder: Option<IconStyleOverride>,
    pub empty_folder: Option<IconStyleOverride>,
    pub file: Option<IconStyleOverride>,
    pub unknown_file: Option<IconStyleOverride>,
}

impl FromOverride<IconThemeOverride> for IconTheme {
    fn from(value: IconThemeOverride, default: Self) -> Self {
        IconTheme {
            folder: FromOverride::from(value.folder, default.folder),
            empty_folder: FromOverride::from(value.empty_folder, default.empty_folder),
            file: FromOverride::from(value.file, default.file),
            unknown_file: FromOverride::from(value.unknown_file, default.unknown_file),
        }
    }
}

impl FromOverride<FileNameStyleOverride> for FileNameStyle {
    fn from(value: FileNameStyleOverride, default: Self) -> Self {
        FileNameStyle {
            icon: FromOverride::from(value.icon, default.icon),
            filename: FromOverride::from(value.filename, default.filename),
        }
    }
}

impl<R, S, T> FromOverride<HashMap<R, S>> for HashMap<R, T>
where
    T: FromOverride<S>,
    R: Clone + Eq + std::hash::Hash,
    T: Clone + Eq + Default,
{
    fn from(value: HashMap<R, S>, default: HashMap<R, T>) -> HashMap<R, T> {
        let mut result = default.clone();
        for (r, s) in value {
            let t = match default.get(&r) {
                Some(t) => t.clone(),
                None => T::default(),
            };
            result.insert(r, FromOverride::from(s, t));
        }
        result
    }
}

#[rustfmt::skip]
#[derive(Clone, Eq, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileKindsOverride {
    pub normal: Option<StyleOverride>,        // fi
    pub directory: Option<StyleOverride>,     // di
    pub symlink: Option<StyleOverride>,       // ln
    pub pipe: Option<StyleOverride>,          // pi
    pub block_device: Option<StyleOverride>,  // bd
    pub char_device: Option<StyleOverride>,   // cd
    pub socket: Option<StyleOverride>,        // so
    pub special: Option<StyleOverride>,       // sp
    pub executable: Option<StyleOverride>,    // ex
    pub mount_point: Option<StyleOverride>,   // mp
}

impl FromOverride<FileKindsOverride> for FileKinds {
    fn from(value: FileKindsOverride, default: Self) -> Self {
        FileKinds {
            normal: FromOverride::from(value.normal, default.normal),
            directory: FromOverride::from(value.directory, default.directory),
            symlink: FromOverride::from(value.symlink, default.symlink),
            pipe: FromOverride::from(value.pipe, default.pipe),
            block_device: FromOverride::from(value.block_device, default.block_device),
            char_device: FromOverride::from(value.char_device, default.char_device),
            socket: FromOverride::from(value.socket, default.socket),
            special: FromOverride::from(value.special, default.special),
            executable: FromOverride::from(value.executable, default.executable),
            mount_point: FromOverride::from(value.mount_point, default.mount_point),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy,Eq, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PermissionsOverride {
    pub user_read:         Option<StyleOverride>,  // ur
    pub user_write:         Option<StyleOverride>,  // uw
    pub user_execute_file:  Option<StyleOverride>,  // ux
    pub user_execute_other: Option<StyleOverride>,  // ue

    pub group_read:    Option<StyleOverride>,       // gr
    pub group_write:   Option<StyleOverride>,       // gw
    pub group_execute: Option<StyleOverride>,       // gx

    pub other_read:    Option<StyleOverride>,       // tr
    pub other_write:   Option<StyleOverride>,       // tw
    pub other_execute: Option<StyleOverride>,       // tx

    pub special_user_file: Option<StyleOverride>,   // su
    pub special_other:     Option<StyleOverride>,   // sf

    pub attribute: Option<StyleOverride>,           // xa
}

impl FromOverride<PermissionsOverride> for Permissions {
    fn from(value: PermissionsOverride, default: Self) -> Self {
        Permissions {
            user_read: FromOverride::from(value.user_read, default.user_read),
            user_write: FromOverride::from(value.user_write, default.user_write),
            user_execute_file: FromOverride::from(
                value.user_execute_file,
                default.user_execute_file,
            ),
            user_execute_other: FromOverride::from(
                value.user_execute_other,
                default.user_execute_other,
            ),
            group_read: FromOverride::from(value.group_read, default.group_read),
            group_write: FromOverride::from(value.group_write, default.group_write),
            group_execute: FromOverride::from(value.group_execute, default.group_execute),
            other_read: FromOverride::from(value.other_read, default.other_read),
            other_write: FromOverride::from(value.other_write, default.other_write),
            other_execute: FromOverride::from(value.other_execute, default.other_execute),
            special_user_file: FromOverride::from(
                value.special_user_file,
                default.special_user_file,
            ),
            special_other: FromOverride::from(value.special_other, default.special_other),
            attribute: FromOverride::from(value.attribute, default.attribute),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Eq, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SizeOverride {
    pub major: Option<StyleOverride>,        // df
    pub minor: Option<StyleOverride>,        // ds

    pub number_byte: Option<StyleOverride>,  // sn nb
    pub number_kilo: Option<StyleOverride>,  // sn nk
    pub number_mega: Option<StyleOverride>,  // sn nm
    pub number_giga: Option<StyleOverride>,  // sn ng
    pub number_huge: Option<StyleOverride>,  // sn nt

    pub unit_byte: Option<StyleOverride>,    // sb ub
    pub unit_kilo: Option<StyleOverride>,    // sb uk
    pub unit_mega: Option<StyleOverride>,    // sb um
    pub unit_giga: Option<StyleOverride>,    // sb ug
    pub unit_huge: Option<StyleOverride>,    // sb ut
}

impl FromOverride<SizeOverride> for Size {
    fn from(value: SizeOverride, default: Self) -> Self {
        Size {
            major: FromOverride::from(value.major, default.major),
            minor: FromOverride::from(value.minor, default.minor),
            number_byte: FromOverride::from(value.number_byte, default.number_byte),
            number_kilo: FromOverride::from(value.number_kilo, default.number_kilo),
            number_mega: FromOverride::from(value.number_mega, default.number_mega),
            number_giga: FromOverride::from(value.number_giga, default.number_giga),
            number_huge: FromOverride::from(value.number_huge, default.number_huge),
            unit_byte: FromOverride::from(value.unit_byte, default.unit_byte),
            unit_kilo: FromOverride::from(value.unit_kilo, default.unit_kilo),
            unit_mega: FromOverride::from(value.unit_mega, default.unit_mega),
            unit_giga: FromOverride::from(value.unit_giga, default.unit_giga),
            unit_huge: FromOverride::from(value.unit_huge, default.unit_huge),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug,Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersOverride {
    pub user_you: Option<StyleOverride>,           // uu
    pub user_root: Option<StyleOverride>,          // uR
    pub user_other: Option<StyleOverride>,         // un
    pub group_yours: Option<StyleOverride>,        // gu
    pub group_other: Option<StyleOverride>,        // gn
    pub group_root: Option<StyleOverride>,         // gR
}

impl FromOverride<UsersOverride> for Users {
    fn from(value: UsersOverride, default: Self) -> Self {
        Users {
            user_you: FromOverride::from(value.user_you, default.user_you),
            user_root: FromOverride::from(value.user_root, default.user_root),
            user_other: FromOverride::from(value.user_other, default.user_other),
            group_yours: FromOverride::from(value.group_yours, default.group_yours),
            group_other: FromOverride::from(value.group_other, default.group_other),
            group_root: FromOverride::from(value.group_root, default.group_root),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct LinksOverride {
    pub normal: Option<StyleOverride>,           // lc
    pub multi_link_file: Option<StyleOverride>,  // lm
}

impl FromOverride<LinksOverride> for Links {
    fn from(value: LinksOverride, default: Self) -> Self {
        Links {
            normal: FromOverride::from(value.normal, default.normal),
            multi_link_file: FromOverride::from(value.multi_link_file, default.multi_link_file),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug,Eq, PartialEq, Serialize, Deserialize)]
pub struct GitOverride {
    pub staged: Option<StyleOverride>,
    pub new: Option<StyleOverride>,         // ga
    pub modified: Option<StyleOverride>,    // gm
    pub deleted: Option<StyleOverride>,     // gd
    pub renamed: Option<StyleOverride>,     // gv
    pub typechange: Option<StyleOverride>,  // gt
    pub ignored: Option<StyleOverride>,     // gi
    pub conflicted: Option<StyleOverride>,  // gc
}

impl FromOverride<GitOverride> for Git {
    fn from(value: GitOverride, default: Self) -> Self {
        Git {
            staged: FromOverride::from(value.staged, default.staged),
            new: FromOverride::from(value.new, default.new),
            modified: FromOverride::from(value.modified, default.modified),
            deleted: FromOverride::from(value.deleted, default.deleted),
            renamed: FromOverride::from(value.renamed, default.renamed),
            typechange: FromOverride::from(value.typechange, default.typechange),
            ignored: FromOverride::from(value.ignored, default.ignored),
            conflicted: FromOverride::from(value.conflicted, default.conflicted),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitMarkersOverride {
    pub not_modified: Option<char>,
    pub staged: Option<char>,
    pub new: Option<char>,
    pub modified: Option<char>,
    pub deleted: Option<char>,
    pub renamed: Option<char>,
    pub typechange: Option<char>,
    pub ignored: Option<char>,
    pub conflicted: Option<char>,
}

impl FromOverride<GitMarkersOverride> for GitMarkers {
    fn from(value: GitMarkersOverride, default: Self) -> Self {
        GitMarkers {
            not_modified: FromOverride::from(value.not_modified, default.not_modified),
            staged: FromOverride::from(value.staged, default.staged),
            new: FromOverride::from(value.new, default.new),
            modified: FromOverride::from(value.modified, default.modified),
            deleted: FromOverride::from(value.deleted, default.deleted),
            renamed: FromOverride::from(value.renamed, default.renamed),
            typechange: FromOverride::from(value.typechange, default.typechange),
            ignored: FromOverride::from(value.ignored, default.ignored),
            conflicted: FromOverride::from(value.conflicted, default.conflicted),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GitRepoOverride {
    pub branch_main: Option<StyleOverride>,  //Gm
    pub branch_other: Option<StyleOverride>, //Go
    pub git_clean: Option<StyleOverride>,    //Gc
    pub git_dirty: Option<StyleOverride>,    //Gd
}

impl FromOverride<GitRepoOverride> for GitRepo {
    fn from(value: GitRepoOverride, default: Self) -> Self {
        GitRepo {
            branch_main: FromOverride::from(value.branch_main, default.branch_main),
            branch_other: FromOverride::from(value.branch_other, default.branch_other),
            git_clean: FromOverride::from(value.git_clean, default.git_clean),
            git_dirty: FromOverride::from(value.git_dirty, default.git_dirty),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct SELinuxContextOverride {
    pub colon: Option<StyleOverride>,
    pub user: Option<StyleOverride>,  // Su
    pub role: Option<StyleOverride>,  // Sr
    pub typ: Option<StyleOverride>,   // St
    pub range: Option<StyleOverride>, // Sl
}

impl FromOverride<SELinuxContextOverride> for SELinuxContext {
    fn from(value: SELinuxContextOverride, default: Self) -> Self {
        SELinuxContext {
            colon: FromOverride::from(value.colon, default.colon),
            user: FromOverride::from(value.user, default.user),
            role: FromOverride::from(value.role, default.role),
            typ: FromOverride::from(value.typ, default.typ),
            range: FromOverride::from(value.range, default.range),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Eq, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityContextOverride {
    pub none:    Option<StyleOverride>, // Sn
    pub selinux: Option<SELinuxContextOverride>,
}

impl FromOverride<SecurityContextOverride> for SecurityContext {
    fn from(value: SecurityContextOverride, default: Self) -> Self {
        SecurityContext {
            none: FromOverride::from(value.none, default.none),
            selinux: FromOverride::from(value.selinux, default.selinux),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct FileTypeOverride {
    pub image: Option<StyleOverride>,       // im - image file
    pub video: Option<StyleOverride>,       // vi - video file
    pub music: Option<StyleOverride>,       // mu - lossy music
    pub lossless: Option<StyleOverride>,    // lo - lossless music
    pub crypto: Option<StyleOverride>,      // cr - related to cryptography
    pub document: Option<StyleOverride>,    // do - document file
    pub compressed: Option<StyleOverride>,  // co - compressed file
    pub temp: Option<StyleOverride>,        // tm - temporary file
    pub compiled: Option<StyleOverride>,    // cm - compilation artifact
    pub build: Option<StyleOverride>,       // bu - file that is used to build a project
    pub source: Option<StyleOverride>,      // sc - source code
}

impl FromOverride<FileTypeOverride> for FileType {
    fn from(value: FileTypeOverride, default: Self) -> Self {
        FileType {
            image: FromOverride::from(value.image, default.image),
            video: FromOverride::from(value.video, default.video),
            music: FromOverride::from(value.music, default.music),
            lossless: FromOverride::from(value.lossless, default.lossless),
            crypto: FromOverride::from(value.crypto, default.crypto),
            document: FromOverride::from(value.document, default.document),
            compressed: FromOverride::from(value.compressed, default.compressed),
            temp: FromOverride::from(value.temp, default.temp),
            compiled: FromOverride::from(value.compiled, default.compiled),
            build: FromOverride::from(value.build, default.build),
            source: FromOverride::from(value.source, default.source),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UiStylesOverride {
    pub colourful: Option<bool>,

    pub filekinds:        Option<FileKindsOverride>,
    pub perms:            Option<PermissionsOverride>,
    pub size:             Option<SizeOverride>,
    pub users:            Option<UsersOverride>,
    pub links:            Option<LinksOverride>,
    pub git:              Option<GitOverride>,
    pub git_markers:      Option<GitMarkersOverride>,
    pub git_repo:         Option<GitRepoOverride>,
    pub security_context: Option<SecurityContextOverride>,
    pub file_type:        Option<FileTypeOverride>,

    pub punctuation:  Option<StyleOverride>,          // xx
    pub date:         Option<StyleOverride>,          // da
    pub inode:        Option<StyleOverride>,          // in
    pub blocks:       Option<StyleOverride>,          // bl
    pub header:       Option<StyleOverride>,          // hd
    pub octal:        Option<StyleOverride>,          // oc
    pub flags:        Option<StyleOverride>,          // ff

    pub symlink_path:         Option<StyleOverride>,  // lp
    pub control_char:         Option<StyleOverride>,  // cc
    pub broken_symlink:       Option<StyleOverride>,  // or
    pub broken_path_overlay:  Option<StyleOverride>,  // bO

    pub icons: Option<IconThemeOverride>,
    pub filenames: Option<HashMap<String, FileNameStyleOverride>>,
    pub extensions: Option<HashMap<String, FileNameStyleOverride>>,
}

impl FromOverride<UiStylesOverride> for UiStyles {
    fn from(value: UiStylesOverride, default: Self) -> Self {
        UiStyles {
            colourful: value.colourful,

            filekinds: FromOverride::from(value.filekinds, default.filekinds),
            perms: FromOverride::from(value.perms, default.perms),
            size: FromOverride::from(value.size, default.size),
            users: FromOverride::from(value.users, default.users),
            links: FromOverride::from(value.links, default.links),
            git: FromOverride::from(value.git, default.git),
            git_markers: FromOverride::from(value.git_markers, default.git_markers),
            git_repo: FromOverride::from(value.git_repo, default.git_repo),
            security_context: FromOverride::from(value.security_context, default.security_context),
            file_type: FromOverride::from(value.file_type, default.file_type),

            punctuation: FromOverride::from(value.punctuation, default.punctuation),
            date: FromOverride::from(value.date, default.date),
            inode: FromOverride::from(value.inode, default.inode),
            blocks: FromOverride::from(value.blocks, default.blocks),
            header: FromOverride::from(value.header, default.header),
            octal: FromOverride::from(value.octal, default.octal),
            flags: FromOverride::from(value.flags, default.flags),

            symlink_path: FromOverride::from(value.symlink_path, default.symlink_path),
            control_char: FromOverride::from(value.control_char, default.control_char),
            broken_symlink: FromOverride::from(value.broken_symlink, default.broken_symlink),
            broken_path_overlay: FromOverride::from(
                value.broken_path_overlay,
                default.broken_path_overlay,
            ),

            icons: FromOverride::from(value.icons, default.icons),
            filenames: FromOverride::from(value.filenames, default.filenames),
            extensions: FromOverride::from(value.extensions, default.extensions),
        }
    }
}
impl ThemeConfig {
    #[must_use]
    pub fn from_path(path: PathBuf) -> Self {
        ThemeConfig { location: path }
    }
    #[must_use]
    pub fn to_theme(&self) -> Option<UiStyles> {
        let ui_styles_override: Option<UiStylesOverride> = {
            let file = std::fs::File::open(&self.location).ok()?;
            serde_norway::from_reader(&file).ok()
        };
        FromOverride::from(ui_styles_override, Some(UiStyles::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::vars::test::MockVars;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn config_dir_prefers_eva_config_dir() {
        let mut vars = MockVars::default();
        vars.set(vars::EVA_CONFIG_DIR, &OsString::from("/tmp/eva-config"));
        vars.set(vars::EZA_CONFIG_DIR, &OsString::from("/tmp/eza-config"));
        vars.set(vars::XDG_CONFIG_HOME, &OsString::from("/tmp/xdg"));

        assert_eq!(
            config_dir_from_vars_and_home(&vars, Some(OsString::from("/home/me"))),
            PathBuf::from("/tmp/eva-config")
        );
    }

    #[test]
    fn config_dir_defaults_to_xdg_config_home_eva() {
        let mut vars = MockVars::default();
        vars.set(vars::XDG_CONFIG_HOME, &OsString::from("/tmp/xdg"));

        assert_eq!(
            config_dir_from_vars_and_home(&vars, Some(OsString::from("/home/me"))),
            PathBuf::from("/tmp/xdg").join("eva")
        );
    }

    #[test]
    fn config_dir_defaults_to_home_dot_config_eva_without_xdg() {
        let vars = MockVars::default();

        assert_eq!(
            config_dir_from_vars_and_home(&vars, Some(OsString::from("/home/me"))),
            PathBuf::from("/home/me").join(".config").join("eva")
        );
    }

    #[test]
    fn config_dirs_include_eza_legacy_fallbacks_after_eva_defaults() {
        let mut vars = MockVars::default();
        vars.set(vars::XDG_CONFIG_HOME, &OsString::from("/tmp/xdg"));
        vars.set(vars::EZA_CONFIG_DIR, &OsString::from("/tmp/eza-config"));

        assert_eq!(
            config_dirs_from_vars_and_home(&vars, Some(OsString::from("/home/me"))),
            vec![
                PathBuf::from("/tmp/xdg").join("eva"),
                PathBuf::from("/tmp/eza-config"),
                PathBuf::from("/tmp/xdg").join("eza"),
            ]
        );
    }

    #[test]
    fn empty_config_env_values_are_ignored() {
        let mut vars = MockVars::default();
        vars.set(vars::EVA_CONFIG_DIR, &OsString::new());
        vars.set(vars::EZA_CONFIG_DIR, &OsString::new());
        vars.set(vars::XDG_CONFIG_HOME, &OsString::new());

        assert_eq!(
            config_dir_from_vars_and_home(&vars, Some(OsString::from("/home/me"))),
            PathBuf::from("/home/me").join(".config").join("eva")
        );
    }

    #[test]
    fn parses_sparse_icon_defaults() {
        let input = br#"
icons:
  folder: { glyph: "F" }
  empty_folder: { glyph: "E" }
  file: { glyph: "f" }
  unknown_file: { glyph: "?" }
"#;
        let override_config: UiStylesOverride = serde_norway::from_reader(&input[..]).unwrap();
        let ui: UiStyles = FromOverride::from(override_config, UiStyles::default());
        let icons = ui.icons.unwrap();

        assert_eq!(icons.folder.unwrap().glyph, Some('F'));
        assert_eq!(icons.empty_folder.unwrap().glyph, Some('E'));
        assert_eq!(icons.file.unwrap().glyph, Some('f'));
        assert_eq!(icons.unknown_file.unwrap().glyph, Some('?'));
    }

    #[test]
    fn parse_none_color_from_string() {
        for case in &["", "none", "None"] {
            assert_eq!(color_from_str(case), None);
        }
    }

    #[test]
    fn parse_default_color_from_string() {
        for case in &["default", "Default"] {
            assert_eq!(color_from_str(case), Some(Color::Default));
        }
    }

    #[test]
    fn parse_fixed_color_from_string() {
        for case in &["black", "Black"] {
            assert_eq!(color_from_str(case), Some(Color::Black));
        }
    }

    #[test]
    fn parse_long_hex_color_from_string() {
        for case in &["#ff00ff", "#FF00FF"] {
            assert_eq!(color_from_str(case), Some(Color::Rgb(255, 0, 255)));
        }
    }

    #[test]
    fn parse_short_hex_color_from_string() {
        for case in &["#f0f", "#F0F"] {
            assert_eq!(color_from_str(case), Some(Color::Rgb(255, 0, 255)));
        }
    }

    #[test]
    fn parse_color_code_from_string() {
        for (s, c) in &[("118", 118), ("10", 10), ("01", 1), ("1", 1), ("001", 1)] {
            assert_eq!(color_from_str(s), Some(Color::Fixed(*c)));
        }
    }

    #[test]
    fn parse_base16_color_names_from_string() {
        for (s, c) in &[
            ("brightblack", 8),
            ("BrightRed", 9),
            ("brightgreen", 10),
            ("brightyellow", 11),
            ("brightblue", 12),
            ("brightmagenta", 13),
            ("brightcyan", 14),
            ("brightwhite", 15),
            ("base0B", 11),
            ("color11", 11),
        ] {
            assert_eq!(color_from_str(s), Some(Color::Fixed(*c)));
        }
    }
}
