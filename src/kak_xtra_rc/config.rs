use std::path::Path;

pub enum ConfigDirValidationErr {
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

pub fn parse_config_dir(config_dir: &str) -> Result<String, ConfigDirValidationErr> {
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
