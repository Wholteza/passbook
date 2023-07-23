# passbook

pass like password manager

# Getting started

1. Install rust using the official getting started on rust-lang.org
2. cargo install
3. Set up gpg paths

- For most linux distributions you can put this in your .bashrc file:
  ```bash
  export PASSBOOK_GPG_PATH="/usr/bin/gpg"
  export PASSBOOK_ROOT_DIRECTORY="/home/username/.password-store"
  ```

## Requirements Ubuntu

- `sudo apt install build-essential libgpgme-dev xorg-dev`

## Requirements Windows

- Install rust using `i686-pc-windows-gnu` as platform
- You will need Gpg4Win.
  - `winget install gnupg.gpg4win`
- And the Microsoft Visual C++ Build Tools which can be installed from the visual studio installer.

## Todo:

- hook into gpg and use it to decrypt the file, show password
- put password in clipboard
- clear clipboard after interval
- UI time!
-
