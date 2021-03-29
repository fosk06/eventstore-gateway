extern crate dotenv;
use dotenv::dotenv;
use crate::toto::titi;

#[macro_use]
extern crate dotenv_codegen;

fn main() {
    println!("Hello, world!");
    dotenv().ok();
    println!("{}", dotenv!("OUTPUT_STREAM"));
}
