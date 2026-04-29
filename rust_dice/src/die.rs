use std::fmt;
use std::u32;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct InternalRng {
    seed: u64,
}

impl InternalRng {
    ///Creates a new internal RNG using the given seed.
    ///Internal RNG uses a
    pub fn new(seed: u64) -> Self{
        let mut rng = InternalRng { seed };
        rng.next();
        rng
    }  
    
    ///Returns the current seed. Used when serializing the die to DieData.
    pub fn get_current_seed(&self) -> u64{
        self.seed
    } 

    ///Gets a random number from 0 (inclusive) to the max value provided (exclusive)
    ///Used to randomize the face weights during dice creation. 
    pub fn get_number(&mut self, max: u32) -> u32{
        let seed = self.next();
        (seed % max) as u32
    }  

    fn next(&mut self) -> u32 {
        let old_seed = self.seed;

        // PCG32 - I don't understand it but it works...
        self.seed = old_seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        let xorshifted = (((old_seed >> 18) ^ old_seed) >> 27) as u32;
        let rot = (old_seed >> 59) as u32;

        xorshifted.rotate_right(rot)
    }
}

///A representation of a physical die. 
///Uses Internal RNG (PCG32) and a vector of face weights to simulate imperfections.
///Can be serialized into dice data, along with its seed. 
///Can be created from dice data, or directly using build_die().  
pub struct Die {
    id: usize,
    label: String,
    current_face: u32,
    result_type: DieResultType,
    current_result: DieResult,
    face_weights: Vec<u32>,
    total_weight: u32,
    internal_rng: InternalRng,
    current_tray: Option<usize>
}

impl Die{
    ///Rolls the Die and makes 
    pub fn roll(&mut self) -> RollLog {
        let last_face = self.current_face;
        let last_result = self.current_result.clone();
        let random_number = self.internal_rng.get_number(self.total_weight);
        for i in 0..self.face_weights.len(){
            if random_number <= self.face_weights[i] {
                self.current_face = (i + 1) as u32;
                break;
            } 
        }
        let face_result = self.current_face;
        self.set_die_result(face_result);

        RollLog {
            die_id: self.id,
            die_label: self.label.clone(),
            old_face: last_face,
            new_face: face_result,
            old_result: last_result,
            new_result: self.current_result.clone(),
            tray_update: self.current_tray.clone()
        }
    }

    ///Gets the current result of the dice without modification of the rng.
    pub fn get_current_face(&self) -> u32 {
        self.current_face
    }

    /// Gets the total face count for the die.
    pub fn get_face_count(&self) -> u32{
        self.face_weights.len() as u32
    }

    /// Sets the result type for a die.
    fn set_result_type(&mut self, new_type: DieResultType){
        let current_result = self.current_face;
        self.current_result = match new_type{
            DieResultType::Face => DieResult::Face(current_result),
            DieResultType::Best => DieResult::Best(current_result),
            DieResultType::Sum => DieResult::Sum(current_result),
            DieResultType::Worst => DieResult::Worst(current_result)
        };
    }

    /// Sets the in_tray to track what tray the die is in. Used by the dice allocator.
    pub fn set_tray(&mut self, tray_id: Option<usize>){
        self.current_tray = tray_id;
    }

    ///Returns the dies current tray by id. Or None.
    pub fn get_tray_id(&self) -> Option<usize>{
        self.current_tray
    }

    /// Returns the die ID, given by the die allocator at generation (or oterwise when the die is created). 
    /// This is session dependenant and is not saved when the die is serialized.
    pub fn get_id(&self) -> usize{
        self.id
    }

    /// Returns string slice containing the die label.
    pub fn get_label(&self) -> &str{
        &self.label
    }

    ///Convert the die to die data so it can be serialized to JSON using Serde.
    pub fn to_data(&self) -> DieData {
        DieData{
            label: self.label.clone(),
            current_face: self.current_face,
            current_result: self.current_result.clone(),
            result_type: self.result_type.clone(),
            face_weights: self.face_weights.clone(),
            total_weight: self.total_weight,
            last_rng_seed: self.internal_rng.get_current_seed(),
            last_tray: self.current_tray
        }
    } 

