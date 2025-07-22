#[derive(Copy, Clone)]
pub struct EndGameInfo {
    pub parked: bool,
    pub onstage_spotlit: bool,
    pub onstage_non_spotlit: bool,
    pub harmony: bool,
    pub trap: bool
}

impl EndGameInfo {
    pub fn new() -> Self {
        Self {
            parked: false,
            onstage_spotlit: false,
            onstage_non_spotlit:false,
            harmony: false,
            trap: false
        }
    }
}

