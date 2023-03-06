use std::{
    fs::{self, DirEntry},
    io::{self, Error},
    path::Path,
};

use crate::constants::DEFAULT_PATH;

#[derive(Clone)]
pub struct Password {
    pub name: String,
    pub relative_path: String,
    pub absolute_path: String,
}

fn to_password(entry: &DirEntry) -> Result<Password, Error> {
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

    let password = Password {
        name,
        absolute_path,
        relative_path,
    };

    return Ok(password);
}

pub fn extract_passwords(dir: &Path, passwords: &mut Vec<Password>) -> io::Result<()> {
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
                extract_passwords(&path, passwords)?;
            } else {
                let name = match path.file_name() {
                    Some(os_string_name) => match os_string_name.to_str() {
                        Some(name) => name,
                        None => "",
                    },
                    None => "",
                };
                if !name.contains(".gpg-id") {
                    passwords.extend([to_password(&entry).expect("Failed to parse password")]);
                }
            }
        }
    }
    Ok(())
}
