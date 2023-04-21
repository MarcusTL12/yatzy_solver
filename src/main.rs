#![feature(array_zip, split_array)]

use crate::above_line::ABOVE_LEVELS_6;

pub mod above_line;
pub mod dice_distributions;
pub mod dice_throw;
pub mod yatzy;
pub mod util;

fn main() {
    for i in 0..7 {
        println!("{:?}", ABOVE_LEVELS_6[i].len());
    }
}
