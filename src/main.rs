use std::io::stdin;
use std::path::Path;
use std::process::exit;

use crate::constants::DEFAULT_PATH;
use crate::password::{extract_passwords_files, into_password};

mod constants;
mod password;

fn main() {
    let path = Path::new(DEFAULT_PATH);
    if !path.is_dir() {
        println!("That is not a directory");
        return;
    }
    println!("That is a directory!");

    let mut passwords = vec![];
    _ = extract_passwords_files(path, &mut passwords);

    println!("What password are you looking for?");
    let mut search_input = String::new();
    stdin()
        .read_line(&mut search_input)
        .expect("A search string was not provided");

    let found_password_file = passwords
        .iter()
        .find(|&p| (p.relative_path.as_str()).contains(search_input.trim()));

    let password = match found_password_file {
        Some(password_file) => match into_password(password_file) {
            Ok(password) => password,
            Err(_) => {
                println!("Sorry but i cannot decrypt that password");
                exit(1)
            }
        },
        None => {
            println!("Sorry but i cannot find that password");
            exit(1)
        }
    };

    println!("{}", password.password)
}
