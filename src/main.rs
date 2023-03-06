use std::io::stdin;
use std::path::Path;
use std::process::exit;
use structopt;

use gpgme::{Context, Protocol};
use std::{
    error::Error,
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
};
use structopt::StructOpt;

use crate::constants::DEFAULT_PATH;
use crate::password::extract_passwords;

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
    _ = extract_passwords(path, &mut passwords);

    passwords.iter().for_each(|p| println!("{}", p.name));

    println!("What password are you looking for?");
    let mut search_input = String::new();
    stdin()
        .read_line(&mut search_input)
        .expect("A search string was not provided");

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

    let pass = found_password.expect("msg");

    let proto = Protocol::OpenPgp;
    let mut ctx = Context::from_protocol(proto)?;
    let mut input = File::open(&pass.absolute_path)?;
    let mut output = Vec::new();
    ctx.decrypt(&mut input, &mut output)
        .map_err(|e| format!("decrypting failed: {:?}", e))?;

    println!("Begin Output:");
    io::stdout().write_all(&output)?;
    println!("End Output.");
}
