use::std::fmt::Display;
use crate::stolar_gerd::Gerd;

pub struct Stoll{
    pub gerd: Gerd,
    pub verd: i32,
    pub stadur: String
}
impl Stoll{
    pub fn new(impgerd:str,impverd:i32,impstadur:str){
        Self{
            gerd: Gerd::try_from(impgerd)?,
            verd: i32,
            stadur: impstadur.to_string()
        }
    }
}