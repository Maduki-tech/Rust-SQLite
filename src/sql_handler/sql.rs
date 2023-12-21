// TODO: add rows to the sql
pub struct Sql {
    data: Vec<Row>,
}

struct Row {
    name: String,
    age: i32,
}

impl Sql {
    /// Create a new [`SQL`] struct
    pub fn new() -> Sql {
        Sql { data: Vec::new() }
    }

    /// set the values of the [`SQL`] struct
    ///
    /// * `name`:
    /// * `age`:
    pub fn set_values(&mut self, name: String, age: i32) {
        self.data.push(Row { name, age });
    }

    /// Get the values of the [`SQL`] struct
    pub fn get_values(&self) -> Vec<(String, i32)> {
        return self.data
            .iter()
            .map(|row| (row.name.clone(), row.age))
            .collect::<Vec<(String, i32)>>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_values() {
        let mut sql = Sql::new();
        sql.set_values(String::from("test"), 1);
        assert_eq!(sql.get_values(), vec![(String::from("test"), 1)]);
    }
}
