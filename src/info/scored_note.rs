#[derive(Copy, Clone)]
pub enum FinalDestination {
    NONE,
    SPEAKER,
    SPEAKERAMPED,
    AMP
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
