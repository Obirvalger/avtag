use std::path::PathBuf;

use cmd_lib::run_fun;
use once_cell::sync::Lazy;

pub static TMPDIR: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(run_fun! {mktemp -d}.expect("can't create tmpdir")));
