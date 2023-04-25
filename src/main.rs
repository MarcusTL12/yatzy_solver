#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use std::env;

use macrosolver::outcore::solve_5dice;

pub mod dice_distributions;
pub mod dice_throw;
pub mod guide;
pub mod level_ordering;
pub mod macrosolver;
pub mod solver;
pub mod util;
pub mod yatzy;

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.get(1).unwrap_or(&"".to_owned()).as_str() {
        "guide-5" => {}
        "compute-strats-5" => solve_5dice(),
        _ => panic!(),
    }
}
