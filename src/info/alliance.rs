
use crate::info::team;

const TEAMS_PER_ALLIANCE: usize = 3;

pub enum AllianceColor {
    BLUE,
    RED
}

pub struct Alliance {
    color: AllianceColor,
    teams: [team::Team; TEAMS_PER_ALLIANCE]
}

impl Alliance {
    pub fn new(color: AllianceColor, teams: [team::Team; TEAMS_PER_ALLIANCE]) -> Self {
        Self {
            color: color,
            teams:teams
        }
    }

    pub fn get(&self, index: usize) -> Option<& team::Team> {
        if index < self.teams.len() {
            return None;
        }
        Some(&self.teams[index])
    }

    pub fn get_teams_len(&self) -> usize {
        self.teams.len()
    }

}

