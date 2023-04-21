#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use crate::level_ordering::BELOW_LEVELS_5;

pub mod level_ordering;
pub mod dice_distributions;
pub mod dice_throw;
pub mod yatzy;
pub mod util;

fn main() {
    println!("{:?}", BELOW_LEVELS_5[3]);
}
