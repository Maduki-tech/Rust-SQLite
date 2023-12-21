use crate::sql_handler::sql_handle::SqlHandle;
use std::{io::Write, fs::File};

pub struct Prompt {
    sql_handle: SqlHandle, 
}

impl Prompt {
    pub fn new(file: &File) -> Prompt {
        Prompt {
            sql_handle: SqlHandle::new(file),
        }
    }
    pub fn run(&mut self) {
        loop {
            let buffer = String::new();
            let buffer = self.generate_prompt(buffer);
            self.get_prompt(buffer);
        }
    }

    fn generate_prompt(&self, mut buffer: String) -> String {
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buffer).unwrap();
        return buffer;
    }

    fn get_prompt(&mut self, buffer: String) {
        let command = self.get_command(buffer.clone());
        match command.as_str() {
            ".exit" => self.sql_handle.exit(buffer),
            ".help" => self.sql_handle.help(buffer),
            ".insert" => self.sql_handle.insert(buffer),
            ".select" => self.sql_handle.select(buffer),
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
    fn test_get_prompt_insert() {
        let prompt = Prompt::new();
        let buffer = String::from(".insert");
        let sut = prompt.get_command(buffer);
        assert_eq!(sut, ".insert".to_string());
    }

    #[test]
    fn test_get_prompt_unknown() {
        let prompt = Prompt::new();
        let buffer = String::from(".unknown");
        let sut = prompt.get_command(buffer);
        assert_eq!(sut, ".unknown".to_string());
    }
}
