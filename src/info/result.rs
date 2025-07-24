
pub struct Result {
    points: u32,
    rank: u32
}

impl Result {
    pub fn new() -> Self {
        Self {
            points: 0,
            rank: 0
        }
    }
}
