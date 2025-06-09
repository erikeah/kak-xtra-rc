mod kak_xtra_rc;

use kak_xtra_rc::config::parse_config_dir;
use kak_xtra_rc::interface::source_kak;
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
        {
            let rc_path = valid_dir.clone() + "/kakrc";
            let rc = Path::new(&rc_path);
            let source = match source_kak(&rc) {
                Ok(value) => value,
                // TODO: Transparent error communication
                Err(_e) => continue,
            };
            print!("{source}");
        }
    }
}
