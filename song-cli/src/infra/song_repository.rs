use csv::{DeserializeRecordsIter, Reader, Writer, WriterBuilder};
use songs::domain::song::Song;
use std::{
    error::Error,
    fs::{File, OpenOptions},
};

use crate::cli::commands::CreateSong;

use super::{
    error::SongResultError,
    model::{SongRowParsed, SongRowToStore},
};

pub trait SongRepository {
    fn save_song(self, song_input: CreateSong) -> Result<Song, Box<dyn Error>>;
    fn load_song(self, song_title: &str) -> Result<Option<Song>, Box<dyn Error>>;
}

pub struct CliRepository {
    path: String,
}

impl CliRepository {
    pub fn new(path: String) -> Self {
        CliRepository { path }
    }

    fn load_file(self) -> Result<File, Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.path)?;
        Ok(file)
    }

    fn build_writer(self) -> Result<Writer<File>, Box<dyn Error>> {
        Ok(WriterBuilder::new()
            .has_headers(false)
            .from_writer(self.load_file()?))
    }

    fn build_reader(self) -> Result<Reader<File>, Box<dyn Error>> {
        Ok(Reader::from_reader(
            OpenOptions::new().read(true).open(self.path)?,
        ))
    }

    fn store_song(self, song: &Song) -> Result<(), Box<dyn Error>> {
        let mut wtr = self.build_writer()?;
        let created_at = song.get_created_at().to_rfc3339();
        let record = SongRowToStore::new(
            song.get_title(),
            song.get_url(),
            song.get_count(),
            &created_at,
        );
        wtr.serialize(&record)?;
        Ok(())
    }

    fn parse_song(self, song_title: &str) -> Result<Option<Song>, Box<dyn Error>> {
        let mut rdr = self.build_reader()?;
        let iter: DeserializeRecordsIter<File, SongRowParsed> = rdr.deserialize();
        // Ideally this would be an iterator
        for record in iter {
            match record {
                Ok(song_record) => {
                    if song_record.get_title() == song_title {
                        let song: Song = song_record.try_into()?;
                        return Ok(Some(song));
                    } else {
                        continue;
                    }
                }
                Err(err) => {
                    return Err(Box::new(SongResultError::LookupError(err.to_string())));
                }
            }
        }
        Ok(None)
    }
}

impl SongRepository for CliRepository {
    fn save_song(self, song_input: CreateSong) -> Result<Song, Box<dyn Error>> {
        let song = Song::new(song_input.title, song_input.url, None, None)?;
        self.store_song(&song)?;
        Ok(song)
    }

    fn load_song(self, song_title: &str) -> Result<Option<Song>, Box<dyn Error>> {
        self.parse_song(song_title)
    }
}
