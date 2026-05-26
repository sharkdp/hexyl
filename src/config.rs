use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use etcetera::BaseStrategy;

pub fn config_file() -> PathBuf {
    env::var("HEXYL_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_config_file())
}

fn default_config_file() -> PathBuf {
    etcetera::choose_base_strategy()
        .map(|basedirs| basedirs.config_dir().join("hexyl").join("config"))
        .unwrap_or_else(|_| PathBuf::from("config"))
}

pub fn get_args_from_config_file() -> Result<Vec<OsString>, shell_words::ParseError> {
    let config = fs::read_to_string(config_file()).unwrap_or_default();
    get_args_from_str(&config)
}

pub fn get_args_from_env_opts_var() -> Option<Result<Vec<OsString>, shell_words::ParseError>> {
    env::var("HEXYL_OPTS").ok().map(|s| get_args_from_str(&s))
}

pub fn get_args_from_str(content: &str) -> Result<Vec<OsString>, shell_words::ParseError> {
    let args_per_line = content
        .split('\n')
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with('#'))
        .map(shell_words::split)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(args_per_line.iter().flatten().map(OsString::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config() {
        assert!(get_args_from_str("").unwrap().is_empty());
    }

    #[test]
    fn ignores_comments_and_blank_lines() {
        assert_eq!(
            vec!["--color=never", "--panels=1"],
            get_args_from_str("# default options\n\n--color=never\n\n--panels=1\n").unwrap()
        );
    }

    #[test]
    fn supports_quoted_values() {
        assert_eq!(
            vec!["--base", "hexadecimal"],
            get_args_from_str("--base \"hexadecimal\"").unwrap()
        );
    }
}
