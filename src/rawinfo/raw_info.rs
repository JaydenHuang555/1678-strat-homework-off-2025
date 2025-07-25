
#[derive(serde::Deserialize)]
pub struct RawInfo {
    pub coop: bool,
    pub alliances: Vec<RawAlliance>
}

#[derive(serde::Deserialize)]
pub struct RawAlliance {
    pub alliance: String,
    pub teams: Vec<Team>,
}


impl std::fmt::Display for RawAlliance {
    fn fmt(&self, formater: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formater, "{}", self.alliance)
    }    
}

#[derive(serde::Deserialize)]
pub struct RawEndGame {
    pub park: bool,
    pub onstage: bool,
    pub onstage_spotlit: bool,
    pub trap: bool
}

#[derive(serde::Deserialize)]
pub struct Team {
    pub number: u32,
    pub auto: Auto,
    pub tele: Tele,
    pub endgame : RawEndGame
}

#[derive(serde::Deserialize)]
pub struct Auto {
    pub speaker: u32,
    pub amp: u32,
}

#[derive(serde::Deserialize)]
pub struct Tele {
    pub speaker: u32,
    pub speaker_amp: u32,
    pub amp: u32,
}
