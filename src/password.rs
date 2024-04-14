use std::io;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    fs::{self, DirEntry},
    io::Error,
};

use otpauth::TOTP;
use regex::Regex;

use crate::{environment, gpg};

#[derive(Clone)]
pub struct PasswordFile {
    pub name: String,
    pub relative_path: String,
    pub absolute_path: String,
}

#[derive(Clone)]
pub struct Password {
    pub password: String,
    pub raw_totp: String,
    pub rest: String,
}

impl Password {
    pub fn generate_totp(&self) -> Option<String> {
        let mut secret = String::new();

        let rx = Regex::new("secret=(.*)$").expect("Could not create regex");
        if let Some(mat) = rx.find(&self.raw_totp) {
            secret.push_str(mat.as_str())
        }

        let rx = Regex::new("secret=(.*)&").expect("Could not create regex");
        if let Some(mat) = rx.find(&self.raw_totp) {
            secret.push_str(mat.as_str())
        }
        println!("{secret}");
        let auth = TOTP::new(secret);
        let totp = auth.generate(
            30,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("cannot get time")
                .as_secs(),
        );

        return Some(totp.to_string());
    }
}

pub fn into_password(pass: &PasswordFile) -> io::Result<Password> {
    let environment_variables =
        environment::get_variables().expect("Could not read environment variables");

    let raw_decrypted_output = gpg::decrypt(&pass.absolute_path, &environment_variables.gpg_path)
        .expect("Could not decrypt file");

    let mut password = String::new();
    let mut raw_totp = String::new();
    let mut rest = String::new();

    let splits = raw_decrypted_output.split('\n');
    let mut is_first_row = true;
    for split in splits {
        if is_first_row {
            password.push_str(split);
            is_first_row = false;
            continue;
        }
        if split.contains("otpauth://") {
            raw_totp.push_str(split);
            continue;
        }
        rest.push_str(split);
        rest.push_str("\n");
    }

    if password.is_empty() {
        return Err(std::io::Error::new(
            io::ErrorKind::InvalidInput,
            "File did not contain a password",
        ));
    }

    Ok(Password {
        password,
        raw_totp,
        rest,
    })
}

fn into_password_file(entry: &DirEntry) -> Result<PasswordFile, Error> {
    let environment_variables = match environment::get_variables() {
        Ok(value) => value,
        Err(_) => {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                "Unable to read environment variables",
            ));
        }
    };

    let file_name: String = match entry.file_name().to_str() {
        Some(file_name) => file_name.into(),
        None => {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                "Could not convert file name into string",
            ))
        }
    };

    let absolute_path: String = match entry.path().to_str() {
        Some(absolute_path) => absolute_path.into(),
        None => {
            return Err(Error::new(
                io::ErrorKind::InvalidInput,
                "Could not convert absolute path into string",
            ))
        }
    };

    let relative_path =
        match absolute_path.get((&environment_variables.root_directory).len().into()..) {
            Some(path) => String::from(path),
            None => {
                return Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    "Could not extract relative path from absolute path",
                ))
            }
        };

    let password = PasswordFile {
        name: file_name,
        absolute_path,
        relative_path,
    };

    return Ok(password);
}

fn extract_passwords_files(dir: &Path, passwords: &mut Vec<PasswordFile>) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let directory_name: &str = match dir.file_name() {
        Some(directory_name) => match directory_name.to_str() {
            Some(directory_name) => directory_name,
            None => "",
        },
        None => "",
    };

    let directory_should_be_ignored = directory_name.is_empty() || directory_name.contains(".git");

    if directory_should_be_ignored {
        return Ok(());
    }

    for directory_entry in fs::read_dir(dir)? {
        let directory_entry = directory_entry?;
        let path = directory_entry.path();
        if path.is_dir() {
            extract_passwords_files(&path, passwords)?;
        } else {
            let file_name = match path.file_name() {
                Some(file_name) => match file_name.to_str() {
                    Some(file_name) => file_name,
                    None => "",
                },
                None => "",
            };

            if file_name.contains(".gpg-id") {
                continue;
            }

            let password_file = match into_password_file(&directory_entry) {
                Ok(password_file) => password_file,
                Err(_) => {
                    return Err(Error::new(
                        io::ErrorKind::InvalidInput,
                        "Could not convert file into password",
                    ))
                }
            };

            passwords.extend([password_file]);
        }
    }
    Ok(())
}

pub fn get_password_files(
    path: &str,
    password_files_buffer: &mut Vec<PasswordFile>,
) -> Result<(), Error> {
    let path = Path::new(path);
    if !path.is_dir() {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "Path provided is not a directory",
        ));
    }

    match extract_passwords_files(path, password_files_buffer) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}
