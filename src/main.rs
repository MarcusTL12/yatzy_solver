#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use macrosolver::incore::solve_5dice;

pub mod dice_distributions;
pub mod dice_throw;
pub mod level_ordering;
pub mod macrosolver;
pub mod solver;
pub mod util;
pub mod yatzy;

fn main() {
    let (scores, _strats) = solve_5dice();

    println!("{:5.2?}", scores[[0, 0, 2]].as_ref().unwrap());
}
