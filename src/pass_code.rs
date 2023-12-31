#[allow(non_snake_case)]
use crate::config::Configurator;
use crate::entropy;
use rand::seq::SliceRandom;

pub struct PassCode {}

impl PassCode {
    pub fn generate(config: &Configurator) {
        let pool = Self::code_pool(config);
        let end = config.matches.get_one::<usize>("count").unwrap();
        for _ in 0..*end {
            let pass = Self::generate_code(config, &pool);
            println!("{} --> ENTROPY: {}\n", pass, entropy::entropy(&pass));
        }
        println!("Please clear the terminal after!");
    }

    fn code_pool(config: &Configurator) -> Vec<char> {
        let mut chars = Vec::new();
        if config.matches.get_flag("numbers"){
            for x in b'0'..b'9' + 1{
                chars.push(x as char);
            }
        }
        if config.matches.get_flag("specials"){
            let special_chars: [char; 18] = [
                '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-', ',', '=', '/', '?',
            ];
            chars.append(&mut special_chars.to_vec());
        }
        for x in b'a'..b'z' + 1{
            chars.push(x as char);
        }
        for x in b'A'..b'Z' + 1{
            chars.push(x as char);
        }
        chars
    }

    fn generate_code(config: &Configurator, pool: &Vec<char>) -> String {
        let mut pass_vec: Vec<char> = Vec::new();
        let end = config.matches.get_one::<usize>("length").unwrap();
        for _ in 0..*end as usize{
            pass_vec.push(*pool.choose(&mut rand::thread_rng()).unwrap());
        }
        let pass: String = pass_vec.into_iter().collect();
        return pass;
    }
}