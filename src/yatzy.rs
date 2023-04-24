// This is the generic yatzy state for either 5 or 6 dice.

use crate::{
    dice_throw::DiceThrow,
    level_ordering::{
        ABOVE_LEVELS_5_MAP, ABOVE_LEVELS_6_MAP, BELOW_LEVELS_5_MAP,
        BELOW_LEVELS_6_MAP,
    },
    util::count_true,
};

#[derive(Clone, Debug)]
pub struct State<const CELLS: usize> {
    pub cells: [bool; CELLS],
    pub points_above: usize,
}

pub type YatzyState5 = State<15>;
pub type YatzyState6 = State<20>;

impl<const CELLS: usize> From<([bool; 6], [bool; CELLS - 6], usize)>
    for State<CELLS>
{
    fn from(
        (above, below, points_above): ([bool; 6], [bool; CELLS - 6], usize),
    ) -> Self {
        let mut cells = [false; CELLS];
        *cells.split_array_mut().0 = above;
        *cells.rsplit_array_mut().1 = below;
        Self {
            cells,
            points_above,
        }
    }
}

impl<const CELLS: usize> State<CELLS> {
    pub fn get_above_cells(&self) -> [bool; 6] {
        *self.cells.split_array_ref().0
    }

    pub fn get_below_cells(&self) -> [bool; CELLS - 6] {
        *self.cells.rsplit_array_ref().1
    }

    pub fn split_cells(&self) -> ([bool; 6], [bool; CELLS - 6]) {
        (self.get_above_cells(), self.get_below_cells())
    }

    pub fn get_n_cells(&self) -> usize {
        count_true(self.cells)
    }

    pub fn get_n_above(&self) -> usize {
        count_true(self.get_above_cells())
    }

    pub fn get_n_below(&self) -> usize
    where
        [(); CELLS - 6]:,
    {
        count_true(self.get_below_cells())
    }

    pub fn modify_cell(&mut self, i: usize, throw: DiceThrow) -> usize
    where
        [(); CELLS / 5 + 2]:,
    {
        let points = throw.cell_score::<{ CELLS / 5 + 2 }>(i);

        if i < 6 {
            self.points_above += points;
        }

        self.cells[i] = true;

        points
    }

    pub fn set_cell(&self, i: usize, throw: DiceThrow) -> (Self, usize)
    where
        [(); CELLS / 5 + 2]:,
    {
        let mut state = self.clone();

        let score = state.modify_cell(i, throw);

        (state, score)
    }
}

impl Default for YatzyState5 {
    fn default() -> Self {
        Self::new()
    }
}

impl YatzyState5 {
    pub fn new() -> Self {
        Self {
            cells: [false; 15],
            points_above: 0,
        }
    }

    pub fn get_above_index(&self) -> usize {
        ABOVE_LEVELS_5_MAP[&(self.points_above, self.get_above_cells())]
    }

    pub fn get_below_index(&self) -> usize {
        BELOW_LEVELS_5_MAP[&self.get_below_cells()]
    }
}

impl Default for YatzyState6 {
    fn default() -> Self {
        Self::new()
    }
}

impl YatzyState6 {
    pub fn new() -> Self {
        Self {
            cells: [false; 20],
            points_above: 0,
        }
    }

    pub fn get_above_index(&self) -> usize {
        ABOVE_LEVELS_6_MAP[&(self.points_above, self.get_above_cells())]
    }

    pub fn get_below_index(&self) -> usize {
        BELOW_LEVELS_6_MAP[&self.get_below_cells()]
    }
}
