use std::cmp::Ordering;
use std::path::PathBuf;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::Defaults;
use crate::util;

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
    pub fn to_repo(&self, defaults: &Defaults) -> Repo {
        Repo {
            path: util::optional_join(&self.path, &defaults.repos_dir),
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
