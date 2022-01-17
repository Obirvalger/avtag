use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use cmd_lib::run_cmd;
use semver::Version;

use crate::tmpdir::TMPDIR;

pub struct BinList {
    map: HashMap<String, Version>,
}

impl BinList {
    fn download<S: AsRef<str>>(url: S) -> Result<()> {
        let tmpdir = &*TMPDIR;
        let mut body = reqwest::blocking::get(url.as_ref())?;
        let mut tmp = tempfile::Builder::new().tempfile_in(&tmpdir)?;
        body.copy_to(&mut tmp)?;

        fs::rename(tmp.path(), tmpdir.join("bin_list_raw"))?;

        Ok(())
    }

    fn extract(extract_cmd: &Option<Vec<String>>) -> Result<()> {
        let tmpdir = &*TMPDIR;
        if let Some(cmd) = extract_cmd {
            if let Some((prog, args)) = cmd.split_first() {
                run_cmd!($prog $[args] < $tmpdir/bin_list_raw > $tmpdir/bin_list)?;
            } else {
                run_cmd!(mv $tmpdir/bin_list_raw $tmpdir/bin_list)?;
            }
        } else {
            run_cmd!(mv $tmpdir/bin_list_raw $tmpdir/bin_list)?;
        }

        Ok(())
    }

    pub fn new<S: AsRef<str>>(url: S, extract_cmd: &Option<Vec<String>>) -> Result<Self> {
        let tmpdir = &*TMPDIR;
        Self::download(url)?;
        Self::extract(extract_cmd)?;

        let mut map = HashMap::new();
        let file = File::open(tmpdir.join("bin_list"))?;
        for (number, line) in BufReader::new(file).lines().enumerate() {
            let line =
                line.with_context(|| format!("bin_list: can't parse {} line from", number))?;
            let fields: Vec<&str> = line.split_ascii_whitespace().take(2).collect();
            if fields.len() < 2 {
                bail!("bin_list: can't parse {} line `{}`", number, line)
            }
            let version = if let Some((ver, _)) = fields[1].rsplit_once('-') {
                if let Ok(ver) = Version::parse(ver) {
                    ver
                } else {
                    continue;
                }
            } else {
                continue;
            };

            map.insert(fields[0].to_string(), version);
        }
        let bin_list = BinList { map };

        Ok(bin_list)
    }

    pub fn need_tag<N: AsRef<str>, T: AsRef<str>>(&self, name: N, tag: T) -> bool {
        if let Some(built_version) = self.map.get(name.as_ref()) {
            if let Ok(tag_version) = Version::parse(tag.as_ref().trim_start_matches('v')) {
                return built_version < &tag_version;
            }
        }

        true
    }

    pub fn built_version<S: AsRef<str>>(&self, name: S) -> Option<String> {
        self.map.get(name.as_ref()).map(|v| v.to_string())
    }
}
