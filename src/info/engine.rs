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

    pub fn calc_notes_for_team(&mut self, alliance: Alliance) -> u32 {
        for i in 0 .. alliance.get_teams_len() {
            match alliance.get(i) {
                Ok(team) => {
                    for note in team.get_notes().iter() {
                          
                    }
                }
                Err(e) =>{
                    panic!("unable to calc how many notes for an team due to {}", e);
                }
            }
        }
        0
    }

    pub fn update_notes_norm(&mut self) {
        self.blue_points_norm = self.calc_notes_for_team(self.blue.clone());
        self.red_points_norm = self.calc_notes_for_team(self.red.clone());
    }

    pub fn update(&mut self) {

    }

}

