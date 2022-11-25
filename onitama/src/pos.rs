use std::ops::Add;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Position {
    pub(crate) row: usize,
    pub(crate) column: usize,
}

impl Add<Shift> for Position {
    type Output = Position;

    fn add(mut self, rhs: Shift) -> Self::Output {
        self.row = (self.row as isize + rhs.dr) as usize;
        self.column = (self.column as isize + rhs.dc) as usize;
        self
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Shift {
    dr: isize,
    dc: isize,
}

impl Shift {
    pub const UP: Shift = Shift { dr: 0, dc: -1 };
    pub const DOWN: Shift = Shift { dr: 0, dc: 1 };
    pub const LEFT: Shift = Shift { dr: -1, dc: 0 };
    pub const RIGHT: Shift = Shift { dr: 1, dc: 0 };

    pub const UP_LEFT: Shift = Shift { dr: -1, dc: -1 };
    pub const UP_RIGHT: Shift = Shift { dr: 1, dc: -1 };
    pub const DOWN_LEFT: Shift = Shift { dr: -1, dc: 1 };
    pub const DOWN_RIGHT: Shift = Shift { dr: 1, dc: 1 };

    pub const UP_UP: Shift = Shift { dr: 0, dc: -2 };
    pub const LEFT_LEFT: Shift = Shift { dr: -2, dc: 0 };
    pub const RIGHT_RIGHT: Shift = Shift { dr: 2, dc: 0 };

    pub const UP_LEFT_LEFT: Shift = Shift { dr: -2, dc: -1 };
    pub const UP_RIGHT_RIGHT: Shift = Shift { dr: 2, dc: -1 };
}
