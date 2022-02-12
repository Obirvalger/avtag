use std::path::{Path, PathBuf};

pub fn expand_tilde(path: &Path) -> PathBuf {
    let s = path.to_string_lossy().to_string();
    PathBuf::from(shellexpand::tilde(&s).to_string())
}

pub fn optional_join<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: &Option<B>) -> PathBuf {
    if let Some(base) = base {
        base.as_ref().join(path)
    } else {
        expand_tilde(path.as_ref())
    }
}
