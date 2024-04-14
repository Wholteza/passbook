use std::io::stdin;
use std::process::exit;

use crate::environment::get_variables;
use crate::password::{get_password_files, into_password};

mod environment;
mod gpg;
mod password;
// Not including sha1 until used to avoid warnings
// mod sha1;
mod totp;

fn main() {
    let environment_variables = match get_variables() {
        Ok(value) => value,
        Err(_) => {
            println!("Unable to read environment variables");
            exit(1)
        }
    };

    let mut password_files = vec![];
    get_password_files(&environment_variables.root_directory, &mut password_files)
        .expect("Unable to detect password files");

    println!("What password are you looking for?");
    let mut search_input = String::new();
    stdin()
        .read_line(&mut search_input)
        .expect("A search string was not provided");

    let found_password_file = password_files
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

    // copy_to_clipboard(&password.password);

    println!(
        "{}\n{}\n{}",
        password.password,
        password.generate_totp().unwrap(),
        password.raw_totp
    );
}

// fn copy_to_clipboard(text: &str) {
//     let mut clipboard = Clipboard::new().expect("err");
//     clipboard.set_text(text).unwrap();
//     // Workaround for text not being sent to clipboard on kubuntu 22.04
//     sleep(Duration::from_millis(10));
//     println!("gpg output: {}", b)
// }
