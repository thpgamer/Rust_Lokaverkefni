use crate::stolargerd::Gerd;

pub struct Stoll {
    pub gerd: Gerd,
    pub verd: i32,
    pub stadur: Stadur,
}

impl Stoll {
    pub fn new(impgerd: &str, impverd: i32, impstadur: &str) -> Result<Self, String> {
        let gerd = Gerd::try_from(impgerd)?;
        let stadur = Stadur::try_from(impstadur)?;
        Ok(Self {
            gerd,
            verd: impverd,
            stadur,
        })
    }
}
