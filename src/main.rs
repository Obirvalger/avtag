use std::collections::BTreeMap;

use anyhow::Result;
use cmd_lib::run_cmd;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, ContentArrangement, Table};
use scopeguard::defer;

use crate::bin_list::BinList;
use crate::repo::Repo;
use crate::tmpdir::TMPDIR;

mod bin_list;
mod cli;
mod config;
mod files;
mod repo;
mod tmpdir;
mod util;

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
    let matches = cli::build_cli().get_matches();
    if let Some(shell) = matches.value_of("completion") {
        cli::completion(shell)?;
        return Ok(());
    }
    if files::install_config()? {
        println!("Installed config to {}", config::config_path().display());
        println!("Edit defaults and repos sections before use");
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
        repo_tags.insert(repo, repo.get_tags(&bin_list)?);
    }

    show_tags_table(&repo_tags, &bin_list, ascii)?;

    Ok(())
}
