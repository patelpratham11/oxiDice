#[allow(non_snake_case)]

use clap::{Arg, ArgAction, Command};
use oxiDice::config::Configurator;
use oxiDice::pass_code::PassCode;
use oxiDice::pass_phrase::PassPhrase;

fn main() {
    let matches = Command::new("oxiDice")
        .author("Pratham Patel")
        .about("Simple CLI to generate passwords.")
        .long_about("Rust-based CLI to generate custom passphrases based on diceware OR passcodes based on set criteria. Users can set the length along with flags for numbers, special characters, or look alikes.")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_parser(["code", "phrase"])
                .default_value("phrase")
                .help("What would you like to generate")
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .requires("type")
                .action(ArgAction::Set)
                .default_value("10")
                .value_parser(clap::value_parser!(usize))
                .help("Specify the length of the passcode."),
        )
        .arg(
            Arg::new("numbers")
                .short('n')
                .long("numbers")
                .requires("type")
                .action(ArgAction::SetFalse)
                .default_value("true")
                .help("Specify whether to use numbers [0-9] or not. [default: true]"),
        )
        .arg(
            Arg::new("specials")
                .short('s')
                .long("special")
                .requires("type")
                .action(ArgAction::SetFalse)
                .default_value("true")
                .help("Specify whether to use special characters or not. [default: true]"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .action(ArgAction::Set)
                .default_value("5")
                .value_parser(clap::value_parser!(usize))
                .help("Number of words to generate for diceware"),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .action(ArgAction::Set)
                .default_value("-")
                .help("Delimiter to use between words."),
        )
        .arg(
            Arg::new("count")
                .short('#')
                .long("count")
                .action(ArgAction::Set)
                .default_value("10")
                .value_parser(clap::value_parser!(usize))
                .help("Number of passes to generate."),
        )
        .get_matches();

        let config = Configurator { matches };
        if config.matches.get_one::<String>("type").unwrap() == "code"{
            PassCode::generate (&config );
        } else {
            PassPhrase::generate(&config);
        }
    
}
