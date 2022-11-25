use std::ops::Not;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Piece {
    pub(crate) team: Team,
    pub(crate) rank: Rank,
}

impl Piece {
    pub const RED_PAWN: Piece = Piece {
        team: Team::Red,
        rank: Rank::Pawn,
    };
    pub const RED_MASTER: Piece = Piece {
        team: Team::Red,
        rank: Rank::Master,
    };
    pub const BLUE_PAWN: Piece = Piece {
        team: Team::Blue,
        rank: Rank::Pawn,
    };
    pub const BLUE_MASTER: Piece = Piece {
        team: Team::Blue,
        rank: Rank::Master,
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Rank {
    #[default]
    Pawn,
    Master,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Team {
    #[default]
    Red,
    Blue,
}

impl Not for Team {
    type Output = Team;

    fn not(self) -> Self::Output {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }
}
