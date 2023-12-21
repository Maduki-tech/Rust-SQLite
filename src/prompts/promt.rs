use crate::sql_handler::sql_handle::SqlHandle;
use std::io::Write;

pub struct Prompt {}

impl Prompt {
    pub fn new() -> Prompt {
        Prompt {}
    }
    pub fn run(&self) {
        let buffer = String::new();
        let buffer = self.generate_prompt(buffer);
        self.get_prompt(buffer);
    }

    fn generate_prompt(&self, mut buffer: String) -> String {
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buffer).unwrap();
        return buffer;
    }

    fn get_prompt(&self, buffer: String) {
        let mut handler = SqlHandle::new();
        let command = self.get_command(buffer.clone());
        match command.as_str() {
            ".exit" => handler.exit(buffer),
            ".help" => handler.help(buffer),
            ".create" => handler.create(buffer),
            ".select" => handler.select(buffer),
            _ => println!("Unkown command"),
        }
    }

    fn get_command(&self, buffer: String) -> String {
        let input = buffer.split_whitespace().collect::<Vec<&str>>();
        return input[0].to_lowercase();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prompt_exit() {
        let prompt = Prompt::new();
        let buffer = String::from(".exit");
        let sut = prompt.get_command(buffer);
        assert_eq!(sut, ".exit".to_string());
    }

    #[test]
    fn test_get_prompt_help() {
        let prompt = Prompt::new();
        let buffer = String::from(".help");
        let sut = prompt.get_command(buffer);
        assert_eq!(sut, ".help".to_string());
    }

    #[test]
    fn test_get_prompt_create() {
        let prompt = Prompt::new();
        let buffer = String::from(".create");
        let sut = prompt.get_command(buffer);
        assert_eq!(sut, ".create".to_string());
    }

    #[test]
    fn test_get_prompt_unknown() {
        let prompt = Prompt::new();
        let buffer = String::from(".unknown");
        let sut = prompt.get_command(buffer);
        assert_eq!(sut, ".unknown".to_string());
    }
}
