#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use crate::level_ordering::{ABOVE_LEVELS_5, BELOW_LEVELS_5};

pub mod dice_distributions;
pub mod dice_throw;
pub mod level_ordering;
pub mod solver;
pub mod util;
pub mod yatzy;

fn main() {
    println!(
        "{:?}",
        ABOVE_LEVELS_5.iter().map(|x| x.len()).collect::<Vec<_>>()
    );

    println!(
        "{:?}",
        BELOW_LEVELS_5.iter().map(|x| x.len()).collect::<Vec<_>>()
    );

    // let first_scores = make_bottom_layer_5dice();

    // let (scores, strats) = solve_layer_type1_5dice(14, &first_scores);

    // println!("{scores:?}");
    // println!("{strats:?}");

    // let max_score = scores
    //     .iter()
    //     .max_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // println!("{max_score}");

    // let state = YatzyState5::new();
}
