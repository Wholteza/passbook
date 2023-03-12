use std::process::Command;

pub fn decrypt(file_path: &str, gpg_path: &str) -> Result<String, std::io::Error> {
    let stdout = Command::new(gpg_path)
        .args(["--decrypt", file_path])
        .output()
        .expect("Failed to decrypt file")
        .stdout;
    let output = String::from_utf8(stdout).expect("Could not parse decrypted file into a string");
    return Ok(output);
}
