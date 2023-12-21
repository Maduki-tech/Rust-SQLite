mod prompts;
mod sql_handler;
use crate::prompts::promt;

fn main() {
    println!("Welcome to the Rust SQL CLI");
    promt::Prompt::new().run();
}
