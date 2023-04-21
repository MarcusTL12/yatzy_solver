#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use yatzy::YatzyState5;

pub mod dice_distributions;
pub mod dice_throw;
pub mod level_ordering;
pub mod util;
pub mod yatzy;

fn main() {
    let s = YatzyState5::new();

    println!("{}", s.get_index());
}
