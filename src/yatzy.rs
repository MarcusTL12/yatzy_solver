// This is the generic yatzy state for either 5 or 6 dice.

use crate::{
    dice_throw::DiceThrow,
    level_ordering::{
        points_above, ABOVE_LEVELS_5_MAP, ABOVE_LEVELS_6_MAP,
        BELOW_LEVELS_5_MAP, BELOW_LEVELS_6_MAP,
    },
    util::{count_true, parse_binary},
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

pub const fn cell_from_dice<const N: usize>() -> usize {
    match N {
        5 => 15,
        6 => 20,
        _ => panic!(),
    }
}

pub const fn dice_from_cells<const CELLS: usize>() -> usize {
    match CELLS {
        15 => 5,
        20 => 6,
        _ => panic!(),
    }
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CELLS: usize> State<CELLS> {
    pub fn new() -> Self {
        Self {
            cells: [false; CELLS],
            points_above: 0,
        }
    }

    pub fn get_above_cells(&self) -> [bool; 6] {
        *self.cells.split_array_ref().0
    }

    pub fn get_below_cells(&self) -> [bool; CELLS - 6] {
        *self.cells.rsplit_array_ref().1
    }

    pub fn _split_cells(&self) -> ([bool; 6], [bool; CELLS - 6]) {
        (self.get_above_cells(), self.get_below_cells())
    }

    pub fn _get_n_cells(&self) -> usize {
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

    fn get_bonus(&self) -> usize {
        match CELLS {
            15 => {
                if self.points_above >= 63 {
                    50
                } else {
                    0
                }
            }
            20 => {
                if self.points_above >= 84 {
                    100
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn modify_cell(&mut self, i: usize, throw: DiceThrow) -> usize
    where
        [(); dice_from_cells::<CELLS>()]:,
    {
        let points = throw.cell_score::<{ dice_from_cells::<CELLS>() }>(i);

        let old_bonus = self.get_bonus();

        if i < 6 {
            self.points_above += points;
        }

        let new_bonus = self.get_bonus();

        self.cells[i] = true;

        points + new_bonus - old_bonus
    }

    pub fn set_cell(&self, i: usize, throw: DiceThrow) -> (Self, usize)
    where
        [(); dice_from_cells::<CELLS>()]:,
    {
        let mut state = self.clone();

        let score = state.modify_cell(i, throw);

        (state, score)
    }

    pub fn from_dyn(dyn_cells: &[bool], points_above: usize) -> Self
    where
        [(); CELLS - 6]:,
    {
        assert!(dyn_cells.len() == CELLS);
        let mut cells = [false; CELLS];
        *cells.split_array_mut().0 = *dyn_cells.split_array_ref::<6>().0;
        *cells.rsplit_array_mut().1 =
            *dyn_cells.rsplit_array_ref::<{ CELLS - 6 }>().1;
        Self {
            cells,
            points_above,
        }
    }
}

impl YatzyState5 {
    pub fn get_above_index(&self) -> usize {
        let pts = points_above::<5>().min(self.points_above);

        ABOVE_LEVELS_5_MAP[[pts, parse_binary(&self.get_above_cells())]]
    }

    pub fn get_below_index(&self) -> usize {
        BELOW_LEVELS_5_MAP[parse_binary(&self.get_below_cells())]
    }
}

impl YatzyState6 {
    pub fn get_above_index(&self) -> usize {
        let pts = points_above::<6>().min(self.points_above);

        ABOVE_LEVELS_6_MAP[[pts, parse_binary(&self.get_above_cells())]]
    }

    pub fn get_below_index(&self) -> usize {
        BELOW_LEVELS_6_MAP[parse_binary(&self.get_below_cells())]
    }
}
