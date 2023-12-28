#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use std::{env, time::Instant};

use dice_distributions::{DICE_DISTR, DICE_DIVISOR};
use guide::start;
use macrosolver::{
    outcore::{solve_5dice, solve_6dice, Layer},
    outcorex::{solve_5dicex, solve_6dicex},
};
use simulation::{simulate_n_5, simulate_n_6};

pub mod dice_distributions;
pub mod dice_throw;
pub mod guide;
pub mod level_ordering;
pub mod macrosolver;
pub mod simulation;
pub mod solver;
pub mod util;
pub mod yatzy;

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.get(1).unwrap_or(&"".to_owned()).as_str() {
        "guide-5" => start::<5, false>(),
        "guide-5x" => start::<5, true>(),
        "guide-6" => start::<6, false>(),
        "compute-strats-5" => solve_5dice(),
        "compute-strats-5x" => solve_5dicex(),
        "compute-strats-6" => solve_6dice(),
        "compute-strats-6x" => solve_6dicex(),
        "expected-score-5" => {
            let mut layer = Layer::<5, false> {
                na: 0,
                nb: 0,
                nt: 2,
                scores: None,
                strats: None,
            };

            layer.load_scores();
            let scores = layer.scores.unwrap();

            let mut score = 0.0;

            for (i, &(_, prob)) in DICE_DISTR.5.iter().enumerate() {
                let prob = prob as f64 / DICE_DIVISOR[5] as f64;

                score += scores[[0, 0, i]] as f64 * prob;
            }

            println!("Expected score for 5 dice: {score:.2}");
        }
        "expected-score-5x" => {
            let mut layer = Layer::<5, true> {
                na: 0,
                nb: 0,
                nt: 2,
                scores: None,
                strats: None,
            };

            layer.load_scores();
            let scores = layer.scores.unwrap();

            let mut score = 0.0;

            for (i, &(_, prob)) in DICE_DISTR.5.iter().enumerate() {
                let prob = prob as f64 / DICE_DIVISOR[5] as f64;

                score += scores[[0, 0, i]] as f64 * prob;
            }

            println!("Expected score for 5 dice: {score:.2}");
        }
        "simulate-5" => {
            let mut scores = vec![0; args[2].parse().unwrap()];

            let timer = Instant::now();
            simulate_n_5(&mut scores);
            let t = timer.elapsed();

            println!("time: {t:.2?}");
            println!("{scores:?}");
        }
        "expected-score-6" => {
            let mut layer = Layer::<6, false> {
                na: 0,
                nb: 0,
                nt: 2,
                scores: None,
                strats: None,
            };

            layer.load_scores();
            let scores = layer.scores.unwrap();

            let mut score = 0.0;

            for (i, &(_, prob)) in DICE_DISTR.6.iter().enumerate() {
                let prob = prob as f64 / DICE_DIVISOR[6] as f64;

                score += scores[[0, 0, i]] as f64 * prob;
            }

            println!("Expected score for 6 dice: {score:.2}");
        }
        "simulate-6" => {
            let mut scores = vec![0; args[2].parse().unwrap()];

            let timer = Instant::now();
            simulate_n_6(&mut scores);
            let t = timer.elapsed();

            println!("time: {t:.2?}");
            println!("{scores:?}");
        }
        _ => panic!(),
    }
}
