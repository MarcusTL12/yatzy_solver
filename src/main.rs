#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use yatzy::YatzyState5;

use crate::dice_throw::DiceThrow;

pub mod dice_distributions;
pub mod dice_throw;
pub mod level_ordering;
pub mod util;
pub mod yatzy;
pub mod solver;

fn main() {
    let mut s = YatzyState5::new();

    let dt = DiceThrow::from([1usize, 0, 0, 3, 0, 1]);

    println!("{dt}");

    s.modify_cell(3, dt);

    println!("{}", s.get_index());
}
