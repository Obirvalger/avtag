use std::fs;
use std::io::Result;

use rust_embed::RustEmbed;

use crate::config::config_dir;

#[derive(RustEmbed)]
#[folder = "files/"]
struct Asset;

pub fn install_config() -> Result<bool> {
    let filename = "config.toml";
    let directory = config_dir();
    fs::create_dir_all(&directory)?;

    let config = &directory.join(filename);
    if !config.exists() {
        let content = Asset::get(filename).expect("Can not found embedded config");
        fs::write(&config, content.data)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
