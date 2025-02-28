use crate::stolar::Stoll;

pub struct Blas {
    svaedi: Vec<Stoll>,
    id: i32,
}

impl Blas {
    pub fn new() -> Self {
        Blas {
            svaedi: Vec::new(),
            id: 0,
        }
    }

    pub fn skra_stol(&mut self, impgerd: &str, impverd: i32, impstadur: &str) -> Result<(), String> {
        let nyr_stoll = Stoll::new(impgerd, impverd, impstadur)?;
        self.svaedi.push(nyr_stoll);
        self.id += 1;
        Ok(())
    }
}
