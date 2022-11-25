#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::pos::Shift;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Card {
    name: &'static str,
    pub(crate) shifts: &'static [Shift],
}

impl Card {
    pub const TIGER: Card = Card {
        name: "Tiger",
        shifts: &[Shift::UP_UP, Shift::DOWN],
    };

    pub const DRAGON: Card = Card {
        name: "Dragon",
        shifts: &[
            Shift::UP_LEFT_LEFT,
            Shift::UP_RIGHT_RIGHT,
            Shift::DOWN_LEFT,
            Shift::DOWN_RIGHT,
        ],
    };

    pub const FROG: Card = Card {
        name: "Frog",
        shifts: &[Shift::UP_LEFT, Shift::LEFT_LEFT, Shift::DOWN_RIGHT],
    };

    pub const RABBIT: Card = Card {
        name: "Rabbit",
        shifts: &[Shift::UP_RIGHT, Shift::RIGHT_RIGHT, Shift::DOWN_LEFT],
    };

    pub const CRAB: Card = Card {
        name: "Crab",
        shifts: &[Shift::UP, Shift::LEFT_LEFT, Shift::RIGHT_RIGHT],
    };

    pub const ELEPHANT: Card = Card {
        name: "Elephant",
        shifts: &[Shift::UP_LEFT, Shift::UP_RIGHT, Shift::LEFT, Shift::RIGHT],
    };

    pub const GOOSE: Card = Card {
        name: "Goose",
        shifts: &[Shift::UP_LEFT, Shift::LEFT, Shift::RIGHT, Shift::DOWN_RIGHT],
    };

    pub const ROOSTER: Card = Card {
        name: "Rooster",
        shifts: &[Shift::UP_RIGHT, Shift::LEFT, Shift::RIGHT, Shift::DOWN_LEFT],
    };

    pub const MONKEY: Card = Card {
        name: "Monkey",
        shifts: &[
            Shift::UP_LEFT,
            Shift::UP_RIGHT,
            Shift::DOWN_LEFT,
            Shift::DOWN_RIGHT,
        ],
    };

    pub const MANTIS: Card = Card {
        name: "Mantis",
        shifts: &[Shift::UP_LEFT, Shift::UP_RIGHT, Shift::DOWN],
    };

    pub const HORSE: Card = Card {
        name: "Horse",
        shifts: &[Shift::UP, Shift::LEFT, Shift::DOWN],
    };

    pub const OX: Card = Card {
        name: "Ox",
        shifts: &[Shift::UP, Shift::RIGHT, Shift::DOWN],
    };

    pub const CRANE: Card = Card {
        name: "Crane",
        shifts: &[Shift::UP, Shift::DOWN_LEFT, Shift::DOWN_RIGHT],
    };

    pub const BOAR: Card = Card {
        name: "Boar",
        shifts: &[Shift::UP, Shift::LEFT, Shift::RIGHT],
    };

    pub const EEL: Card = Card {
        name: "Eel",
        shifts: &[Shift::UP_LEFT, Shift::RIGHT, Shift::DOWN_LEFT],
    };

    pub const COBRA: Card = Card {
        name: "Cobra",
        shifts: &[Shift::UP_RIGHT, Shift::LEFT, Shift::DOWN_RIGHT],
    };

    pub const DECK: [Card; 16] = [
        Self::TIGER,
        Self::DRAGON,
        Self::FROG,
        Self::RABBIT,
        Self::CRAB,
        Self::ELEPHANT,
        Self::GOOSE,
        Self::ROOSTER,
        Self::MONKEY,
        Self::MANTIS,
        Self::HORSE,
        Self::OX,
        Self::CRANE,
        Self::BOAR,
        Self::EEL,
        Self::COBRA,
    ];
}
