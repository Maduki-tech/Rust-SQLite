mod prompts;
mod sql_handler;
use crate::prompts::promt;

fn main() {
    // TODO: READ IN THE FILE
    // get input file as argument
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    if args.len() < 2 {
        panic!("Please provide a file name");
    }
    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(filename)
        .unwrap();


    println!("Welcome to the Rust SQL CLI");
    promt::Prompt::new(&file).run();
}
