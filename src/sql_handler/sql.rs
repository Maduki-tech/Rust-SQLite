use std::{fs::File, io::{Write, Read}};

pub struct Sql {
    data: Vec<Row>,
    file: File,
}

struct Row {
    name: String,
    age: i32,
}

impl Sql {
    /// Create a new [`SQL`] struct
    pub fn new(file: &File) -> Sql {
        Sql {
            data: Vec::new(),
            file: file.try_clone().unwrap(),
        }
    }

    /// set the values of the [`SQL`] struct
    ///
    /// * `name`:
    /// * `age`:
    pub fn set_values(&mut self, name: String, age: i32) {
        self.data.push(Row { name, age });
        self.create_file();
    }

    /// Get the values of the [`SQL`] struct
    pub fn get_values(&self) -> Vec<(String, i32)> {
        return self
            .data
            .iter()
            .map(|row| (row.name.clone(), row.age))
            .collect::<Vec<(String, i32)>>();
    }

    /// Get the values of the [`SQL`] struct by name
    ///
    /// * `name`:
    pub fn get_values_by_name(&self, name: String) -> Vec<(String, i32)> {
        return self
            .data
            .iter()
            .filter(|row| row.name == name)
            .map(|row| (row.name.clone(), row.age))
            .collect::<Vec<(String, i32)>>();
    }

    fn create_file(&mut self) {
        self.file
            .write(format!("{} {}\n", "Name", "Age").as_bytes())
            .unwrap();
        for row in &self.data {
            let row = format!("{} {}\n", row.name, row.age);
            self.file.write_all(row.as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_values() {
        let mut sql = Sql::new(&File::create("test.txt").unwrap());
        sql.set_values(String::from("test"), 1);
        assert_eq!(sql.get_values(), vec![(String::from("test"), 1)]);
    }
}
