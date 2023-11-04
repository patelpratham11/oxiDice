#[allow(non_snake_case)]

use oxiDice::*;
use clap::{Parser, Arg, Command, ArgAction, command};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use oxiDice::config::Configurator;

// /// Simple passphrase generator based on diceware
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Number of words to generate
//     #[arg(short, long, default_value_t = 5)]
//     length: u8,

//     /// Delimiter character
//     #[arg(short, long, default_value = "-", required = false )]
//     delimiter: String,

//     /// Number of passphrases to generate
//     #[arg(short, long, default_value_t = 10)]
//     count: u8,

//     /// Generate similar characters as prefix
//     #[arg(short, long)]
//     similar: ,
// }

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("code")
                .short('c')
                .long("code")
                .action(ArgAction::SetTrue)
                .help("Generate a passCODE. Default --> False")
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .requires("code")
                .action(ArgAction::Set)
                .default_value("10")
                .value_parser(clap::value_parser!(u32))
                .help("Specify the length of the passcode."),
        )
        .arg(
            Arg::new("numbers")
                .short('n')
                .long("numbers")
                .requires("code")
                .action(ArgAction::Set)
                .default_value("true")
                .value_parser(clap::value_parser!(bool))
                .help("Specify whether to use numbers [0-9] or not."),
        )
        .arg(
            Arg::new("similar")
                .short('s')
                .long("similar")
                .requires("code")
                .action(ArgAction::Set)
                .default_value("true")
                .value_parser(clap::value_parser!(bool))
                .help("Specify whether to use similar values {!lL'`0oO} or not."),
        )
        .arg(
            Arg::new("specials")
                .short('b')
                .long("special")
                .requires("code")
                .action(ArgAction::Set)
                .default_value("true")
                .value_parser(clap::value_parser!(bool))
                .help("Specify whether to use special characters or not."),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .action(ArgAction::Set)
                .default_value("5")
                .value_parser(clap::value_parser!(u8))
                .help("Number of words to generate for diceware"),
        )
        .get_matches();

        let config = oxiDice::config::Configurator { matches };
        if config.matches.get_flag("code"){
            oxiDice::pass_code::pass_code::generate (&config );
        } else {
            println!("dice");
        }

    // let args = Args::parse();
    
    // if args.phrase == 0 {
    //     for _ in 0..args.number{
    //         let rand_string: String = thread_rng()
    //         .sample_iter(&Alphanumeric)
    //         .take(40)
    //         .map(char::from)
    //         .collect();
    //         println!("{}\n\n\tEntropy: {}", rand_string, entropy(&rand_string));
    //     }
    // } else{
    //     println!("Implement phrases");
    // }
    
}

fn entropy(pass: &str) -> f32 {
    (pass.chars().count() as f32)*(calculate_unique(pass).log2())
}

fn calculate_unique(pass: &str) -> f32{
    let mut v: Vec<char> = pass.chars().collect();
    v.dedup();
    v.len() as f32
}