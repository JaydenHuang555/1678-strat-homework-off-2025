
trait RetrieveVal {
    fn val(&self) -> u32;
}

pub enum AutoValues {
    SPEAKER,
    AMP
}

impl AutoValues {
    pub fn val(&self) -> u32 {
        match self {
            Self::SPEAKER => 5,
            Self::AMP => 2
        }
    }
}

pub enum TeleValues {
    AMP,
    SPEAKER_UNAMP,
    SPEAKER_AMP
}

impl TeleValues {
    pub fn val(&self) -> u32 {
        match self {
            Self::AMP => 1,
            Self::SPEAKER_UNAMP => 2,
            Self::SPEAKER_AMP => 5
        }
    }
}

pub enum RankingPointValues {
    ENSEMBLE,
    MELODY,
    COOP
}

impl RankingPointValues {
    pub fn val(&self) -> u32 {
        match self {
            Self::ENSEMBLE => 1,
            Self::MELODY => 1,
            Self::COOP => 0
        }
    }
}

