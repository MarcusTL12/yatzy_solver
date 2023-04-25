#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use macrosolver::outcore::solve_5dice;

pub mod dice_distributions;
pub mod dice_throw;
pub mod level_ordering;
pub mod macrosolver;
pub mod solver;
pub mod util;
pub mod yatzy;

fn main() {
    solve_5dice();
}
