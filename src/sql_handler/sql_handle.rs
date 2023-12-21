use super::sql::Sql;

pub struct SqlHandle {
    sql: Sql,
}

impl SqlHandle {
    pub fn new() -> SqlHandle {
        SqlHandle { sql: Sql::new() }
    }

    pub fn insert(&mut self, buffer: String) {
        let input = buffer.split_whitespace().collect::<Vec<&str>>();
        let name = input[1].to_string();
        let age = input[2].parse::<i32>().unwrap();
        self.sql.set_values(name, age);
    }
    pub fn exit(&self, _: String) {
        std::process::exit(0);
    }
    pub fn help(&self, _: String) {
        println!("help");
    }

    pub fn select(&self, buffer: String) {
        let input = buffer.split_whitespace().collect::<Vec<&str>>();
        match input[1].to_lowercase().to_string().as_str() {
            "*" => println!("{:?}", self.sql.get_values()),
            "where" => println!("{:?}", self.sql.get_values_by_name(input[2].to_string())),
            _ => println!("Unkown command"),
        }
        if input[1] == "*" {
            println!("{:?}", self.sql.get_values());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut sql_handle = SqlHandle::new();
        let buffer = String::from(".insert test 1");
        sql_handle.insert(buffer);
        println!("{:?}", sql_handle.sql.get_values());
        assert_eq!(sql_handle.sql.get_values(), (String::from("test"), 1));
    }

    #[test]
    fn test_help() {
        let sql_handle = SqlHandle::new();
        let buffer = String::from(".help");
        sql_handle.help(buffer);
    }

    #[test]
    fn test_select() {
        let sql_handle = SqlHandle::new();
        let buffer = String::from(".select *");
        sql_handle.select(buffer);
    }
}
