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
    pub fn new(red_nums: [u32; 3], blue_nums: [u32; 3]) -> Self {
        let mut red_teams: [Team; 3] = [Team::new(0), Team::new(0), Team::new(0)]; 
        let mut blue_teams: [Team; 3] = [Team::new(0), Team::new(0), Team::new(0)];
        let mut saved: [Option<u32>; 6] = [Option::None; 6];
        let mut saved_last = 0;
        for i in 0..red_nums.len() {
            for num in &saved {
                match num {
                    None => {
                        continue;
                    }
                    Some(n) => {
                        if *n == red_nums[i] || *n == blue_nums[i] {
                            eprintln!("error, found same team");
                        }
                    }
                }
            }
            red_teams[i] = Team::new(red_nums[i]);
            blue_teams[i] = Team::new(blue_nums[i]);
        }

        Self {
            red: Alliance::new(AllianceColor::RED, red_teams),
            blue: Alliance::new(AllianceColor::BLUE, blue_teams),
            red_points_norm: 0,
            blue_points_norm: 0,
            red_points_rank: 0,
            blue_points_rank: 0
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

