use std::fmt::Error;
use std::fs::{self, DirEntry};
use std::io::{self, stdin};
use std::path::Path;

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
            .get(default_path.len().into()..)
            .expect("Error"),
    );
    // put in structure

    let password = Password {
        name,
        absolute_path,
        relative_path,
    };

    return Ok(password);
}

fn extract_passwords(dir: &Path, passwords: &mut Vec<Password>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                extract_passwords(&path, passwords)?;
            } else {
                passwords.extend([parse_dir_entry(&entry).expect("Failed to parse password")]);
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
struct Password {
    name: String,
    relative_path: String,
    absolute_path: String,
}
const default_path: &str = "C:\\Users\\wholteza\\password-store";
fn main() {
    // get dir

    println!("What is your pass root directory?");

    let mut buffer: String = String::new();

    stdin().read_line(&mut buffer);
    let directory_path: &str = match buffer.trim_end() {
        "" => default_path.into(),
        name => name.into(),
    };
    println!("{directory_path}");

    // find all files
    let path = Path::new(directory_path);
    if !path.is_dir() {
        println!("That is not a directory");
        return;
    }
    println!("That is a directory!");

    let mut passwords: Vec<Password> = vec![];
    extract_passwords(path, &mut passwords);

    // print to screen
    // for password in passwords {
    //     println!(
    //         "{}, {}, {}",
    //         password.name, password.absolute_path, password.relative_path
    //     );
    // }

    passwords.iter().for_each(|p| println!("{}", p.name));

    // get search string
    println!("What password are you looking for?");
    let mut search_input = String::new();
    stdin()
        .read_line(&mut search_input)
        .expect("A search string was not provided");

    println!("{}", &search_input);

    // find file
    let found_password = passwords
        .into_iter()
        .find(|p| p.absolute_path.contains(&search_input));

    if found_password.is_none() {
        println!("Sorry but i cannot find that password");
        return;
    }

    // print file path or smth
    println!("Found password: {}", found_password.expect("never").name);
}