    ///Convert DieData into a die that can be rolled.
    pub fn from_data(die_data: DieData, die_id: usize) -> Die{   
        Die { 
            id: die_id,
            label: die_data.label,
            current_face: die_data.current_face,
            result_type: die_data.result_type,
            current_result: die_data.current_result,
            face_weights: die_data.face_weights,
            total_weight: die_data.total_weight,
            internal_rng: InternalRng::new(die_data.last_rng_seed),
            current_tray: None
        }
    } 

    ///Returns a summary snapshot of the die without excess infroamtion.
    pub fn to_summary(&self) -> DieSummary<'_>{
        DieSummary { 
            die_id: self.id,
            die_label: &self.label,
            total_faces: self.face_weights.len() as u32,
            current_face: self.get_current_face(),
            result: self.current_result.clone() 
        }
    }

    // Private functions that run internal dice logic.
    fn set_die_result(&mut self, face_result: u32){
        let current_result_num = match self.current_result.get_num(){
            Ok(num) => num,
            Err(_) => 0
        };

        self.current_result = match &self.result_type{
            DieResultType::Face => DieResult::Face(face_result),
            DieResultType::Best => {
                if face_result > current_result_num{
                    DieResult::Best(face_result)
                }
                else{
                    DieResult::Best(current_result_num)
                }
            },
            DieResultType::Worst => {
                if face_result < current_result_num{
                    DieResult::Worst(face_result)
                }
                else{
                    DieResult::Worst(current_result_num)
                }
            },
            DieResultType::Sum => DieResult::Sum(current_result_num + face_result)
        }
    }

    fn new(id: usize, die_label: Option<String>, faces: u32, seed: u64, std_weight: u32, weight_varience: u32) -> Self {
        let face_weights = Self::map_face_weights(seed, faces, std_weight, weight_varience);

        let label = match die_label{
            Some(l) => l.to_string(),
            None => format!("d{}", faces)
        };

        Die { 
            id,
            label,
            current_face: 1,
            result_type: DieResultType::Face,
            current_result: DieResult::Face(1),
            face_weights : face_weights.0,
            total_weight : face_weights.1,
            internal_rng: InternalRng::new(seed),
            current_tray: None
        }
    }

    fn map_face_weights(seed: u64, face_count: u32, std_weight: u32, weight_varience: u32) -> (Vec<u32>, u32){
        let mut rng = InternalRng::new(seed);
        let mut weights = Vec::new();
        let mut cumulative = 0;
        for _ in 0..face_count{
            cumulative += (std_weight + rng.get_number(weight_varience));
            weights.push(cumulative);
        }

        (weights, cumulative)
    }
}

impl fmt::Display for Die{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let result_num = self.current_result.get_num().unwrap_or(0);
        write!(
            f,
            "{}[id = {}, faces = d{}, current_face = {}, result = {}, current_tray = {:?}]",
            self.label,
            self.id,
            self.face_weights.len(),
            self.current_face,
            result_num,
            self.current_tray
        )
    }
}

