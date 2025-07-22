
#[derive(Copy, Clone)]
pub struct ScoredNote {
    pub in_auto: bool,
    pub speaker: bool,
}

impl ScoredNote {
    pub fn new() -> Self {
        Self {
            in_auto: false,
            speaker: false
        }
    }
}
