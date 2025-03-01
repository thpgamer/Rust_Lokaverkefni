use serde::{Serialize, Deserialize};
use crate::table::Table;
use crate::chair::Chair;
use crate::projector::Projector;
use std::fmt;
#[derive(Debug, Serialize, Deserialize)]
pub enum Equipment {
    Table(Table),      
    Chair(Chair),       
    Projector(Projector) 
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Equipment::Table(t) => write!(f, "{}", t),
            Equipment::Chair(c) => write!(f, "{}", c),
            Equipment::Projector(p) => write!(f, "{}", p),
        }
    }
}