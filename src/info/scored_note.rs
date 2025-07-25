
use crate::info::constants;

#[derive(Copy, Clone)]
pub enum FinalDestination {
    NONE,
    SPEAKER,
    SPEAKERAMPED,
    AMP
}

impl FinalDestination {
    pub fn get(&self, in_auto: bool) -> u32 {
        match self {
            FinalDestination::NONE => 0,
            FinalDestination::AMP => {
                if in_auto {
                    return constants::AutoValues::AMP.val();
                }
                return constants::TeleValues::AMP.val();
            }
            FinalDestination::SPEAKER => {
                if in_auto {
                    return constants::AutoValues::SPEAKER.val();
                }
                return constants::TeleValues::SPEAKER_UNAMP.val();
            }
            FinalDestination::SPEAKERAMPED => constants::TeleValues::SPEAKER_AMP.val()
        }
    }
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
            destination: FinalDestination::NONE
        }
    }
}



