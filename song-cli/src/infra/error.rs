use std::{error::Error, fmt};

#[derive(Debug)]
pub enum SongResultError {
    LookupError(String),
}

impl fmt::Display for SongResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SongResultError::LookupError(reason) => {
                write!(f, "the song could be loaded because of {}", reason)
            }
        }
    }
}

impl Error for SongResultError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &*self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            SongResultError::LookupError(reason) => None,
        }
    }
}
