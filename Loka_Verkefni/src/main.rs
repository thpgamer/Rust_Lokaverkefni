// src/main.rs
// Aðalforrit sem sér um notendaviðmót og samskipti við birgðaskrá

use std::io;
mod table;
mod chair;
mod projector;
mod chair_type;
mod equipment;
mod inventory;
mod stadir;
mod location;

use inventory::Inventory;
use equipment::Equipment;
use stadir::Stadur;
use location::Location;
use table::Table;
use chair::Chair;
use projector::Projector;
use chair_type::ChairType;

/// Fall sem parsar innsláttarnotanda yfir í staðsetningu (bygging, hæð, herbergi)
fn parse_location(input: &str) -> (Stadur, u8, u16) {
    let input = input.to_uppercase();
    if input.len() < 3 {
        println!("Ógilt staðsetningaform: {}", input);
        return (Stadur::H, 0, 0);
    }
    
    let building = match &input[..1] {
        "H" => Stadur::H,
        "S" => Stadur::S,
        "A" if &input[..2] == "HA" => Stadur::HA,
        _ => {
            println!("Ógild byggingarkóði: {}", input);
            return (Stadur::H, 0, 0);
        }
    };
    
    let floor: u8 = input.chars().nth(input.len() - 3)
        .and_then(|c| c.to_digit(10))
        .map(|d| d as u8)
        .unwrap_or(0);

    let room: u16 = input[input.len() - 2..].parse().unwrap_or(0);
    
    (building, floor, room)
}

/// Aðalforritið sem heldur utan um skipanir notanda
fn main() {
    let mut inventory = Inventory::load_from_file("inventory.json");
    
    loop {
        println!("Sláðu inn skipun: (baeta, eyda, uppfaera, birta, haetta, help)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Mistókst að lesa innslátt");
        let args: Vec<&str> = input.trim().split_whitespace().collect();
        
        if args.is_empty() {
            continue;
        }
        
        match args[0] {
            "help" => {
                println!("\nHjálp - Skipanir í forritinu:");
                println!("  baeta bord <verd:u32> <bygging:HA/H/S-hæð+herbergi:str> <saeti:u8>");
                println!("  baeta stoll <verd:u32> <bygging:HA/H/S-hæð+herbergi:str> <gerd:HS/SS/SO/AN>");
                println!("  baeta skjavarpi <verd:u32> <bygging:HA/H/S-hæð+herbergi:str> <lumens:u32>");
                println!("  eyda <id:u32> - Eyða búnaði með ákveðið auðkenni");
                println!("  uppfaera <id:u32> <bygging:HA/H/S-hæð+herbergi:str>");
                println!("  birta allt - Birta allan búnað í skrá");
                println!("  birta id <id:u32> - Birta ákveðinn búnað");
                println!("  birta hus <bygging:HA/H/S> - Birta allan búnað í ákveðnu húsi");
                println!("  haetta - Hætta í forritinu");
            }
            "baeta" if args.len() >= 3 => {
                let value: u32 = args[2].parse().unwrap_or_else(|_| {
                    println!("Ógilt verð");
                    return 0;
                });

                if value == 0 {
                    continue;
                }

                let (building, floor, room) = parse_location(args[3]);
                let id = inventory.next_id;
                inventory.next_id += 1;

                match args[1] {
                    "bord" if args.len() == 5 => {
                        if let Ok(seats) = args[4].parse() {
                            inventory.add_equipment(Equipment::Table(Table {
                                id,
                                value,
                                location: Location { building, floor, room },
                                seats,
                            }));
                        }
                    }
                    "stoll" if args.len() == 5 => {
                        let chair_type = match args[4] {
                            "HS" => ChairType::Lounge,
                            "SS" => ChairType::School,
                            "SO" => ChairType::Office,
                            _ => ChairType::Other(args[4].to_string()),
                        };

                        inventory.add_equipment(Equipment::Chair(Chair {
                            id,
                            value,
                            location: Location { building, floor, room },
                            chair_type,
                        }));
                    }
                    "skjavarpi" if args.len() == 5 => {
                        if let Ok(lumens) = args[4].parse() {
                            inventory.add_equipment(Equipment::Projector(Projector {
                                id,
                                value,
                                location: Location { building, floor, room },
                                lumens,
                            }));
                        }
                    }
                    _ => println!("Ógild tegund búnaðar eða vantar gögn."),
                }
            }
            "eyda" if args.len() == 2 => {
                if let Ok(id) = args[1].parse() {
                    inventory.remove_equipment(id);
                }
            }
            "uppfaera" if args.len() == 3 => {
                let (building, floor, room) = parse_location(args[2]);
                if let Ok(id) = args[1].parse() {
                    inventory.update_location(id, Location { building, floor, room });
                }
            }
            "birta" => {
                match args.get(1) {
                    Some(&"allt") => inventory.display_all(),
                    Some(&"id") if args.len() == 3 => {
                        if let Ok(id) = args[2].parse() {
                            inventory.display_by_id(id);
                        }
                    }
                    Some(&"hus") if args.len() == 3 => inventory.display_by_building(args[2]),
                    _ => println!("Ógild birtingarskipun."),
                }
            }
            "haetta" => break,
            _ => println!("Ógild skipun. Skrifaðu 'help' fyrir lista af skipunum."),
        }
        
        inventory.save_to_file("inventory.json");
    }
}
