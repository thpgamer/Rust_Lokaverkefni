mod stolar;

use std::io::Write;

use stolar::Stoll;
fn main() {
    loop {
        print!("Enter: ");
        std::io::stdout().flush().expect("flush náðist ekki");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("náði ekki að lesa");
        let svor: Vec<&str> = input.split_ascii_whitespace().collect();
        match svor.first() {
            Some(svar) =>{
                let low_svar = svar.to_lowercase();
                let final_svar = low_svar.trim();
                match final_svar {
                    "hætta" => break,
                    "stoll" => {
                            if svor.len() < 5{
                                println!("{:?}",svor);
                                let stolar = svor[2];
                                let stadur=svor[1];
                                if let Ok(verd) = svor[3].parse::<i32>() {
                                    match Stoll.new(stadur, stolar, verd) {
                                        Ok(_) => println!("Skrá nýjan stól"),
                                        Err(e) => println!("{}", e),
                                    }
                                }
                                /*
                                if let Some((hus,numer)) = svor[1].split_once("-"){
                                    println!("Hús: {}, Númer: {}", hus,numer);
                                    let haed = &numer[0..1];
                                    let herbergi = &numer[1..];
                                }
                                println!("Hæð: {} Herbergi: {}",haed,herbergi);
                                */
                            }
                    }         


                    "h" | "hjálp" =>{
                        println!("sita inn nýjan stól:");
                        println!("stoll staður:'str' gerð:'sae=sægindastóll, sko=skólastóll, skr= skrifstofustóll, ' '= annað' verð:'int' ");
                        
                        println!("hætta: stopar forrit");
                    }
                    _ => println!("Skill ekki"),
                }
            }
            None => println!("ekkert var sláð inn"),
        }
    }
}
