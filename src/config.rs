use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::repo::{Repo, RepoSerDe};
use crate::util;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct BinList {
    pub url: Option<String>,
    pub extract_cmd: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Defaults {
    pub remote: String,
    pub max_tags: usize,
    pub repos_dir: Option<PathBuf>,
    #[serde(default)]
    #[serde(with = "serde_regex")]
    pub tags_format_re: Option<Regex>,
    #[serde(default)]
    #[serde(with = "serde_regex")]
    pub ignore_tags_re: Option<Regex>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub bin_list: BinList,
    defaults: Defaults,
    repos: Vec<RepoSerDe>,
}

pub fn config_dir() -> PathBuf {
    let home_config_dir = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| "~/.config".to_string());
    util::expand_tilde(&PathBuf::from(home_config_dir)).join("avtag")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

impl Config {
    pub fn new(config: Option<&str>) -> Result<Config> {
        let config_path =
            if let Some(config) = config { PathBuf::from(config) } else { config_path() };
        let config_str = &fs::read_to_string(&config_path)?;

        let mut config: Config = toml::from_str(config_str)?;
        config.defaults.repos_dir = config.defaults.repos_dir.map(|d| util::expand_tilde(&d));

        Ok(config)
    }

    pub fn repos(&self) -> Vec<Repo> {
        self.repos.iter().map(|r| r.to_repo(&self.defaults)).collect()
    }
}
