use std::fmt::Display;

pub enum Gerd{
    sægindastóll,
    skólastóll,
    skrifstofustóll,
    annað
}
impl TryFrom<&str> for Gerd {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "sae" => Ok(Self::sægindastóll),
            "sko" => Ok(Self::skólastóll),
            "skr" => Ok(Self::skrifstofustóll),
            _ => Ok(Self::annað)
        }
    }
}
impl Display for Gerd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gerd::sægindastóll => write!(f, "Sægindastóll"),
            Gerd::skólastóll => write!(f, "Skólastóll"),
            Gerd::skrifstofustóll => write!(f, "Skrifstofustóll"),
            Gerd::annað => write!(f, "annað"),
        }
    }
}