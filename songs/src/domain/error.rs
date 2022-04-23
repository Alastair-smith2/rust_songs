use std::{error, fmt};

// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html
#[derive(Debug)]
pub enum SongError {
    InvalidUrl,
    InvalidTitle,
}

// Update details here
impl fmt::Display for SongError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SongError::InvalidUrl => write!(f, "the provided string could not be parsed as a url"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            SongError::InvalidTitle => write!(f, "please use a non empty title"),
        }
    }
}

impl error::Error for SongError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SongError::InvalidUrl => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            SongError::InvalidTitle => None,
        }
    }
}
