use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct DiceResultTable {
    name: String,
    table: HashMap<u32, String>,
}

impl DiceResultTable {
    ///Creates a new DiceResultTable from a Vector of Strings.
    ///The Strings are moved into the the new table.
    pub fn new(name: String, table_results: Vec<String>) -> DiceResultTable {
        let mut index = 1;
        let table: HashMap<u32, String> = table_results
            .into_iter()
            .map(|s| {
                let kvp = (index, s);
                index += 1;
                kvp
            })
            .collect();
        Self { name, table }
    }

    ///Looks up the result of the table. Values that fall out of the table range are "wraped" in order to always return a value.
    pub fn lookup(&self, mut face: u32) -> Result<&str, String> {
        let table_length: u32 = match self.table.len().try_into() {
            Ok(len) => len,
            Err(_) => {
                return Err(format![
                    "Table length is larger than the max supported by dice-tray. Are you sure you need a table this big?"
                ]);
            }
        };

        if face > table_length {
            let leftovers = face - table_length;
            face = leftovers;
        }

        match self.table.get(&face) {
            Some(s) => Ok(&s),
            None => Err(format![
                "No result found at {} in table {} ",
                face, self.name
            ]),
        }
    }

    ///Returns the name of the table.
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub fn implement_test_table() -> DiceResultTable {
    DiceResultTable {
        name: "test".to_string(),
        table: HashMap::from([
            (1, "Critical Fail".to_string()),
            (2, "Fail".to_string()),
            (3, "Neutral".to_string()),
            (4, "Success".to_string()),
            (5, "Critical Success".to_string()),
            (6, "Unexpected Result".to_string()),
        ]),
    }
}
