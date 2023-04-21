// This is the generic yatzy state for either 5 or 6 dice.

use crate::{level_ordering::*, util::count_true};

const N_CELLS_5: usize = 15;
const N_CELLS_6: usize = 20;

pub struct State<const N: usize> {
    cells: [bool; N],
    points_above: u32,
}

impl<const N: usize> State<N> {
    pub fn get_above_cells(&self) -> [bool; 6] {
        *self.cells.split_array_ref().0
    }

    pub fn get_below_cells(&self) -> [bool; N - 6] {
        *self.cells.rsplit_array_ref().1
    }

    pub fn split_cells(&self) -> ([bool; 6], [bool; N - 6]) {
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
        [(); N - 6]:,
    {
        count_true(self.get_below_cells())
    }
}

impl State<N_CELLS_5> {
    pub fn get_index(&self) -> usize {
        let (above_cells, below_cells) = self.split_cells();
        let above_ind = ABOVE_LEVELS_5_MAP[&(self.points_above, above_cells)];
        let below_ind = BELOW_LEVELS_5_MAP[&below_cells];

        let max_above = ABOVE_LEVELS_5[self.get_n_above()].len();

        below_ind * max_above + above_ind
    }
}

impl State<N_CELLS_6> {
    pub fn get_index(&self) -> usize {
        let (above_cells, below_cells) = self.split_cells();
        let above_ind = ABOVE_LEVELS_6_MAP[&(self.points_above, above_cells)];
        let below_ind = BELOW_LEVELS_6_MAP[&below_cells];

        let max_above = ABOVE_LEVELS_6[self.get_n_above()].len();

        below_ind * max_above + above_ind
    }
}
