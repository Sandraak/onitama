#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::piece::{Piece, Rank, Team};
use crate::pos::Position;
use crate::Action;

const ROWS: usize = 5;
const COLUMNS: usize = 5;

const RED_START_ROW: usize = 0;
const BLUE_START_ROW: usize = ROWS - 1;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Board {
    pieces: [[Option<Piece>; COLUMNS]; ROWS],
}

impl Board {
    pub(crate) fn perform_raw(&mut self, action: Action) -> Result<()> {
        match self.get(action.pos)? {
            None => Err(Error::Empty),
            Some(piece) => {
                match self.get(action.pos + action.shift)? {
                    None => {
                        // Move
                        let piece = self.get_mut(action.pos)?.take().unwrap();
                        *self.get_mut(action.pos + action.shift)? = Some(piece);
                        Ok(())
                    }
                    Some(other) if piece.team != other.team => {
                        // Capture
                        let piece = self.get_mut(action.pos)?.take().unwrap();
                        *self.get_mut(action.pos + action.shift)? = Some(piece);
                        Ok(())
                    }
                    Some(_) => Err(Error::Occupied),
                }
            }
        }
    }

    pub(crate) fn winner(&self) -> Option<Team> {
        for team in [Team::Red, Team::Blue] {
            if !self.pieces.iter().flatten().flatten().any(|piece| {
                *piece
                    == Piece {
                        team,
                        rank: Rank::Master,
                    }
            }) {
                return Some(!team);
            }
        }

        if Some(Piece::RED_MASTER) == self.pieces[BLUE_START_ROW][COLUMNS / 2] {
            return Some(Team::Red);
        } else if Some(Piece::BLUE_MASTER) == self.pieces[RED_START_ROW][COLUMNS / 2] {
            return Some(Team::Blue);
        }

        None
    }

    pub(crate) fn get(&self, index: Position) -> Result<&Option<Piece>> {
        self.pieces
            .get(index.row)
            .and_then(|row| row.get(index.column))
            .ok_or(Error::OutOfBounds)
    }

    fn get_mut(&mut self, index: Position) -> Result<&mut Option<Piece>> {
        self.pieces
            .get_mut(index.row)
            .and_then(|row| row.get_mut(index.column))
            .ok_or(Error::OutOfBounds)
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut pieces: [[Option<Piece>; COLUMNS]; ROWS] = Default::default();

        pieces[RED_START_ROW][0] = Some(Piece::RED_PAWN);
        pieces[RED_START_ROW][1] = Some(Piece::RED_PAWN);
        pieces[RED_START_ROW][2] = Some(Piece::RED_MASTER);
        pieces[RED_START_ROW][3] = Some(Piece::RED_PAWN);
        pieces[RED_START_ROW][4] = Some(Piece::RED_PAWN);

        pieces[BLUE_START_ROW][0] = Some(Piece::BLUE_PAWN);
        pieces[BLUE_START_ROW][1] = Some(Piece::BLUE_PAWN);
        pieces[BLUE_START_ROW][2] = Some(Piece::BLUE_MASTER);
        pieces[BLUE_START_ROW][3] = Some(Piece::BLUE_PAWN);
        pieces[BLUE_START_ROW][4] = Some(Piece::BLUE_PAWN);

        Board { pieces }
    }
}
