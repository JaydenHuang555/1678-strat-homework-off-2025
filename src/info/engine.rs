use crate::info::alliance::AllianceColor;
use crate::info::scored_note;
use crate::info::alliance::Alliance;
use crate::info::team::Team;

pub enum Result {
    WIN,
    LOST,
    TIE
}

pub struct Engine {
    red: Alliance,
    blue: Alliance,
    red_points_norm: u32,
    blue_points_norm: u32,
    red_points_rank: u32,
    blue_points_rank: u32
}

impl Engine {
    pub fn new(red: Alliance, blue: Alliance) -> Self {
        Self {
            red: red,
            blue: blue,
            red_points_norm: 0,
            blue_points_norm: 0,
            blue_points_rank: 0,
            red_points_rank: 0
        }
    }

    pub fn calc_notes(&self, color: AllianceColor) -> u32 {
        let alliance: &Alliance;
        match color {
            AllianceColor::BLUE => {
                alliance = &self.blue;
            }
            AllianceColor::RED => {
                alliance = &self.red;
            }
        }
        let mut total: u32 = 0;
        for i in 0 .. alliance.get_teams_len() {
            match alliance.get(i) {
                None => {
                    eprintln!("unable to get team at index {i}");
                    std::process::exit(1);
                }
                Some(team) => {
                    let notes: Vec<scored_note::ScoredNote> = team.get_notes();
                    for note in notes {
                        total += note.destination.get(note.in_auto);
                    }
                }
            }
        }
        total 
    }

    pub fn calc(&self, color: AllianceColor) -> u32 {
        return       self.calc_notes(AllianceColor::BLUE) + self.calc_notes(AllianceColor::RED)
    }

}

