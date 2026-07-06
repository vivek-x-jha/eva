// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use std::ffi::OsString;

// General variables

/// Environment variable used to colour files, both by their filesystem type
/// (symlink, socket, directory) and their file name or extension (image,
/// video, archive);
pub static LS_COLORS: &str = "LS_COLORS";

/// Environment variable used to override the width of the terminal, in
/// characters.
pub static COLUMNS: &str = "COLUMNS";

/// Environment variable used to datetime format.
pub static TIME_STYLE: &str = "TIME_STYLE";

/// Environment variable used to disable colors.
/// See: <https://no-color.org/>
pub static NO_COLOR: &str = "NO_COLOR";

// eva/eza/exa-specific variables

/// Environment variable used to colour eva’s interface when colours are
/// enabled. This includes all the colours that `LS_COLORS` would recognise,
/// overriding them if necessary. It can also contain eva-specific codes.
pub static EVA_COLORS: &str = "EVA_COLORS";
pub static EZA_COLORS: &str = "EZA_COLORS";
pub static EXA_COLORS: &str = "EXA_COLORS";

/// Environment variable used to choose the eva configuration directory.
pub static EVA_CONFIG_DIR: &str = "EVA_CONFIG_DIR";
pub static EZA_CONFIG_DIR: &str = "EZA_CONFIG_DIR";

/// XDG base-directory variable used for the default eva configuration directory.
pub static XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";

/// Environment variable used to switch on strict argument checking, such as
/// complaining if an argument was specified twice, or if two conflict.
/// This is meant to be so you don’t accidentally introduce the wrong
/// behaviour in a script, rather than for general command-line use.
/// Any non-empty value will turn strict mode on.
pub static EVA_STRICT: &str = "EVA_STRICT";
pub static EZA_STRICT: &str = "EZA_STRICT";
pub static EXA_STRICT: &str = "EXA_STRICT";

/// Environment variable used to make eva print out debugging information as
/// it runs. Any non-empty value will turn debug mode on.
pub static EVA_DEBUG: &str = "EVA_DEBUG";
pub static EZA_DEBUG: &str = "EZA_DEBUG";
pub static EXA_DEBUG: &str = "EXA_DEBUG";

/// Environment variable used to limit the grid-details view
/// (`--grid --long`) so it’s only activated if there’s at least the given
/// number of rows of output.
pub static EVA_GRID_ROWS: &str = "EVA_GRID_ROWS";
pub static EZA_GRID_ROWS: &str = "EZA_GRID_ROWS";
pub static EXA_GRID_ROWS: &str = "EXA_GRID_ROWS";

/// Environment variable used to specify how many spaces to print between an
/// icon and its file name. Different terminals display icons differently,
/// with 1 space bringing them too close together or 2 spaces putting them too
/// far apart, so this may be necessary depending on how they are shown.
pub static EVA_ICON_SPACING: &str = "EVA_ICON_SPACING";
pub static EZA_ICON_SPACING: &str = "EZA_ICON_SPACING";
pub static EXA_ICON_SPACING: &str = "EXA_ICON_SPACING";

pub static EVA_OVERRIDE_GIT: &str = "EVA_OVERRIDE_GIT";
pub static EZA_OVERRIDE_GIT: &str = "EZA_OVERRIDE_GIT";
pub static EXA_OVERRIDE_GIT: &str = "EXA_OVERRIDE_GIT";

/// Environment variable used to set the minimum luminance in `color_scale`. Its value
/// can be between -100 and 100.
pub static EVA_MIN_LUMINANCE: &str = "EVA_MIN_LUMINANCE";
pub static EZA_MIN_LUMINANCE: &str = "EZA_MIN_LUMINANCE";
pub static EXA_MIN_LUMINANCE: &str = "EXA_MIN_LUMINANCE";

/// Environment variable used to automate the same behavior as `--icons=auto` if set.
/// Any explicit use of `--icons=WHEN` overrides this behavior.
pub static EVA_ICONS_AUTO: &str = "EVA_ICONS_AUTO";
pub static EZA_ICONS_AUTO: &str = "EZA_ICONS_AUTO";

pub static EVA_STDIN_SEPARATOR: &str = "EVA_STDIN_SEPARATOR";
pub static EZA_STDIN_SEPARATOR: &str = "EZA_STDIN_SEPARATOR";

/// Environment variable used to choose how windows attributes are displayed.
/// Short will display a single character for each set attribute, long will
/// display a comma separated list of descriptions.
pub static EVA_WINDOWS_ATTRIBUTES: &str = "EVA_WINDOWS_ATTRIBUTES";
pub static EZA_WINDOWS_ATTRIBUTES: &str = "EZA_WINDOWS_ATTRIBUTES";

