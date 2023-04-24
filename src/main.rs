#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use std::time::Instant;

use ndarray::Array3;
use solver::{bottom_layer_dimensions_5dice, solve_layer_type1_5dice};

use crate::solver::solve_layer_type2_5dice;

pub mod dice_distributions;
pub mod dice_throw;
pub mod level_ordering;
pub mod solver;
pub mod util;
pub mod yatzy;

fn main() {
    let scores0 = Array3::zeros(bottom_layer_dimensions_5dice());

    let timer = Instant::now();

    let (scores1, _) =
        solve_layer_type1_5dice(6, 8, &Array3::zeros([0; 3]), &scores0);

    let d = timer.elapsed();
    println!("time = {d:?}");

    let timer = Instant::now();

    let (scores2, strats2) = solve_layer_type2_5dice(6, 8, &scores1);

    let d = timer.elapsed();
    println!("time = {d:?}");

    // let max_score = scores
    //     .iter()
    //     .max_by(|a, b| a.partial_cmp(b).unwrap())
    //     .unwrap();

    // println!("{max_score}");

    // let state = YatzyState5::new();
}
