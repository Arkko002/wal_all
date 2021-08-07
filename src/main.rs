use std::{
    fs::{File, OpenOptions},
    io::Error,
    io::Write,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use chrono::Local;
pub mod config;

fn main() -> Result<(), Error> {
    let cfg: config::WalAllConfig = confy::load("wal_all").unwrap();

    for cmd in cfg.exec_list.iter() {
        match exec_cmd(cmd) {
            Ok(status) => match status.success() {
                true => continue,
                false => {
                    let err_msg = format!("Error at {} execution.", cmd.to_str().unwrap());
                    write_error_log(err_msg)
                }
            },
            Err(err) => write_error_log(err.to_string()),
        }
    }

    for cmd in cfg.exec_with_args.iter() {
        match exec_cmd_with_args(cmd.0, cmd.1) {
            Ok(status) => match status.success() {
                true => continue,
                false => {
                    // TODO
                    let err_msg = format!("Error at {} execution.", cmd.0.to_str().unwrap());
                    write_error_log(err_msg)
                }
            },
            Err(err) => write_error_log(err.to_string()),
        }
    }

    for cmd in cfg.exec_with_write.iter() {
        match exec_cmd_with_write(cmd.0, cmd.1) {
            Ok(status) => match status.success() {
                true => continue,
                false => {
                    let err_msg = format!("Error at {} execution.", cmd.0.to_str().unwrap());
                    write_error_log(err_msg)
                }
            },
            Err(err) => write_error_log(err.to_string()),
        }
    }

    Ok(())
}

fn exec_cmd(cmd: &PathBuf) -> Result<ExitStatus, Error> {
    let mut cmd = Command::new(cmd);
    Ok(cmd.status()?)
}

fn exec_cmd_with_args(cmd: &PathBuf, args: &Vec<String>) -> Result<ExitStatus, Error> {
    let mut cmd = Command::new(cmd);
    cmd.args(args);
    Ok(cmd.status()?)
}

fn exec_cmd_with_write(cmd: &PathBuf, cfg_file: &PathBuf) -> Result<ExitStatus, Error> {
    let mut cmd = Command::new(cmd);
    let scheme = cmd.output()?;

    let mut file = File::create(cfg_file)?;
    file.write_all(&scheme.stdout)?;

    Ok(scheme.status)
}

fn write_error_log(err: String) {
    let mut logs = OpenOptions::new()
        .append(true)
        .open("./wal_all.logs")
        .unwrap();

    let datetime = Local::now();
    let header = format!("\n\n======== {} ========", datetime.naive_utc().to_string());

    logs.write(header.as_bytes()).unwrap();
    logs.write(err.as_bytes()).unwrap();
}
