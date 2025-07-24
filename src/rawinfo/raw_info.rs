
#[derive(serde::Deserialize)]
pub struct RawInfo {
    pub alliances: Vec<RawAlliance>
}

#[derive(serde::Deserialize)]
pub struct RawAlliance {
    pub alliance: String,
    pub teams: Vec<Team>,
}

#[derive(serde::Deserialize)]
pub struct Team {
    pub number: u32,
    pub auto: Auto,
    pub tele: Tele,
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
