///A dice_profile is a template that can be passed to Die::new() in order to create a die. To goal is for dice_profiles to support saving and laodind form JSON.

pub struct DieProfile {
    pub label: Option<String>,
    pub die_type: DieType
}

/// DieType is used to specify what kind of die the die_profile should build. Can be extended with more die types later. 
pub enum DieType{
    Numerical(u32),
    Custom
}


impl DieProfile{
    ///Creates and returns a new dice profile that can be used to create new dice. 
    pub fn new(label: Option<String>, die_type : DieType) -> Self {
        DieProfile{
            label,
            die_type   
        }
    }
}