///Builds and retuns a new die from the provided seed. 
///Each face of the die has a base weight of std_weight + (0 - weight_varience). 
///Higher varience will create more "unfair" dice.
///Once dice face weights are set they will persist for the lifetime of the die. 
pub fn build_die(id: usize, label: Option<String>, faces: u32, seed: u64, std_weight: u32, weight_varience: u32) -> Result<Die, String>{
    if faces > 1000{
        return Err(format!("Error: cannot create a die with over a thousand faces."));
    }
    else if std_weight <= 0 {
        return Err(format!("Standard die weight must be greater than zero, recomend a value of 100."));
    }
    else if weight_varience <= 0 {
        return Err(format!("Weight varience of new dice must be greater than zero, recomend a value of 25."));
    }
    let mut die = Die::new(id, label, faces, seed, std_weight, weight_varience);
    die.roll();
    Ok(die)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum DieResultType{
    Face,
    Best,
    Worst,
    Sum
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DieResult{
    Face(u32),
    Best(u32),
    Worst(u32),
    Sum(u32)
}

impl DieResult{
    pub fn get_num(&self) -> Result<u32, String> {
        match self {
            DieResult::Best(x) => return Ok(*x),
            DieResult::Face(x) => return Ok(*x),
            DieResult::Sum(x) => return Ok(*x),
            DieResult::Worst(x) => return Ok(*x),
            _ => Err(format!("Cannot cast DieResult {:?} as number.", self))
        }
    }

    pub fn to_string(&self) -> String {
        match self{
            DieResult::Best(x) => format!("Best = {}", x),
            DieResult::Face(x) => format!("Face = {}", x),
            DieResult::Worst(x) => format!("Worst = {}", x),
            DieResult::Sum(x) => format!("Sum = {}", x)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RollLog{
    die_id: usize,
    die_label: String,
    old_face: u32,
    new_face: u32,
    old_result: DieResult,
    new_result: DieResult,
    tray_update: Option<usize>
}

#[derive(Serialize, Deserialize)]
///A summary of a given die, used by trays to sort. Or passed to applicaitons to show dice with minimum information. 
pub struct DieSummary<'a>{
    pub die_id: usize,
    pub die_label: &'a str,
    pub total_faces: u32,
    pub current_face: u32,
    pub result: DieResult
}


impl<'a> PartialEq for DieSummary<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.total_faces == other.total_faces
            && self.current_face == other.current_face
            && self.result.get_num().unwrap_or(0) == other.result.get_num().unwrap_or(0)
            && self.die_id == other.die_id
    }
}

impl<'a> Eq for DieSummary<'a> {}

impl<'a> PartialOrd for DieSummary<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for DieSummary<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_faces
            .cmp(&other.total_faces)
            .then_with(|| self.current_face.cmp(&other.current_face))
            .then_with(|| self.result.get_num().unwrap_or(0).cmp(&other.result.get_num().unwrap_or(0)))
            .then_with(|| self.die_id.cmp(&other.die_id))
    }
}

#[derive(Serialize, Deserialize)]
pub struct DieData{
    label: String,
    current_face: u32,
    result_type: DieResultType,
    current_result: DieResult,
    face_weights: Vec<u32>,
    total_weight: u32,
    last_rng_seed: u64,
    last_tray: Option<usize>
}

#[derive(Serialize, Deserialize)]
pub struct DiceDataList{
    pub file_name: String,
    pub dice_data_vec: Vec<DieData>
}

impl DiceDataList{
    ///Creates a new dice data list for serilization into JSON.
    ///Lets dice be saved between sessions. 
    pub fn new(file_name: String) -> Self{
        DiceDataList { 
            file_name, 
            dice_data_vec: Vec::new() 
        }
    }

    ///Allows for adding data to a dice data list. 
    ///Used to iterate through a series of dice for serilization.
    pub fn add_data(&mut self, die_data: DieData){
        self.dice_data_vec.push(die_data);
    }
}

#[cfg(test)]

#[test]
fn random_number(){
    let mut rng = InternalRng::new(222);
    
    println!("Getting random numbers 0 - 5");
    for _ in 0..100{
        println!("{}", rng.get_number(6));
    }

    println!("Getting random numbers 0 - 19");
    for _ in 0..100{
        println!("{}", rng.get_number(20));
    }
}

