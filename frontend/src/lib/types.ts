type State = {
  board: Board,
  currentPlayer: Colour,
  spareCard: Card,
  cards: Map<Colour, Card[]>,
};

type Board = {
  board: (Piece | null)[][],
};

type Piece = [Rank, Colour];

type MovePiece = {
  from: Position,
  mov: Move,
  card: number
}

type Position = {
  x: number,
  y: number
}

enum Rank {
  Pawn = 'PAWN',
  Master = 'MASTER'
}

enum Colour {
  Red = 'RED',
  Blue = 'BLUE'
}

namespace Colour {
  export function opposite(colour: Colour): Colour {
    if(colour === Colour.Red) {
      return Colour.Blue;
    } else {
      return Colour.Red;
    }
  }
}

type Card = {
  animal: string,
  moves: Move[]
};

type Move = {
  dx: number,
  dy: number
};

export {
  Rank,
  Colour
};

export type {
  State,
  Board,
  Piece,
  MovePiece,
  Position,
  Card,
  Move
};
