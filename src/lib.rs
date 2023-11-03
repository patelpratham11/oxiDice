mod pass_code;

pub use crate::pass_code::generate;

pub fn code() {
    generate::generate();
}