use std::{fs::File, io::Error, io::Write, path::PathBuf, process::Command};
pub mod config;

fn main() -> Result<(), Error> {
    let cfg: config::Config = confy::load("config").unwrap();

    exec_cmd(cfg.alacritty_bin).unwrap();
    exec_cmd_with_args(cfg.pywalfox_bin, vec!["update".to_string()]).unwrap();
    exec_cmd_with_write(cfg.discord_bin, cfg.discord_config).unwrap();
    exec_cmd_with_write(cfg.zathura_bin, cfg.zathura_config)
}

fn exec_cmd(cmd: PathBuf) -> Result<(), Error> {
    let mut cmd = Command::new(cmd);
    match cmd.status() {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn exec_cmd_with_args(cmd: PathBuf, args: Vec<String>) -> Result<(), Error> {
    let mut cmd = Command::new(cmd);
    cmd.args(args);
    match cmd.status() {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

fn exec_cmd_with_write(cmd: PathBuf, cfg_file: PathBuf) -> Result<(), Error> {
    let mut cmd = Command::new(cmd);
    let scheme = cmd.output().unwrap();

    let mut file = File::create(cfg_file).unwrap();
    file.write_all(&scheme.stdout)
}
