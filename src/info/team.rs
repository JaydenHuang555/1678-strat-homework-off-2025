
use crate::info::scored_note::ScoredNote;
use crate::info::endgame::EndGameInfo;
#[derive(Clone)]
pub struct Team {
    number: u32,
    notes: Vec<ScoredNote>,
    engame: EndGameInfo
}

impl Team {

    pub fn new(number: u32) -> Self {
        Self {
            number: number,
            notes: Vec::new(),
            engame: EndGameInfo::new()
        }
    }

    pub fn set_number(&mut self, number: u32) {
        self.number = number;
    }

}