/// Mockable wrapper for `std::env::var_os`.
pub trait Vars {
    fn get(&self, name: &'static str) -> Option<OsString>;

    /// Get the variable `name` and if not set get the variable `fallback`.
    fn get_with_fallback(&self, name: &'static str, fallback: &'static str) -> Option<OsString> {
        self.get_with_fallbacks(&[name, fallback])
    }

    /// Get the first set variable in priority order.
    fn get_with_fallbacks(&self, names: &[&'static str]) -> Option<OsString> {
        names.iter().find_map(|&name| self.get(name))
    }

    /// Get the source of the value.  If the variable `name` is set return
    /// `Some(name)` else if the variable `fallback` is set return
    /// `Some(fallback)` else `None`.
    fn source(&self, name: &'static str, fallback: &'static str) -> Option<&'static str> {
        self.source_with_fallbacks(&[name, fallback])
    }

    /// Get the source of the first set variable in priority order.
    fn source_with_fallbacks(&self, names: &[&'static str]) -> Option<&'static str> {
        names.iter().copied().find(|&name| self.get(name).is_some())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::collections::HashMap;

    // Test impl that just returns the value it has.
    impl Vars for Option<OsString> {
        fn get(&self, _name: &'static str) -> Option<OsString> {
            self.clone()
        }
    }

    #[derive(Default)]
    pub struct MockVars {
        pub overrides: HashMap<&'static str, OsString>,
        pub columns: OsString,
        pub colors: OsString,
        pub config_dir: OsString,
        pub xdg_config_home: OsString,
        pub no_colors: OsString,
        pub strict: OsString,
        pub debug: OsString,
        pub grid_rows: OsString,
        pub icon_spacing: OsString,
        pub luminance: OsString,
        pub icons: OsString,
        pub time: OsString,
    }

    impl Vars for MockVars {
        fn get(&self, name: &'static str) -> Option<OsString> {
            if let Some(value) = self.overrides.get(name) {
                return Some(value.clone());
            }

            match name {
                "EVA_STRICT" | "EZA_STRICT" | "EXA_STRICT" if !self.strict.is_empty() => {
                    Some(self.strict.clone())
                }
                "EVA_COLORS" | "EZA_COLORS" | "LS_COLORS" | "EXA_COLORS"
                    if !self.colors.is_empty() =>
                {
                    Some(self.colors.clone())
                }
                "EVA_CONFIG_DIR" | "EZA_CONFIG_DIR" if !self.config_dir.is_empty() => {
                    Some(self.config_dir.clone())
                }
                "XDG_CONFIG_HOME" if !self.xdg_config_home.is_empty() => {
                    Some(self.xdg_config_home.clone())
                }
                "EVA_DEBUG" | "EZA_DEBUG" | "EXA_DEBUG" if !self.debug.is_empty() => {
                    Some(self.debug.clone())
                }
                "EVA_GRID_ROWS" | "EZA_GRID_ROWS" | "EXA_GRID_ROWS"
                    if !self.grid_rows.is_empty() =>
                {
                    Some(self.grid_rows.clone())
                }
                "EVA_ICON_SPACING" | "EZA_ICON_SPACING" | "EXA_ICON_SPACING"
                    if !self.icon_spacing.is_empty() =>
                {
                    Some(self.icon_spacing.clone())
                }
                "EVA_MIN_LUMINANCE" | "EZA_MIN_LUMINANCE" | "EXA_MIN_LUMINANCE"
                    if !self.luminance.is_empty() =>
                {
                    Some(self.luminance.clone())
                }
                "EVA_ICONS_AUTO" | "EZA_ICONS_AUTO" if !self.icons.is_empty() => {
                    Some(self.icons.clone())
                }
                "COLUMNS" if !self.columns.is_empty() => Some(self.columns.clone()),
                "NO_COLOR" if !self.no_colors.is_empty() => Some(self.no_colors.clone()),
                "TIME_STYLE" if !self.time.is_empty() => Some(self.time.clone()),
                _ => None,
            }
        }
    }

    impl MockVars {
        pub fn set(&mut self, var: &'static str, value: &OsString) {
            self.overrides.insert(var, value.clone());
        }
    }

    #[test]
    fn set_test() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(TIME_STYLE, &OsString::from("iso"));
        assert_eq!(vars.get(TIME_STYLE), Some(OsString::from("iso")));
    }
}
