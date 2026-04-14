use std::fs::ReadDir;

use serde::{Serialize, Deserialize};
use crate::die::{Die, build_die};

#[derive(Serialize, Deserialize)]
enum ResultType{
    Face(u32),
    Best(u32),
    Worst(u32),
    Sum(u32)
}

impl ResultType{
    fn get_num(&self) -> u32 {
        match self {
            ResultType::Best(x) => *x,
            ResultType::Face(x) => *x,
            ResultType::Sum(x) => *x,
            ResultType::Worst(x) => *x
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Tracking{
    None,
}

#[derive(Serialize, Deserialize)]
struct DieReader{
    die: Die,
    label: String,
    id: usize,
    current_result: ResultType,
    tracking: Tracking
}

const STD_WEIGHT: u32 = 100;
const VAR_WEIGHT: u32 = 25;

impl DieReader{
    pub fn new(id: usize, tag: Option<String>, seed: u64, faces: u32) -> Result<Self, String>{
        let label = match tag{
            Some(l) => l.to_string(),
            None => format!("d{}", faces)
        };

        let mut die = match build_die(faces, seed, STD_WEIGHT, VAR_WEIGHT){
            Ok(d) => d,
            Err(e) => return Err(format!("Failed to create die reader {}", e))
        };

        let current_face = die.roll();

        Ok(DieReader{
            die,
            label,
            id,
            current_result: ResultType::Face(current_face),
            tracking: Tracking::None
        })
    }

    pub fn set_result_type(&mut self, new_type: ResultType){
        let current_result = self.die.get_current_face();
        self.current_result = match new_type{
            ResultType::Face(_) => ResultType::Face(current_result),
            ResultType::Best(_) => ResultType::Best(current_result),
            ResultType::Sum(_) => ResultType::Sum(current_result),
            ResultType::Worst(_) => ResultType::Worst(current_result)
        };
    }

    pub fn roll(&mut self) -> RollLog {
        let new_roll = self.die.roll();
        self.current_result = match &self.current_result {
            &ResultType::Face(_) => ResultType::Face(new_roll),
            &ResultType::Best(x) => {
                if new_roll > x {
                    ResultType::Best(new_roll)
                }
                else{
                    ResultType::Best(x)
                }
            },
            &ResultType::Worst(x) =>{
                if new_roll < x {
                    ResultType::Worst(new_roll)
                }
                else{
                    ResultType::Worst(x)
                }
            },
            &ResultType::Sum(x) => ResultType::Sum(x + new_roll)
        };

        RollLog { reader_id: self.id, new_face: new_roll, new_result: self.current_result.get_num() }
    }

    pub fn get_label(&self) -> &str{
        &self.label
    } 
}

#[derive(Serialize, Deserialize)]
struct RollLog{
    reader_id: usize,
    new_face: u32,
    new_result: u32
}

#[cfg(test)]

#[test]
fn test_roll_log(){
    let mut reader_one = DieReader::new(0, Some("Die 1".to_string()), 1, 6).unwrap();
    let mut reader_two = DieReader::new(1, None, 2, 6).unwrap();

    reader_one.set_result_type(ResultType::Worst(0));

    let label_one = reader_one.get_label().to_string();
    let label_two = reader_two.get_label().to_string();

    for _ in 0..100{
        println!("{} rolled {}", label_one, serde_json::to_string(&reader_one.roll()).unwrap()); 
        println!("{} rolled {}", label_two, serde_json::to_string(&reader_two.roll()).unwrap()); 
    }
}