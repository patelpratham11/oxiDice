use clap::Parser;

/// Simple passphrase generator based on diceware
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of words to generate
    #[arg(short, long, default_value_t = 5)]
    words: u8,

    /// Delimiter character
    #[arg(short, long, default_value = "-" )]
    delimiter: String,

    /// Number of passphrases to generate
    #[arg(short, long, default_value = 1)]
    number: u8,
}

fn main() {
    let args = Args::parse();

    println!("{} words with {} delim", args.words, args.delimiter);
}