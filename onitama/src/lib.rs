use std::collections::BTreeMap;
use std::mem;

use rand::seq::SliceRandom;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use crate::board::Board;
pub use crate::card::Card;
pub use crate::error::{Error, Result};
pub use crate::piece::{Piece, Rank, Team};
pub use crate::pos::{Position, Shift};

mod board;
mod card;
mod error;
mod piece;
mod pos;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Onitama {
    board: Board,
    current_player: Team,
    cards: BTreeMap<Team, [Card; 2]>,
    spare_card: Card,
}

impl Onitama {
    pub fn new() -> Self {
        let board = Board::default();

        let current_player = Team::default();

        let mut deck = Card::DECK.to_vec();
        deck.shuffle(&mut rand::thread_rng());

        let mut cards = BTreeMap::new();
        cards.insert(Team::Red, [deck.pop().unwrap(), deck.pop().unwrap()]);
        cards.insert(Team::Blue, [deck.pop().unwrap(), deck.pop().unwrap()]);

        let spare_card = deck.pop().unwrap();

        Onitama {
            board,
            current_player,
            cards,
            spare_card,
        }
    }

    pub fn perform(&mut self, action: Action) -> Result<()> {
        match self.board.get(action.pos)? {
            None => return Err(Error::Empty),
            Some(piece) => {
                if piece.team != self.current_player {
                    return Err(Error::InvalidTeam);
                }
            }
        }

        if !self.cards[&self.current_player][action.card_index]
            .shifts
            .contains(&action.shift)
        {
            return Err(Error::InvalidMove);
        }

        self.board.perform_raw(action)?;

        mem::swap(
            &mut self.cards.get_mut(&self.current_player).unwrap()[action.card_index],
            &mut self.spare_card,
        );

        self.current_player = !self.current_player;

        Ok(())
    }

    pub fn current_player(&self) -> Team {
        self.current_player
    }

    pub fn winner(&self) -> Option<Team> {
        self.board.winner()
    }
}

impl Default for Onitama {
    fn default() -> Self {
        Onitama::new()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Action {
    pub(crate) pos: Position,
    pub(crate) shift: Shift,
    pub(crate) card_index: usize,
}
