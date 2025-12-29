use super::dice::{Die, DieResultType};

///Result type for a dice tray.
pub enum TrayResultType {
    Sum,
    Best,
    Worst,
}

impl TrayResultType {
    pub fn to_string(&self) -> String {
        match self {
            TrayResultType::Sum => "Tray sum".to_string(),
            TrayResultType::Best => "High roll in tray".to_string(),
            TrayResultType::Worst => "Worst roll in tray".to_string(),
        }
    }
}

pub enum TrayResult {
    Number(u32),
    String(String),
    None,
}

impl TrayResult {
    pub fn to_string(&self) -> String {
        match self {
            TrayResult::Number(n) => n.to_string(),
            TrayResult::String(s) => s.clone(),
            TrayResult::None => "None".to_string(),
        }
    }
}

///Ways a dice tray can be sorted. Passed to Tray::sort() to organize the dice in the tray.
pub enum TraySortType{
    Face
}

impl TraySortType{
    ///Returns a string explaining how the tray was sorted. Used by the cli logger.
    pub fn to_string(&self) -> String{
        match self {
            TraySortType::Face => "Sorting tray by die face.".to_string()          
        }
    }
    
}

///Trait for a dice tray. Tray's own the refrences to the dice in them.
///This trait provides fuinctions for adding and removing dice from the tray and for getting information about the tray state. 
pub trait Tray{
    ///Adds a single die to the tray.
    fn add_die(&mut self, die: Box<dyn Die>);

    ///Adds all the dice provided to the tray.
    fn add_dice(&mut self, dice: Vec<Box<dyn Die>>);

    ///Removes the die at the specified tray index or returns an error if it isn't available.
    fn remove_die_at(&mut self, index : usize) -> Result<Box<dyn Die>, String>;

    ///Removes the die witht he specified id, throws an error if the ID is not available in the tray.
    fn remove_die_by_id(&mut self, id : usize) -> Result<Box<dyn Die>, String>;

    ///Removes all the dice with the specified label, throws an error if no dice are found.
    fn remove_dice_by_label(&mut self, label : &str) -> Result<Vec<Box<dyn Die>>, String>;

    ///Removes all the dice form the tray and returns them, or throws an error if the tray is empty.
    fn remove_all(&mut self) -> Result<Vec<Box<dyn Die>>, String>;

    ///Clears the tray of all dice. 
    fn clear(&mut self);

    ///Applies the provided result type then rolls all the dice in the tray. 
    fn roll_all(&mut self, result_type: DieResultType);

    ///Rolls the dice at the provided index, using the provided result type. 
    ///Throws an error if no die is present at the index.
    fn roll_at(&mut self, index: usize, result_type: DieResultType) -> Result<(), String>;

    /// Rolls all dice in the tray with the specified label.
    /// Throws an error if no die has the label provided.
    fn roll_by_label(&mut self, label: &str, result_type: DieResultType) -> Result<(), String>;

    ///Reorganizes the dice tray based on the sort type provided.
    fn sort(&mut self, sort_by : TraySortType);

    /// Gets a reffrence to all the dice in the tray.
    fn get_dice(&self) -> &Vec<Box<dyn Die>>;

    /// Gets the result type of the tray
    fn get_result_type(&self) -> &TrayResultType;

    /// Gets the current tray result as a TrayResult enum.
    fn get_result(&self) -> TrayResult;
}



