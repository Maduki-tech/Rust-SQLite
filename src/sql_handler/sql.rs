pub struct Sql {
    name: String,
    age: i32,
}

impl Sql {
    pub fn new() -> Sql {
        Sql {
            name: String::from(""),
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
