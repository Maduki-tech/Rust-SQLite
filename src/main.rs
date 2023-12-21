

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!("Welcome to the Rust SQL CLI");

    let res = match buffer.trim_end() {
        "" => "No name Provided".to_owned(),
        name => format!("Hello, {}!", name),
    };

    println!("{}", res);
}
