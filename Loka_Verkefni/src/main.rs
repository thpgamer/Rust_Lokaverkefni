use std::io::Write;
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
                    "new" => {
                        if let Some((fyrsta_ord,afgangur)) = input.split_once(" "){
                            if let Some((hus,numer)) = afgangur.split_once("-"){
                                println!("Hús: {}, Númer: {}", hus,numer);
                                let haed = &numer[0..1];
                                let herbergi = &numer[1..];
                                println!("Hæð: {} Herbergi: {}",haed,herbergi);
                            }
                        }
                    }         



                    _ => println!("Skill ekki"),
                }
            }
            None => println!("ekkert var sláð inn"),
        }
    }
}
