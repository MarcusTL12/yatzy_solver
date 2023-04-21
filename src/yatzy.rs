// This is for normal 5 dice yatzy like shown in misc/yatzy.png

use crate::above_line::ABOVE_LEVELS_5_MAP;

struct State {
    cells: [bool; 15],
    points_above: u32,
}

impl State {
    fn get_index(&self) -> usize {
        let n_above = self.cells[..6].iter().filter(|&&b| b).count();
        let n_below = self.cells[6..].iter().filter(|&&b| b).count();

        let &above_cells = self.cells.split_array_ref().0;
        let above_ind = ABOVE_LEVELS_5_MAP[&(self.points_above, above_cells)];

        // TODO: make lookup tables for levels below

        todo!()
    }
}
