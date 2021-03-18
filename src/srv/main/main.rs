//! This is the server backend for DnDCent. It both serves the pages and responds to requests about
//! the character object.
//!
//! # Overview
//!
//! The general process of serving the content is as follows:
//!
//! 1. Startup and load the character file given as command line arg.
//! 2. Open a browser to localhost.
//! 3. Respond to requests.
//!
//! The server is responsible for resolving [StoredCharacters](StoredCharacter) into [Characters](character::Character).
//! That is a very (very) involved process and is the whole reason why this project is insane.
//! More deats can be found in the documentation for other files.

#![feature(proc_macro_hygiene, decl_macro)]
#[deny(missing_docs)]

#[macro_use] extern crate rocket;

extern crate proc_macros;

use std::process::exit;
use crate::character::StoredCharacter;
use colored::Colorize;

mod server;
mod character;
mod content;
mod feature;
mod moves;
mod misc;
mod macros;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let path = match args.len() {
        1 if cfg!(debug_assertions) => {
            "test_character.json".to_string()
        }
        2 if args[1] != "--help" => {
            args[1].clone()
        }
        3 => {
            let path =
                match &*args[1] {
                    "--new" | "-n" => args[2].clone(),
                    _ => match &*args[2] {
                        "--new" | "-n" => args[1].clone(),
                        _ => display_help()
                    }
                };
            if std::path::Path::new(&*path).exists() {
                println!("{}",
                         format!("File {} already exists.\nPlease delete it before creating a new character file with that name.", path)
                             .bright_red()
                );
                exit(2);
            }
            StoredCharacter::default().write(&*path);
            path
        }
        _ => display_help()
    };
    if !std::path::Path::new(&*path).exists() {
        println!(
            "{}",
            format!("File {} does not exist.\nPlease check that you gave the right path, or create it with -n or --new.", path)
                .bright_red()
        );
        exit(1);
    }
    let rocket = server::ignite(path.clone());
    if cfg!(not(debug_assertions)) {
        println!("Serving {} on {}", path.green(), "http://localhost:8000".green());
    }
    rocket.launch();
}

fn display_help() -> ! {
    println!(
        indoc::indoc! { r"
            {} {} command line help

            This is the DnDCent server. It either loads or creates a character file, and serves
            the inteface on {}.

            USAGE: dndcent [--new | -n] <file>

                <file>:     name of character file, should have .json extension (not required)
                            The filename has no relation to the character's name.
                -n, --new:  create a new default character. (fails if file already exists.)

            EXAMPLES:

                # load and run and already existing character file:
                dndcent jeebus-creebus.json

                # create and run a new character file:
                dndcent --new jeebus-creebus.json
        "},
        "DnDCent".bright_green(),
        env!("CARGO_PKG_VERSION").green(),
        "http://localhost:8000".bright_blue()
    );
    exit(0)
}