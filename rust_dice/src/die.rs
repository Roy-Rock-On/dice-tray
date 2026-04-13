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
}

pub struct Die {
    faces: u32,
    current_face: u32,
    internal_rng: InternalRng,
}

impl Die{
    pub fn new(faces: u32, seed: u64) -> Self {
        Die { faces, current_face: 1, internal_rng: InternalRng::new(seed) }
    }

    pub fn roll(&mut self) -> u32 {
        let rng = self.internal_rng.get_rng();
        self.current_face = rng % self.faces + 1;
        self.current_face
    }
}



#[cfg(test)]

#[test]
fn randomize(){
    let mut rng = InternalRng::new(57);
    let mut counts: [i32; 11] = [0,0,0,0,0,0,0,0,0,0,0];
    for i in 0..10000 {
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
fn die_test(){
    let faces = 4;
    let mut die = Die::new(faces, 44);
    let mut counts: Vec<i32> = vec![0; faces as usize];
    for i in 0..1000000 {
        let roll = die.roll();
        counts[(roll - 1) as usize] += 1;
        println!("{} --- {}", i, die.roll())
    }

    let sum_of_counts = {
        let mut sum = 0;
        for i in 0..counts.len(){
            sum += counts[i];
        }
        sum
    };

    println!("Sum of counts = {}", sum_of_counts);
    assert_eq!(sum_of_counts, 1000000);

    let mut percents: Vec<f64> = vec![0.0; faces as usize];
    let mut sum_of_percents = 0.0; 
    for i in 0..faces {
        percents[i as usize] = (counts[i as usize] as f64 / sum_of_counts as f64) * 100.0;
        sum_of_percents += percents[i as usize];
        println!("Face of {} count of {}", i + 1, counts[i as usize]);
        println!("Precent of total {}", percents[i as usize]);
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
fn max_u32(){
    let digits = u32::MAX.to_string().len();
    println!("{} Digitcount: {}", u32::MAX, digits);
}

#[test]
fn min_u32(){
    let digits = u32::MIN.to_string().len();
    println!("{} Digitcount: {}", u32::MIN, digits);
}



