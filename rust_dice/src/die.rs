use std::u32;


struct InternalRng {
    seed: u64,
}

impl InternalRng {
    pub fn new(seed: u64) -> Self{
        let mut rng = InternalRng { seed };
        rng.next();
        rng
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

    pub fn get_rng(&mut self) -> u32 {
        self.next() as u32
    }

    pub fn get_number(&mut self, max: u32) -> u32{
        let seed = self.next();
        (seed % max) + 1 as u32
    }    
}

pub struct Die {
    current_face: u32,
    face_weights: Vec<u32>,
    total_weight: u32,
    internal_rng: InternalRng,
}

impl Die{
    pub fn new(faces: u32, seed: u64, std_weight: u32, weight_varience: u32) -> Self {
        let face_weights = map_face_weights(seed, faces, std_weight, weight_varience);
        Die { 
            current_face: 1,
            face_weights : face_weights.0,
            total_weight : face_weights.1,
            internal_rng: InternalRng::new(seed)
        }
    }

    pub fn roll(&mut self) -> u32 {
        let random_number = self.internal_rng.get_number(self.total_weight);
        for i in 0..self.face_weights.len(){
            if random_number <= self.face_weights[i] {
                self.current_face = (i + 1) as u32;
                break;
            } 
        }
        self.current_face
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

#[cfg(test)]

#[test]
fn randomize(){
    let mut rng = InternalRng::new(57);
    let mut counts: [i32; 11] = [0,0,0,0,0,0,0,0,0,0,0];
    for i in 0..1000 {
        let number = rng.get_rng();
        let digitcount = number.to_string().len();
        counts[digitcount] += 1;
        println!("{} --- {} --- {}", i, number, digitcount);
    }
    for i in 1..=10{
        println!("digit count count {} -- {}", i, counts[i]);
    }
}

#[test]
fn random_number(){
    let mut rng = InternalRng::new(222);
    
    println!("Getting random numbers 1 - 6");
    for _ in 0..100{
        println!("{}", rng.get_number(6));
    }

    println!("Getting random numbers 1 - 20");
    for _ in 0..100{
        println!("{}", rng.get_number(20));
    }
}

#[test]
fn die_test(){
    let faces = 10;
    let mut die = Die::new(faces, 1789, 100, 100);
    let mut counts: Vec<i32> = vec![0; faces as usize];
    for i in 0..1000000 {
        let roll = die.roll();
        counts[(roll - 1) as usize] += 1;
        println!("{} --- {}", i, roll);
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
    let die = Die::new(10, 1, 100, 10);
    println!{"Face weights = {:?} Total = {}", die.face_weights, die.total_weight};
}



