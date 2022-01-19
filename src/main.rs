use std::collections::BTreeMap;

use anyhow::Result;
use cmd_lib::{run_cmd, run_fun};
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, ContentArrangement, Table};
use scopeguard::defer;

use crate::bin_list::BinList;
use crate::config::Repo;
use crate::tmpdir::TMPDIR;

mod bin_list;
mod cli;
mod config;
mod files;
mod tmpdir;

fn filter_tag<S: AsRef<str>>(bin_list: &Option<BinList>, repo: &Repo, tag: S) -> bool {
    if repo.accept_tag(&tag) {
        if let Some(bin_list) = bin_list {
            bin_list.need_tag(repo.bin_package_name(), tag)
        } else {
            true
        }
    } else {
        false
    }
}

fn show_tags_table(
    repo_tags: &BTreeMap<&Repo, Vec<String>>,
    bin_list: &Option<BinList>,
    ascii: bool,
) -> Result<()> {
    let mut table = Table::new();
    if !ascii {
        table.load_preset(UTF8_FULL);
    }
    table.set_content_arrangement(ContentArrangement::Dynamic).set_table_width(80);
    if bin_list.is_some() {
        table.set_header(vec![Cell::new("Name"), Cell::new("Built version"), Cell::new("Tags")]);
    } else {
        table.set_header(vec![Cell::new("Name"), Cell::new("Tags")]);
    }

    for (repo, tags) in repo_tags {
        if !tags.is_empty() {
            if let Some(bin_list) = bin_list {
                table.add_row(vec![
                    Cell::new(repo.display_name()),
                    Cell::new(bin_list.built_version(repo.bin_package_name()).unwrap_or_default()),
                    Cell::new(tags.join(" ")),
                ]);
            } else {
                table.add_row(vec![Cell::new(repo.display_name()), Cell::new(tags.join(" "))]);
            }
        }
    }

    println!("{}", table);

    Ok(())
}

fn main() -> Result<()> {
    if files::install_config()? {
        println!("Installed config to {}", config::config_path().display());
        println!("Edit defaults and repos sections before use");
        return Ok(());
    }
    let matches = cli::build_cli().get_matches();
    if let Some(shell) = matches.value_of("completion") {
        cli::completion(shell)?;
        return Ok(());
    }
    let ascii = matches.is_present("ascii");
    let config = config::Config::new(matches.value_of("config"))?;
    let tmpdir = &TMPDIR;
    defer! {
        // ignore fail in removing tmpdir
        let _ = run_cmd!(rm -rf $tmpdir);
    }

    let bin_list = if let Some(url) = &config.bin_list.url {
        Some(BinList::new(url, &config.bin_list.extract_cmd)?)
    } else {
        None
    };

    let repos = config.repos();
    let mut repo_tags: BTreeMap<&Repo, Vec<String>> = BTreeMap::new();

    for repo in &repos {
        let repo_path = &repo.path;
        let repo_remote = &repo.remote;
        if let Ok(tags) = run_fun! {
            cd $repo_path;
            git ls-remote --tags --refs --sort=objectname $repo_remote > $tmpdir/remote_tags;
            git show-ref --tags --hash | sort > $tmpdir/fetched_tags;
            join $tmpdir/remote_tags $tmpdir/fetched_tags -v 1 -o 1.2 | sed "s|refs/tags/||";
        } {
            let tags: Vec<String> = tags
                .split('\n')
                .filter(|t| filter_tag(&bin_list, repo, t))
                .take(repo.max_tags)
                .map(|t| t.to_string())
                .collect();
            repo_tags.insert(repo, tags.to_owned());
        }
    }

    show_tags_table(&repo_tags, &bin_list, ascii)?;

    Ok(())
}