#[test]
fn die_test(){
    let faces = 10;
    let mut die = build_die(faces, Some("new die".to_string()), 71, 100, 50, 10).unwrap();
    let mut counts: Vec<i32> = vec![0; faces as usize];
    for i in 0..100000 {
        let roll = die.roll();
        counts[(roll.new_face - 1) as usize] += 1;
        println!("{} --- {}", i, roll.new_result.get_num().unwrap());
    }

    let sum_of_counts = {
        let mut sum = 0;
        for i in 0..counts.len(){
            sum += counts[i];
        }
        sum
    };

    let mut percents: Vec<f64> = vec![0.0; faces as usize];
    let mut sum_of_percents = 0.0; 
    for i in 0..faces {
        percents[i as usize] = (counts[i as usize] as f64 / sum_of_counts as f64) * 100.0;
        sum_of_percents += percents[i as usize];
        println!("Face of {} count of {} percent of {}", i + 1, counts[i as usize], percents[i as usize]);
    }

    let mean = sum_of_percents as f64 / counts.len() as f64;
    let variance = percents.iter()
        .map(|&percent| {
            let diff = percent as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / counts.len() as f64;
    let std_dev = variance.sqrt();
    println!("Standard deviation: {}", std_dev);
}


#[test]
fn face_weights_mapping(){
    let die = Die::new(0, None, 10, 1, 100, 10);
    println!{"Face weights = {:?} Total = {}", die.face_weights, die.total_weight};
}

#[test]
fn test_serialize_die(){
    let mut die = build_die(20, None, 6, 100, 25, 25).unwrap();
    {
        let die_json = serde_json::to_string(&die.to_data()).unwrap();
        println!("JSON die = {}", die_json);
    }
    die.roll();
    {
        let die_json = serde_json::to_string(&die.to_data()).unwrap();
        println!("JSON die = {}", die_json);
    }
    die.roll();
    {
        let die_json = serde_json::to_string(&die.to_data()).unwrap();
        println!("JSON die = {}", die_json);
    }
}

#[test]
fn test_create_die_data() -> Result<(), std::io::Error>{
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use dotenv;

    dotenv::from_filename("rust_dice/src/.env").unwrap();

    let rel = PathBuf::from(std::env::var("DICE_DATA_PATH").unwrap());
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    fs::create_dir_all(&data_dir)?;

    println!("Data dir this = {:?}", data_dir);
    println!("exists: {}, is_dir: {}", data_dir.exists(), data_dir.is_dir());

    let mut dice_data_list = DiceDataList::new("die_test".to_string());
    let mut die_zero = build_die(0, Some("Die_Zero".to_string()), 4, 10, 100, 1).unwrap();
    die_zero.roll();
    let mut die_one = build_die(1, Some("Die_One".to_string()), 6, 20, 100, 25).unwrap();
    die_one.roll();
    let mut die_two = build_die(2, Some("Die_Two".to_string()), 8, 30, 100, 50).unwrap();
    die_two.roll();
    let mut die_three = build_die(3, Some("Die_Three".to_string()), 10, 40, 100, 100).unwrap();
    die_three.roll();
    let mut die_four = build_die(4, Some("Die_Four".to_string()), 12, 50, 100, 200).unwrap();
    die_four.roll();
    let mut die_five = build_die(5, Some("Die_Five".to_string()), 20, 60, 100, 300).unwrap();
    die_five.roll();
    let mut die_six = build_die(6, Some("Die_Six".to_string()), 100, 70, 100, 500).unwrap();
    die_six.roll();

    dice_data_list.add_data(die_zero.to_data());
    dice_data_list.add_data(die_one.to_data());
    dice_data_list.add_data(die_two.to_data());
    dice_data_list.add_data(die_three.to_data());
    dice_data_list.add_data(die_four.to_data());
    dice_data_list.add_data(die_five.to_data());
    dice_data_list.add_data(die_six.to_data());

    let file_path = data_dir.join(dice_data_list.file_name.clone());
    let die_list_json = serde_json::to_string_pretty(&dice_data_list)?;

    let mut test_file = fs::File::create(&file_path)?;
    test_file.write_all(die_list_json.as_bytes())
}

#[test]
fn test_die_data_list_to_dice() -> Result<(), std::io::Error> {
    use std::fs;
    use std::path::PathBuf;
    use dotenv;

    dotenv::from_filename("rust_dice/src/.env").unwrap();

    let rel = PathBuf::from(std::env::var("DICE_DATA_PATH").unwrap());
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);

    println!("Data dir this = {:?}", data_dir);
    println!("exists: {}, is_dir: {}", data_dir.exists(), data_dir.is_dir());

    let file_path = data_dir.join("die_test");
    let die_file = fs::read_to_string(&file_path)?;

    let decoded_list: DiceDataList = serde_json::from_str(&die_file)?;

    let mut restored_dice: Vec<Die> = decoded_list
        .dice_data_vec
        .into_iter()
        .enumerate()
        .map(|(i, data)| Die::from_data(data, i))
        .collect();

    for d in restored_dice.iter_mut(){
        println!("{}", d);
        d.roll();
        println!("{}", d);
    }

    Ok(())
}



