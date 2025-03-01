use serde::{Serialize, Deserialize};
use crate::stadir::Stadur;
use std::cmp::Ordering;
use std::fmt;


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Location {
    pub building: Stadur, 
    pub floor: u8,       
    pub room: u16,       
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.building.cmp(&other.building)
            .then_with(|| self.floor.cmp(&other.floor))
            .then_with(|| self.room.cmp(&other.room))
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {}-{}{:02}", self.building, self.floor, self.room)
    }
}
