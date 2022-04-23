use chrono::{DateTime, Utc};
use url::Url;

use super::error::SongError;

#[derive(Debug, Clone)]
pub struct Song {
    title: String,
    url: Url,
    play_count: i64,
    created_at: DateTime<Utc>, // Add author
}

impl Song {
    pub fn new(
        title: String,
        url: String,
        play_count: Option<i64>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Song, SongError> {
        let validated_url = valid_url(&url)?;
        if title.is_empty() {
            return Err(SongError::InvalidTitle);
        }
        Ok(Song {
            title,
            url: validated_url,
            play_count: play_count.unwrap_or(0),
            created_at: created_at.unwrap_or(chrono::offset::Utc::now()),
        })
    }

    pub fn get_url(&self) -> &str {
        self.url.as_str()
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_count(&self) -> i64 {
        self.play_count
    }

    pub fn get_created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

fn valid_url(url: &str) -> Result<Url, SongError> {
    match Url::parse(url) {
        Ok(valid_url) => Ok(valid_url),
        Err(_reason) => Err(SongError::InvalidUrl),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn example_song() -> Song {
        Song::new(
            String::from("SoT"),
            String::from("https://www.youtube.com/watch?v=-Vqo0pjNVCw"),
            Some(1),
            Some(DateTime::from_utc(
                NaiveDateTime::from_timestamp(61, 0),
                Utc,
            )),
        )
        .unwrap()
    }

    #[test]
    fn it_should_be_able_to_get_song_count() {
        assert_eq!(example_song().get_count(), 1);
    }

    #[test]
    fn it_should_be_able_to_get_song_url() {
        assert_eq!(
            example_song().get_url(),
            "https://www.youtube.com/watch?v=-Vqo0pjNVCw"
        );
    }

    #[test]
    fn it_should_be_able_to_get_song_title() {
        assert_eq!(example_song().get_title(), "SoT");
    }

    #[test]
    fn it_should_return_an_error_given_a_invalid_url() {
        assert!(Song::new(
            String::from("SoT"),
            String::from("unexpected_url"),
            Some(1),
            Some(DateTime::from_utc(
                NaiveDateTime::from_timestamp(61, 0),
                Utc,
            )),
        )
        .is_err())
    }

    #[test]
    fn it_should_return_an_error_with_invalid_title() {
        assert!(Song::new(
            String::from(""),
            String::from("https://www.youtube.com/watch?v=-Vqo0pjNVCw"),
            Some(1),
            Some(DateTime::from_utc(
                NaiveDateTime::from_timestamp(61, 0),
                Utc,
            )),
        )
        .is_err())
    }
}
