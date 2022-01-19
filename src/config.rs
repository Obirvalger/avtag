use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct RepoSerDe {
    path: PathBuf,
    remote: Option<String>,
    max_tags: Option<usize>,
    bin_package_name: Option<String>,
    display_name: Option<String>,
    #[serde(default)]
    #[serde(with = "serde_regex")]
    tags_format_re: Option<Regex>,
    #[serde(default)]
    #[serde(with = "serde_regex")]
    ignore_tags_re: Option<Regex>,
}

impl RepoSerDe {
    fn to_repo(&self, defaults: &Defaults) -> Repo {
        Repo {
            path: optional_join(&self.path, &defaults.repos_dir),
            remote: self.remote.as_ref().unwrap_or(&defaults.remote).to_string(),
            max_tags: self.max_tags.unwrap_or(defaults.max_tags),
            bin_package_name: self.bin_package_name.to_owned(),
            display_name: self.display_name.to_owned(),
            tags_format_re: self
                .tags_format_re
                .to_owned()
                .or_else(|| defaults.tags_format_re.to_owned()),
            ignore_tags_re: self
                .ignore_tags_re
                .to_owned()
                .or_else(|| defaults.ignore_tags_re.to_owned()),
        }
    }
}

pub struct Repo {
    pub path: PathBuf,
    pub remote: String,
    pub max_tags: usize,
    bin_package_name: Option<String>,
    display_name: Option<String>,
    tags_format_re: Option<Regex>,
    ignore_tags_re: Option<Regex>,
}

impl Ord for Repo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for Repo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Repo {}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Repo {
    pub fn accept_tag<S: AsRef<str>>(&self, tag: S) -> bool {
        let tag = tag.as_ref();
        let get_tags = if let Some(re) = &self.tags_format_re { re.is_match(tag) } else { true };
        let ignore_tags =
            if let Some(re) = &self.ignore_tags_re { re.is_match(tag) } else { false };

        get_tags && !ignore_tags
    }

    fn name(&self) -> String {
        if let Some(name) = self.path.file_name() {
            name.to_string_lossy().to_string()
        } else {
            self.path.display().to_string()
        }
    }

    pub fn bin_package_name(&self) -> String {
        if let Some(name) = &self.bin_package_name {
            name.to_string()
        } else {
            self.name()
        }
    }

    pub fn display_name(&self) -> String {
        if let Some(name) = &self.display_name {
            name.to_string()
        } else {
            self.name()
        }
    }
}

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
struct Defaults {
    remote: String,
    max_tags: usize,
    repos_dir: Option<PathBuf>,
    #[serde(default)]
    #[serde(with = "serde_regex")]
    tags_format_re: Option<Regex>,
    #[serde(default)]
    #[serde(with = "serde_regex")]
    ignore_tags_re: Option<Regex>,
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

fn expand_tilde(path: &Path) -> PathBuf {
    let s = path.to_string_lossy().to_string();
    PathBuf::from(shellexpand::tilde(&s).to_string())
}

fn optional_join<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: &Option<B>) -> PathBuf {
    if let Some(base) = base {
        base.as_ref().join(path)
    } else {
        expand_tilde(path.as_ref())
    }
}

pub fn config_dir() -> PathBuf {
    let home_config_dir = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| "~/.config".to_string());
    expand_tilde(&PathBuf::from(home_config_dir)).join("avtag")
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
        config.defaults.repos_dir = config.defaults.repos_dir.map(|d| expand_tilde(&d));

        Ok(config)
    }

    pub fn repos(&self) -> Vec<Repo> {
        self.repos.iter().map(|r| r.to_repo(&self.defaults)).collect()
    }
}
