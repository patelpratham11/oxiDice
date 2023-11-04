#[allow(non_snake_case)]
pub struct Entropy {}

pub fn entropy(pass: &str) -> f32 {
    (pass.chars().count() as f32)*(calculate_unique(pass).log2())
}

fn calculate_unique(pass: &str) -> f32{
    let mut v: Vec<char> = pass.chars().collect();
    v.dedup();
    v.len() as f32
}