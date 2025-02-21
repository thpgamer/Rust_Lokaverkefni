
fn main() {
    let texti = "abc def ghj jkl";
    if let Some((fyrsta_ord,afgangur)) = texti.split_once(" "){
        println!("1. {}, afgangur: {}", fyrsta_ord,afgangur)
    }
    let stadasetning = "H-202";//H 2 2
    if let Some((hus,numer)) = stadasetning.split_once("-"){
        println!("Hús: {}, Númer: {}", hus,numer);
        let haed = &numer[0..1];
        let herbergi = &numer[1..];
        println!("Hæð: {} Herbergi: {}",haed,herbergi);
    }
}
