use std::io;

use anyhow::{bail, Result};
use clap::{App, AppSettings, Arg, ArgEnum};
use clap_complete::{generate, Generator, Shell};

fn print_completions<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}

pub fn completion(shell: &str) -> Result<()> {
    let mut app = build_cli();
    if let Ok(gen) = Shell::from_str(shell, true) {
        print_completions(gen, &mut app)
    } else {
        bail!("Unknown shell `{}` for completion", shell)
    }

    Ok(())
}

pub fn build_cli() -> clap::App<'static> {
    App::new("avtag")
        .about("Shows available tags for git repositories")
        .version("0.1.0")
        .setting(AppSettings::NoAutoVersion)
        .arg(Arg::new("ascii").long("ascii").help("Use only ascii characters to format output"))
        .arg(
            Arg::new("completion")
                .long("completion")
                .help("Generate completions")
                .takes_value(true)
                .possible_values(&["bash", "elvish", "fish", "powershell", "zsh"]),
        )
}
