// projector.rs
use serde::{Serialize, Deserialize};
use crate::location::Location;
use std::fmt;

/// Struct sem lýsir skjávarpa
#[derive(Debug, Serialize, Deserialize)]
pub struct Projector {
    pub id: u32,          
    pub value: u32,       
    pub location: Location, 
    pub lumens: u32,      
}

impl fmt::Display for Projector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Skjávarpi: ID={} | {} kr | {} lumens | Staðsetning: {}", 
            self.id, self.value, self.lumens, self.location)
    }
}
