use clap::ArgMatches;

// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::options::parser::ShowWhen;
use crate::options::{vars, Vars};
use crate::output::color_scale::ColorScaleOptions;
use crate::theme::{Definitions, Options, UseColours};
use super::config::{ThemeConfig, config_dirs_from_vars};

impl Options {
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        let use_colours = UseColours::deduce(matches, vars);
        let colour_scale = ColorScaleOptions::deduce(matches, vars);
        let theme_config = ThemeConfig::deduce(vars);

        let definitions = if use_colours == UseColours::Never {
            Definitions::default()
        } else {
            Definitions::deduce(vars)
        };

        Self {
            use_colours,
            colour_scale,
            definitions,
            theme_config,
        }
    }
}

impl ThemeConfig {
    fn deduce<V: Vars>(vars: &V) -> Option<Self> {
        for path in config_dirs_from_vars(vars) {
            let theme = path.join("theme.yml");
            if theme.exists() {
                return Some(ThemeConfig::from_path(theme));
            }

            let theme = path.join("theme.yaml");
            if theme.exists() {
                return Some(ThemeConfig::from_path(theme));
            }
        }

        None
    }
}

impl UseColours {
    fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        let default_value = match vars.get(vars::NO_COLOR) {
            Some(_) => Self::Never,
            None => Self::Automatic,
        };

        match matches.get_one("color").unwrap() {
            ShowWhen::Auto => default_value,
            ShowWhen::Always => Self::Always,
            ShowWhen::Never => Self::Never,
        }
    }
}

impl Definitions {
    fn deduce<V: Vars>(vars: &V) -> Self {
        let ls = vars
            .get(vars::LS_COLORS)
            .map(|e| e.to_string_lossy().to_string());
        let exa = vars
            .get_with_fallbacks(&[vars::EVA_COLORS, vars::EZA_COLORS, vars::EXA_COLORS])
            .map(|e| e.to_string_lossy().to_string());
        Self { ls, exa }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{parser::test::mock_cli, vars::test::MockVars};
    use std::ffi::OsString;

    #[test]
    fn deduce_definitions() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            Definitions::deduce(&vars),
            Definitions {
                ls: None,
                exa: None,
            }
        );
    }

    #[test]
    fn deduce_definitions_colors() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(vars::LS_COLORS, &OsString::from("di=34"));
        vars.set(vars::EVA_COLORS, &OsString::from("uR=1;34"));

        assert_eq!(
            Definitions::deduce(&vars),
            Definitions {
                ls: Some("di=34".to_string()),
                exa: Some("uR=1;34".to_string()),
            }
        );
    }

    #[test]
    fn deduce_definitions_colors_uses_eza_fallback() {
        let mut vars = MockVars::default();
        vars.set(vars::EZA_COLORS, &OsString::from("uR=1;34"));

        assert_eq!(
            Definitions::deduce(&vars),
            Definitions {
                ls: None,
                exa: Some("uR=1;34".to_string()),
            }
        );
    }

    #[test]
    fn deduce_use_colors_no_color_env() {
        let vars = MockVars {
            no_colors: OsString::from("1"),
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec![""]), &vars),
            UseColours::Never
        );
    }

    #[test]
    fn deduce_use_colors_no_color_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec!["--color", "never"]), &vars),
            UseColours::Never
        );
    }

    #[test]
    fn deduce_use_colors_always() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec!["--color", "always"]), &vars),
            UseColours::Always
        );
    }

    #[test]
    fn deduce_use_colors_auto() {
        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            UseColours::deduce(&mock_cli(vec!["--color", "auto"]), &vars),
            UseColours::Automatic
        );
    }
}
