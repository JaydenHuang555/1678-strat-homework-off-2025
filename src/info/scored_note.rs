
use crate::info::constants;

#[derive(Copy, Clone)]
pub enum FinalDestination {
    None,
    Speaker,
    SpeakerAmped,
    Amp,
    Trap
}

#[derive(Copy, Clone)]
pub struct ScoredNote {
    pub in_auto: bool,
    pub destination: FinalDestination
}

impl ScoredNote {
    pub fn new() -> Self {
        Self {
            in_auto: false,
            destination: FinalDestination::None
        }
    }

    pub fn val() -> u32 {
        0
    }

}



