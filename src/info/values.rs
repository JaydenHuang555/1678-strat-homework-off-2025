
struct AutoValues {
    amp: u32,
    note_unamp: u32,
}

impl AutoValues {
    pub fn new() -> Self {
        Self {
            amp: 2,
            note_unamp: 5  
        }
    }
}

struct TeleValues {
    amp: u32,
    note_unamp: u32,
    note_amp: u32
}

impl TeleValues {
    pub fn new() -> Self {
        Self {
            amp: 1,
            note_unamp: 2,
            note_amp: 5 
        }
    }
}

struct EndGameValues {
    park: u32,
    onstage_non_spotlit: u32,
    onstage_spotlit: u32,
    harmony: u32,
    trap: u32
}

impl EndGameValues {
    pub fn new() -> Self {
        Self {
            park: 1,
            onstage_spotlit: 4,
            onstage_non_spotlit: 3,
            harmony: 2,
            trap: 5
        }
    }
}



