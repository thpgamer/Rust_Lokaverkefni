mod stolar;
mod storage;
mod stolargerd;

use std::io::Write;
use stolar::Stoll;
use storage::Blas;

fn main() {
    let mut storage = Blas::new(); // Create an instance of Blas

    loop {
        print!("Enter: ");
        std::io::stdout().flush().expect("flush náðist ekki");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("náði ekki að lesa");
        let svor: Vec<&str> = input.split_ascii_whitespace().collect();

        match svor.first() {
            Some(svar) => {
                let final_svar = svar.to_lowercase().trim().to_string();

                match final_svar.as_str() {
                    "hætta" => break,

                    "stoll" => {
                        if svor.len() < 4 {
                            println!("Villa: Ekki nægileg gögn!");
                            continue;
                        }

                        let stadur = svor[1];
                        let stoll = svor[2];

                        if let Ok(verd) = svor[3].parse::<i32>() {
                            
                            if let Some((hus,numer)) = svor[1].split_once("-"){
                                println!("Hús: {}, Númer: {}", hus,numer);
                                let haed = &numer[0..1];
                                let herbergi = &numer[1..];
                                println!("Hæð: {} Herbergi: {}",haed,herbergi);
                                match storage.skra_stol(stoll, verd, stadur) {
                                    Ok(_) => println!("Skráði nýjan stól"),
                                    Err(e) => println!("Villa: {}", e),
                                }
                            }
                                else {
                                    println!("Villa: Verð verður að vera heiltala.");
                                }
                        }
                    }

                    "h" | "hjálp" => {
                        println!("Skrá inn nýjan stól:");
                        println!("stoll staður:'str' gerð:'sae=sægindastóll, sko=skólastóll, skr= skrifstofustóll, ' '= annað' verð:'int' ");
                        println!("hætta: stoppar forrit");
                    }

                    _ => println!("Skill ekki"),
                }
            }
            None => println!("Ekkert var slegið inn."),
        }
    }
}
