use serde::{Serialize, Deserialize};
use std::fmt;


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stadur {
    HA, 
    H,  
    S,  
}

impl fmt::Display for Stadur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stadur::HA => write!(f, "Hafnarfjörður"),
            Stadur::H => write!(f, "Háteigsvegur"),
            Stadur::S => write!(f, "Skólavörðuholt"),
        }
    }
}