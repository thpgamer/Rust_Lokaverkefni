use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use crate::equipment::Equipment;
use crate::location::Location;


#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub items: HashMap<u32, Equipment>, 
    pub next_id: u32, 
}

impl Inventory {
    /// býr til tóma skrá
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    /// bætir við nýju efni í skrána
    pub fn add_equipment(&mut self, item: Equipment) {
        let id = self.next_id; // notar næsta id
        self.items.insert(id, item);
        self.next_id += 1; // hækkar id um einn
    }

    /// eyðir efni með ákveðið id
    pub fn remove_equipment(&mut self, id: u32) {
        self.items.remove(&id);
    }

    /// uppfærir staðsetningu efnis
    pub fn update_location(&mut self, id: u32, new_location: Location) {
        if let Some(item) = self.items.get_mut(&id) {
            match item {
                Equipment::Table(t) => t.location = new_location,
                Equipment::Chair(c) => c.location = new_location,
                Equipment::Projector(p) => p.location = new_location,
            }
        }
    }

    /// birtir allt efni í skránni
    pub fn display_all(&self) {
        for (id, item) in &self.items {
            println!("auðkenni: {} - {}", id, item);
        }
    }

    /// birtir efni eftir id
    pub fn display_by_id(&self, id: u32) {
        if let Some(item) = self.items.get(&id) {
            println!("auðkenni: {} - {}", id, item);
        } else {
            println!("efni fannst ekki með þetta auðkenni.");
        }
    }

    /// birtir allt efni í ákveðnu húsi
    pub fn display_by_building(&self, building: &str) {
        for (id, item) in &self.items {
            match item {
                Equipment::Table(t) if t.location.building.to_string() == building => println!("id: {} - {}", id, t),
                Equipment::Chair(c) if c.location.building.to_string() == building => println!("id: {} - {}", id, c),
                Equipment::Projector(p) if p.location.building.to_string() == building => println!("id: {} - {}", id, p),
                _ => {}
            }
        }
    }

    /// vistar json skrá
    pub fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self).expect("mistókst að umbreyta í json");
        let mut file = File::create(filename).expect("mistókst að búa til skrá");
        file.write_all(json.as_bytes()).expect("mistókst að skrifa í skrá");
    }

    // býr til json file ef hún er ekki til
    pub fn load_from_file(filename: &str) -> Self {
        let mut file = OpenOptions::new().read(true).open(filename).unwrap_or_else(|_| File::create(filename).expect("mistókst að búa til skrá"));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("mistókst að lesa skrá");
        serde_json::from_str(&contents).unwrap_or_else(|_| Inventory::new())
    }
}
