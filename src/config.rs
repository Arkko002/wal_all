use std::{env, path::{Path, PathBuf}, str::FromStr};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub alacritty_bin: PathBuf,
    pub discord_bin: PathBuf,
    pub discord_config: PathBuf,
    pub pywalfox_bin: PathBuf,
    pub zathura_bin: PathBuf,
    pub zathura_config: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alacritty_bin: find_in_path("alacritty").unwrap(),
            discord_bin: find_in_path("pywal-discord").unwrap(),
            pywalfox_bin: find_in_path("pywalfox").unwrap(),
            zathura_bin: find_in_path("genzathura").unwrap(),
            zathura_config: PathBuf::from_str("~/.config/zathura/zathurarc").unwrap(),
            discord_config: PathBuf::from_str("~/.config/BetterDiscord/themes/pywal-discord-default.theme.css").unwrap()
        }
    }
}

fn find_in_path<P>(exe: P) -> Option<PathBuf> where P: AsRef<Path>, {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
