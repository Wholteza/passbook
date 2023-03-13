use std::{env, fs, io::Error};

const DEFAULT_UNIX_PATH: &str = "~/.password-store";

pub struct Variables {
    pub root_directory: String,
    pub gpg_path: String,
}
pub fn get_variables() -> Result<Variables, Error> {
    // TODO: fix this error handling
    let root_directory = get_root_directory().expect("error");

    // TODO: Throw error here and handle on top level to let the user know what went wrong
    let gpg_path = env::var("PASSBOOK_GPG_PATH").expect("PASSBOOK_GPG_PATH not specified");

    return Ok(Variables {
        root_directory,
        gpg_path,
    });
}

fn get_root_directory() -> Result<String, Error> {
    // TODO: Fix this error handling
    let root_directory = match env::var("PASSBOOK_ROOT_DIRECTORY") {
        Ok(value) => {
            if cfg!(target_os = "linux") {
                match into_absolute_if_relative(value) {
                    Ok(path) => path,
                    Err(_) => {
                        return Err(Error::new(std::io::ErrorKind::InvalidInput, "error"));
                    }
                }
            } else {
                value
            }
        }
        Err(_) => {
            if cfg!(target_os = "linux") {
                DEFAULT_UNIX_PATH.to_owned()
            } else {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, "On windows the environment variable PASSBOOK_ROOT_DIRECTORY must be set to an absolute path"));
            }
        }
    };

    Ok(root_directory)
}

fn into_absolute_if_relative(root_directory: String) -> Result<String, Error> {
    let root_directory = fs::canonicalize(root_directory)?;
    let root_directory = match root_directory.as_path().to_str() {
        Some(value) => value.to_owned(),
        None => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Could not decide root directory",
            ))
        }
    };
    Ok(root_directory)
}
