#[allow(non_snake_case)]

use clap::{Parser, Arg, Command, ArgAction, command};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
                .help("Disable usage of lowercase letters.")
        )
        .arg(
            Arg::new("no")
                .short('n')
                .long("no")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!("verbose: {:?}", matches.get_flag("verbose"));
    println!("no: {:?}", matches.get_flag("no"));
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