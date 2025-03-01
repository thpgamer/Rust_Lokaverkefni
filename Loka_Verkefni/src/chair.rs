use serde::{Serialize, Deserialize};
use crate::location::Location;
use crate::chair_type::ChairType;
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
pub struct Chair {
    pub id: u32,         
    pub value: u32,       
    pub location: Location, 
    pub chair_type: ChairType, 
}


impl fmt::Display for Chair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stóll: ID={} | {} kr | Tegund: {} | Staðsetning: {}", 
            self.id, self.value, self.chair_type, self.location)
    }
}
