use std::fmt::Error;
use std::fs::{self, DirEntry};
use std::io::{self, stdin};
use std::path::Path;
use std::process::exit;

fn parse_dir_entry(entry: &DirEntry) -> Result<Password, Error> {
    let name: String = match entry.file_name().to_str() {
        Some(st) => st.into(),
        None => "".into(),
    };

    let absolute_path: String = match entry.path().to_str() {
        Some(st) => st.into(),
        None => "".into(),
    };

    let relative_path: String = String::from(
        absolute_path
            .get(DEFAULT_PATH.len().into()..)
            .expect("Error"),
    );
    // put in structure

    let password = Password {
        name,
        _absolute_path: absolute_path,
        relative_path,
    };

    return Ok(password);
}

fn extract_passwords(dir: &Path, passwords: &mut Vec<Password>) -> io::Result<()> {
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
                    passwords.extend([parse_dir_entry(&entry).expect("Failed to parse password")]);
                }
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
struct Password {
    name: String,
    relative_path: String,
    _absolute_path: String,
}
const DEFAULT_PATH: &str = "C:\\Users\\wholteza\\password-store";
fn main() {
    // find all files
    let path = Path::new(DEFAULT_PATH);
    if !path.is_dir() {
        println!("That is not a directory");
        return;
    }
    println!("That is a directory!");

    let mut passwords = vec![];
    _ = extract_passwords(path, &mut passwords);

    passwords.iter().for_each(|p| println!("{}", p.name));

    println!("What password are you looking for?");
    let mut search_input = String::new();
    stdin()
        .read_line(&mut search_input)
        .expect("A search string was not provided");

    // find file
    let found_password = passwords
        .iter()
        .find(|&p| (p.relative_path.as_str()).contains(search_input.trim()));

    match found_password {
        None => {
            println!("Sorry but i cannot find that password");
            exit(1)
        }
        Some(password) => {
            println!("Found password: {}", password.relative_path)
        }
    }

    // print file path or smth
}
