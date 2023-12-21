pub struct Sql {
    name: String,
    age: i32,
}

impl Sql {
    pub fn new() -> Sql {
        Sql {
            name: String::new(),
            age: 0,
        }
    }

    pub fn set_values(&mut self, name: String, age: i32) {
        self.name = name;
        self.age = age;

    }

    pub fn get_values(&self) -> (String, i32) {
        (self.name.clone(), self.age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_values() {
        let mut sql = Sql::new();
        sql.set_values(String::from("test"), 1);
        assert_eq!(sql.get_values(), (String::from("test"), 1));
    }
}
