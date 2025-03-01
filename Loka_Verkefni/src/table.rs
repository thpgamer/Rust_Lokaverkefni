// table.rs
use serde::{Serialize, Deserialize};
use crate::location::Location;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub id: u32,     
    pub value: u32,    
    pub location: Location,
    pub seats: u8,    
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Borð: ID={} | {} kr | {} sæti | Staðsetning: {}", 
            self.id, self.value, self.seats, self.location)
    }
}
