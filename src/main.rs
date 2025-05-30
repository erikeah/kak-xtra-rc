use std::env::{self, VarError};
use std::path::Path;

use std::process;

#[derive(Debug)]
struct Args {
    help: bool,
}

fn parse_args() -> Args {
    let mut args = Args { help: false };
    for arg in std::env::args() {
        match arg.as_str() {
            "--help" => {
                args.help = true;
            }
            &_ => continue,
        }
    }
    args
}

enum ConfigDirValidationErr {
    DoNotExist,
    IsNotAbsolutePath,
    IsNotADirectory,
    InvalidPath,
}

impl std::fmt::Display for ConfigDirValidationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigDirValidationErr::IsNotAbsolutePath => {
                write!(f, "provided path is not an absolute path")
            }
            ConfigDirValidationErr::DoNotExist => {
                write!(f, "provided path do not exist")
            }
            ConfigDirValidationErr::IsNotADirectory => {
                write!(f, "provided path is not a directory")
            }
            ConfigDirValidationErr::InvalidPath => {
                write!(f, "invalid path")
            }
        }
    }
}

fn parse_config_dir(config_dir: &str) -> Result<String, ConfigDirValidationErr> {
    let actual_dir = Path::new(&config_dir);
    if !actual_dir.exists() {
        return Err(ConfigDirValidationErr::DoNotExist);
    }
    if !actual_dir.is_absolute() {
        return Err(ConfigDirValidationErr::IsNotAbsolutePath);
    }
    if !actual_dir.is_dir() {
        return Err(ConfigDirValidationErr::IsNotADirectory);
    }
    match actual_dir.to_str() {
        Some(s) => {
            if s.ends_with("/") {
                Ok(s.trim_end_matches("/").to_owned())
            } else {
                Ok(s.to_owned())
            }
        }
        None => Err(ConfigDirValidationErr::InvalidPath),
    }
}

fn main() {
    let args = parse_args();
    let xtra_config_env_var_name = "KAKOUNE_EXTRA_CONFIG_DIRS";
    if args.help {
        eprintln!("usage: kak-xtra-rc [options]\n\t--help\tShow this message");
        process::exit(0)
    }
    let config_dirs_raw = match env::var(xtra_config_env_var_name) {
        Ok(string) => string,
        Err(e) => match e {
            VarError::NotPresent => {
                eprintln!("{xtra_config_env_var_name}: {e}: Nothing to do");
                process::exit(0)
            }
            VarError::NotUnicode(_) => {
                eprintln!("Error while reading {xtra_config_env_var_name}: {e}");
                process::exit(2)
            }
        },
    };
    let config_dirs = config_dirs_raw.split(":");
    for config_dir in config_dirs {
        let valid_dir = match parse_config_dir(config_dir) {
            Err(e) => {
                eprintln!("ignoring '{config_dir}' because {e}");
                continue;
            }
            Ok(value) => value,
        };
        let mut buf = String::new();
        // TODO: wrap source rc logic inside a function
        let rc_path = valid_dir.clone() + "/kakrc";
        let rc = Path::new(&rc_path);
        let rc_str = match rc.to_str() {
            Some(value) => value,
            None => {
                continue;
            }
        };
        if rc.exists() && rc.is_file() {
            buf.push_str("source ");
            buf.push_str(rc_str);
            buf.push_str("\n");
        } else {
            eprintln!("ignoring '{valid_dir}' because does not contain a kakrc file");
        }
        print!("{buf}")
    }
}
