use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, env, path::PathBuf, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalAllConfig {
    pub exec_list: Vec<PathBuf>,
    pub exec_with_args: HashMap<PathBuf, Vec<String>>,
    pub exec_with_write: HashMap<PathBuf, PathBuf>,
}

impl Default for WalAllConfig {
    fn default() -> Self {
        let vec_exec = vec![
            find_in_path("walcritty.sh").unwrap(),
            find_in_path("pywal-discord").unwrap(),
        ];

        let mut map_with_args = HashMap::new();
        map_with_args.insert(
            find_in_path("pywalfox").unwrap(),
            vec!["update".to_string()],
        );

        let mut map_with_write = HashMap::new();
        map_with_write.insert(
            find_in_path("genzathurarc").unwrap(),
            PathBuf::from_str("/home/arkko/.config/zathura/zathurarc").unwrap(),
        );

        Self {
            exec_list: vec_exec,
            exec_with_args: map_with_args,
            exec_with_write: map_with_write,
        }
    }
}

fn find_in_path(exe: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(exe);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
