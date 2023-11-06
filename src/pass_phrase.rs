#[allow(non_snake_case)]
use crate::config::Configurator;
use crate::entropy;
use rand::Rng;

pub struct PassPhrase {}

impl PassPhrase {
    pub fn generate(config: &Configurator) {
        let pool = Self::code_pool();
        let end = config.matches.get_one::<usize>("count").unwrap(); // number to generate
        for _ in 0..*end {
            let pass = Self::generate_code(config, &pool); // get me the password
            println!("{} --> ENTROPY: {}\n", pass, entropy::entropy(&pass)); // print and entropy
        }
        println!("Please clear the terminal after!");
    }

    fn code_pool() -> std::collections::HashMap<String, String> { // read into hashmap
        let wordlist = include_str!("../assets/dice.txt");
        let mut diceware = std::collections::HashMap::new();
        for line in wordlist.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            diceware.insert(parts[0].to_owned(), parts[1].to_owned());
        }
        diceware
    }

    fn generate_code(config: &Configurator, diceware: &std::collections::HashMap<String, String>) -> String {
        let mut rng = rand::thread_rng();
        let mut key = String::new();
        let mut pass_vec: Vec<String> = Vec::new();
        let end = config.matches.get_one::<usize>("words").unwrap();
        let delim = config.matches.get_one::<String>("delimiter").unwrap();

        for x in 0..*end as usize{ // generate this many words
            for _ in 0..5 { // 5 dice
                key.push_str(&(rng.gen_range(1..=6)).to_string()); // can only go from 1-6
            }
            pass_vec.push(diceware.get(&key).unwrap().to_owned());
           if !(x == *end-1){
                pass_vec.push(delim.clone()); // if we're not at the end, add delim
           }
            key = String::new();
        }
        let pass: String = pass_vec.into_iter().collect();
        return pass;
    }
}