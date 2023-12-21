use super::sql::Sql;

pub struct SqlHandle {
    sql: Sql,
}

impl SqlHandle {
    pub fn new() -> SqlHandle {
        SqlHandle {
            sql: Sql::new(),
        }

    }

    pub fn create(&mut self, buffer: String) {
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
        if input[1] == "*" {
            println!("{} {}", self.sql.get_values().0, self.sql.get_values().1);
        }

    }
}
