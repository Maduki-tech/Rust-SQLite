mod prompts;
mod sql_handler;
use crate::prompts::promt;

fn main() {
    // TODO: READ IN THE FILE
    println!("Welcome to the Rust SQL CLI");
    promt::Prompt::new().run();
}
