#[allow(non_snake_case)]

use clap::{Arg, ArgAction, command, Command};
use oxiDice::config::Configurator;
use oxiDice::pass_code::PassCode;

fn main() {
    let matches = Command::new("oxiDice")
        .author("Pratham Patel")
        .about("Simple CLI to generate passwords.")
        .long_about("Rust-based CLI to generate custom passphrases based on diceware OR passcodes based on set criteria. Users can set the length along with flags for numbers, special characters, or look alikes.")
        .arg(
            Arg::new("code")
                .short('c')
                .long("code")
                .action(ArgAction::SetTrue)
                .default_value("false")
                .value_parser(clap::value_parser!(bool))
                .help("Generate a passCODE.")
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .requires("code")
                .action(ArgAction::Set)
                .default_value("10")
                .value_parser(clap::value_parser!(usize))
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
            Arg::new("specials")
                .short('s')
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

        let config = Configurator { matches };
        if config.matches.get_flag("code"){
            PassCode::generate (&config );
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
