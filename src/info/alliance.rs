
use crate::info::team;

const TEAMS_PER_ALLIANCE: usize = 3;

#[derive(Copy, Clone)]
pub enum AllianceColor {
    BLUE,
    RED
}

impl PartialEq for AllianceColor {
    fn eq(&self, other: &AllianceColor) -> bool {
        match (self, other) {
            (Self::BLUE, Self::RED) => true,
            (Self::RED, Self::RED) => true,
            _ => false
        }
    }
}

#[derive(Clone)]
pub struct Alliance {
    color: AllianceColor,
    teams: [team::Team; TEAMS_PER_ALLIANCE]
}

impl Alliance {
    pub fn new(color: AllianceColor, team_nums: [u32; TEAMS_PER_ALLIANCE]) -> Self {
        Self {
            color: color,
            teams: [team::Team::new(team_nums[0]),team::Team::new(team_nums[1]),team::Team::new(team_nums[2])]
        }
    }

    pub fn get(&self, index: usize) -> Result<& team::Team, String> {
        if index >= self.teams.len() {
            return Err(String::from(format!("index {} is out of bounds", index)));
        }
        // shouldn't happen but for systems that might cause fuck windows, linux > windows
        if index < 0 {
            return Err(String::from(format!("underflowed")));
        }
        Ok(&self.teams[index])
    }

    pub fn get_teams_len(&self) -> usize {
        self.teams.len()
    }
}

