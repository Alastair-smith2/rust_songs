use serde::{Deserialize, Serialize};
use songs::domain::{error::SongError, song::Song};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct SongRowParsed {
    title: String,
    url: String,
    play_count: i64,
    created_at: String,
}

impl SongRowParsed {
    pub fn new(title: String, url: String, play_count: i64, created_at: String) -> Self {
        SongRowParsed {
            title,
            url,
            play_count,
            created_at,
        }
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }
}

impl TryFrom<SongRowParsed> for Song {
    type Error = SongError;
    fn try_from(row: SongRowParsed) -> Result<Self, Self::Error> {
        Song::new(row.title, row.url, Some(row.play_count), None)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct SongRowToStore<'a> {
    title: &'a str,
    url: &'a str,
    play_count: i64,
    created_at: &'a str,
}

impl<'a> SongRowToStore<'a> {
    pub fn new(title: &'a str, url: &'a str, play_count: i64, created_at: &'a str) -> Self {
        SongRowToStore {
            title,
            url,
            play_count,
            created_at,
        }
    }
}
