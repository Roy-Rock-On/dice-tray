use std::u32;
use std::collections::BTreeMap;
use std::ops::Range;

struct InternalRng {
    seed: u64,
}

impl InternalRng {
    pub fn new(seed: u64) -> Self{
        let mut rng = InternalRng { seed };
        rng.next();
        rng
    }

    fn next(&mut self) -> u32{
        let old_seed = self.seed;
        self.seed = old_seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        let xorshift = (old_seed >> 59) as u32;
        let rot  = (old_seed >> 59) as u32;
        (xorshift >> rot)|(xorshift << ((rot.wrapping_neg()) & 31))
    }

    pub fn get_rng(&mut self) -> u32 {
        self.next() as u32
    }    
}

pub struct Die {
    faces: u32,
    face_weights: Vec<f64>,  // Probabilities that sum to 1.0
    cumulative_ranges: Vec<f64>,  // RNG ranges mapped to face numbers
    current_face: u32,
    internal_rng: InternalRng,
}


#[cfg(test)]
#[test]
fn randomize(){
    let mut rng = InternalRng::new(4);
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
fn max_u32(){
    let digits = u32::MAX.to_string().len();
    println!("{} Digitcount: {}", u32::MAX, digits);
}

#[test]
fn min_u32(){
    let digits = u32::MIN.to_string().len();
    println!("{} Digitcount: {}", u32::MIN, digits);
}



