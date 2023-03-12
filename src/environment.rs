use std::{env, fs, io::Error};

const DEFAULT_WIN_PATH: &str = "%userprofile%\\password-store";
const DEFAULT_UNIX_PATH: &str = "~/.password-store";

pub struct Variables {
    pub root_directory: String,
    pub gpg_path: String,
}
pub fn get_variables() -> Result<Variables, Error> {
    let root_directory = match env::var("PASSBOOK_ROOT_DIRECTORY") {
        Ok(value) => value,
        Err(_) => {
            if cfg!(target_os = "linux") {
                DEFAULT_UNIX_PATH.to_owned()
            } else {
                DEFAULT_WIN_PATH.to_owned()
            }
        }
    };
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

    let gpg_path = env::var("PASSBOOK_GPG_PATH").expect("PASSBOOK_GPG_PATH not specified");

    return Ok(Variables {
        root_directory,
        gpg_path,
    });
}
