use std::collections::BTreeMap;

use rand::prelude::SliceRandom;

use card::{Card, Move};

pub mod card;

pub struct State {
    board: Board,
    current_player: Colour,
    pub spare_card: Card,
    cards: BTreeMap<Colour, [Card; 2]>,
}

impl State {
    pub fn new() -> Self {
        let mut deck = Card::deck();
        let mut cards = BTreeMap::new();
        cards.insert(Colour::Blue, [deck.pop().unwrap(), deck.pop().unwrap()]);
        cards.insert(Colour::Red, [deck.pop().unwrap(), deck.pop().unwrap()]);
        deck.shuffle(&mut rand::thread_rng());
        State {
            board: Board::default(),
            current_player: Colour::Red,
            spare_card: deck.pop().unwrap(),
            cards,
        }
    }

    fn perform_mov(&mut self, mov: MovePiece) -> Result<(), MoveError> {
        if !self
            .cards
            .get(&self.current_player)
            .unwrap()
            .iter()
            .any(|card| card.moves.contains(&mov.mov))
        {
            return Err(MoveError::InvalidMove);
        }
        let piece = self.board.board[mov.from.x][mov.from.y]
            .take()
            .ok_or(MoveError::NoPiece)?;
        let to: Position = Position {
            x: (mov.from.x as i8 + mov.mov.dx) as usize,
            y: (mov.from.y as i8 + mov.mov.dy) as usize,
        };
        self.board.board[to.x][to.y] = Some(piece);
        Ok(())
    }

    fn winner(&self) -> Option<Colour> {
        if !self
            .board
            .board
            .iter()
            .flatten()
            .flatten()
            .any(|piece| *piece == Piece::REDMASTER)
        {
            return Some(Colour::Blue);
        } else if !self
            .board
            .board
            .iter()
            .flatten()
            .flatten()
            .any(|piece| *piece == Piece::BLUEMASTER)
        {
            return Some(Colour::Red);
        }

        if self.board.board[0][2] == Some(Piece::REDMASTER) {
            return Some(Colour::Red);
        } else if self.board.board[4][2] == Some(Piece::BLUEMASTER) {
            return Some(Colour::Blue);
        }
        None
    }
}

struct Board {
    board: [[Option<Piece>; 5]; 5],
}

impl Board {}

impl Default for Board {
    fn default() -> Self {
        let mut board: [[Option<Piece>; 5]; 5] = Default::default();
        board[0][0] = Some(Piece::BLUEPAWN);
        board[0][1] = Some(Piece::BLUEPAWN);
        board[0][2] = Some(Piece::BLUEMASTER);
        board[0][3] = Some(Piece::BLUEPAWN);
        board[0][4] = Some(Piece::BLUEPAWN);

        board[4][0] = Some(Piece::REDPAWN);
        board[4][1] = Some(Piece::REDPAWN);
        board[4][2] = Some(Piece::REDMASTER);
        board[4][3] = Some(Piece::REDPAWN);
        board[4][4] = Some(Piece::REDPAWN);
        Board { board }
    }
}

#[derive(PartialEq, Eq)]
struct Piece(Rank, Colour);

impl Piece {
    const REDPAWN: Piece = Piece(Rank::Pawn, Colour::Red);
    const REDMASTER: Piece = Piece(Rank::Master, Colour::Red);
    const BLUEPAWN: Piece = Piece(Rank::Pawn, Colour::Blue);
    const BLUEMASTER: Piece = Piece(Rank::Master, Colour::Blue);
}

struct MovePiece {
    from: Position,
    mov: Move,
    card: usize, //index van de kaarten van de spelen
}

struct Position {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq)]
enum Rank {
    Pawn,
    Master,
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
enum Colour {
    Red,
    Blue,
}

enum MoveError {
    NoPiece,
    InvalidMove,
}
