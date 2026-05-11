use std::fmt::{Formatter, Display};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum DiceTargets {
    All,
    Index(Vec<usize>),
    Label(Vec<String>),
}

impl Display for DiceTargets{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceTargets::All => write!(f, "all"),
            DiceTargets::Index(indices) => write!(f, "indices={:?}", indices),
            DiceTargets::Label(labels) => write!(f, "label=\"{:?}\"", labels),
        }
    }
}