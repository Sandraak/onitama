use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("there is no piece there")]
    Empty,
    #[error("another piece is there already")]
    Occupied,
    #[error("that tile is off the board")]
    OutOfBounds,
    #[error("that card has no such move")]
    InvalidMove,
    #[error("that piece is not from your team")]
    InvalidTeam,
}
