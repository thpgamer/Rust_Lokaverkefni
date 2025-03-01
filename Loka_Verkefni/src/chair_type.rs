use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChairType {
    Lounge,      
    School,       
    Office,      
    Other(String) 
}

impl fmt::Display for ChairType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChairType::Lounge => write!(f, "Hægindastóll"),
            ChairType::School => write!(f, "Skólastóll"),
            ChairType::Office => write!(f, "Skrifstofustóll"),
            ChairType::Other(desc) => write!(f, "Annað: {}", desc),
        }
    }
}
