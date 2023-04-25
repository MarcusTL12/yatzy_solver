#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use macrosolver::incore::solve_5dice;

pub mod dice_distributions;
pub mod dice_throw;
pub mod level_ordering;
pub mod solver;
pub mod util;
pub mod yatzy;
pub mod macrosolver;

fn main() {
    let (scores, _strats) = solve_5dice();

    println!("{:5.2?}", scores[[5, 8, 2]].as_ref().unwrap());
}
