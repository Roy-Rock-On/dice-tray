use std::collections::HashMap;

pub struct DiceResultTable{
    pub table: HashMap<u32, String>
}


pub fn implement_test_table() -> DiceResultTable{
    DiceResultTable{
        table: HashMap::from([
            (1, "Critical Fail".to_string()),
            (2, "Fail".to_string()),
            (3, "Neutral".to_string()),
            (4, "Success".to_string()),
            (5, "Critical Success".to_string()),
            (6, "Unexpected Result".to_string())
        ])
    }
}