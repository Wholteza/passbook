use gpgme::{Context, Protocol};
use std::path::Path;
use std::process::exit;
use std::vec;
use std::{
    fs::File,
    io::{self, prelude::*},
};
use std::{
    fs::{self, DirEntry},
    io::Error,
};

use crate::constants::DEFAULT_PATH;

#[derive(Clone)]
pub struct PasswordFile {
    pub name: String,
    pub relative_path: String,
    pub absolute_path: String,
}

#[derive(Clone)]
pub struct Password {
    pub password: String,
}

pub fn into_password(pass: &PasswordFile) -> io::Result<Password> {
    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto)?;
    let mut input = File::open(&pass.absolute_path)?;
    let mut output = Vec::new();
    match ctx
        .decrypt(&mut input, &mut output)
        .map_err(|e| format!("decrypting failed: {:?}", e))
    {
        Ok(_) => println!("ok"),
        Err(_) => {
            println!("no");
            exit(1);
        }
    };

    let password = match String::from_utf8(output) {
        Ok(password) => password,
        Err(err) => {
            println!("Could not parse contents of file");
            exit(1);
        }
    };

    Ok(Password { password })
}

fn into_password_file(entry: &DirEntry) -> Result<PasswordFile, Error> {
    let name = match entry.file_name().to_str() {
        Some(st) => st.into(),
        None => String::new(),
    };

    let absolute_path = match entry.path().to_str() {
        Some(st) => st.into(),
        None => String::new(),
    };

    let relative_path = match absolute_path.get(DEFAULT_PATH.len().into()..) {
        Some(path) => String::from(path),
        None => String::new(),
    };

    let password = PasswordFile {
        name,
        absolute_path,
        relative_path,
    };

    return Ok(password);
}

pub fn extract_passwords_files(dir: &Path, passwords: &mut Vec<PasswordFile>) -> io::Result<()> {
    if dir.is_dir() {
        let name = match dir.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(name) => name,
                None => "",
            },
            None => "",
        };

        if name.contains(".git") {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                extract_passwords_files(&path, passwords)?;
            } else {
                let name = match path.file_name() {
                    Some(os_string_name) => match os_string_name.to_str() {
                        Some(name) => name,
                        None => "",
                    },
                    None => "",
                };
                if !name.contains(".gpg-id") {
                    passwords
                        .extend([into_password_file(&entry).expect("Failed to parse password")]);
                }
            }
        }
    }
    Ok(())
}
