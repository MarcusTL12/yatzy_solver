// This is the generic yatzy state for either 5 or 6 dice.

use crate::{dice_throw::DiceThrow, level_ordering::*, util::count_true};

pub struct State<const CELLS: usize> {
    cells: [bool; CELLS],
    points_above: usize,
}

pub type YatzyState5 = State<15>;
pub type YatzyState6 = State<20>;

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

    pub fn set_cell(&mut self, i: usize, throw: DiceThrow) -> usize
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

    pub fn get_index(&self) -> usize {
        let (above_cells, below_cells) = self.split_cells();
        let above_ind = ABOVE_LEVELS_5_MAP[&(self.points_above, above_cells)];
        let below_ind = BELOW_LEVELS_5_MAP[&below_cells];

        let max_above = ABOVE_LEVELS_5[self.get_n_above()].len();

        below_ind * max_above + above_ind
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

    pub fn get_index(&self) -> usize {
        let (above_cells, below_cells) = self.split_cells();
        let above_ind = ABOVE_LEVELS_6_MAP[&(self.points_above, above_cells)];
        let below_ind = BELOW_LEVELS_6_MAP[&below_cells];

        let max_above = ABOVE_LEVELS_6[self.get_n_above()].len();

        below_ind * max_above + above_ind
    }
}
