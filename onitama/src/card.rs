use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Card {
    animal: &'static str,
    pub moves: Vec<Move>,
}

impl Card {
    pub fn deck() -> Vec<Card> {
        let tiger = Card {
            animal: "Tiger",
            moves: vec![UPUP, DOWN],
        };
        let dragon = Card {
            animal: "Dragon",
            moves: vec![UPLEFTLEFT, UPRIGHTRIGHT, DOWNLEFT, DOWNRIGHT],
        };
        let frog = Card {
            animal: "Frog",
            moves: vec![UPLEFT, LEFTLEFT, DOWNRIGHT],
        };
        let rabbit = Card {
            animal: "Rabbit",
            moves: vec![UPRIGHT, RIGHTRIGHT, DOWNLEFT],
        };
        let crab = Card {
            animal: "Crab",
            moves: vec![UP, LEFTLEFT, RIGHTRIGHT],
        };
        let elephant = Card {
            animal: "Elephant",
            moves: vec![UPLEFT, UPRIGHT, LEFT, RIGHT],
        };
        let goose = Card {
            animal: "Goose",
            moves: vec![UPLEFT, LEFT, RIGHT, DOWNRIGHT],
        };
        let rooster = Card {
            animal: "Rooster",
            moves: vec![UPRIGHT, LEFT, RIGHT, DOWNLEFT],
        };
        let monkey = Card {
            animal: "Monkey",
            moves: vec![UPLEFT, UPRIGHT, DOWNLEFT, DOWNRIGHT],
        };
        let mantis = Card {
            animal: "Mantis",
            moves: vec![UPLEFT, UPRIGHT, DOWN],
        };
        let horse = Card {
            animal: "Horse",
            moves: vec![UP, LEFT, DOWN],
        };
        let ox = Card {
            animal: "Ox",
            moves: vec![UP, RIGHT, DOWN],
        };
        let crane = Card {
            animal: "Crane",
            moves: vec![UP, DOWNLEFT, DOWNRIGHT],
        };
        let boar = Card {
            animal: "Boar",
            moves: vec![UP, LEFT, RIGHT],
        };
        let eel = Card {
            animal: "Eel",
            moves: vec![UPLEFT, RIGHT, DOWNLEFT],
        };
        let cobra = Card {
            animal: "Cobra",
            moves: vec![UPRIGHT, LEFT, DOWNRIGHT],
        };

        vec![
            tiger, dragon, frog, rabbit, crab, elephant, goose, rooster, monkey, mantis, horse, ox,
            crane, boar, eel, cobra,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Move {
    pub dx: i8,
    pub dy: i8,
}

const UP: Move = Move { dx: 0, dy: -1 };
const DOWN: Move = Move { dx: 0, dy: 1 };
const LEFT: Move = Move { dx: -1, dy: 0 };
const RIGHT: Move = Move { dx: 1, dy: 0 };

const UPLEFT: Move = Move { dx: -1, dy: -1 };
const UPRIGHT: Move = Move { dx: 1, dy: -1 };
const DOWNLEFT: Move = Move { dx: -1, dy: 1 };
const DOWNRIGHT: Move = Move { dx: 1, dy: 1 };

const UPUP: Move = Move { dx: 0, dy: -2 };
const LEFTLEFT: Move = Move { dx: -2, dy: 0 };
const RIGHTRIGHT: Move = Move { dx: 2, dy: 0 };

const UPLEFTLEFT: Move = Move { dx: -2, dy: -1 };
const UPRIGHTRIGHT: Move = Move { dx: 2, dy: -1 };
