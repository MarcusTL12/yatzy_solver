use std::{
    fs::create_dir_all,
    ops::Add,
    time::{Duration, Instant},
};

use crate::macrosolver::Layer;

use super::{measures, PREFIX};

pub fn solve_5dice_dyn(name: &str) {
    match name {
        "mean" => solve_5dice(name, measures::mean),
        "median" => solve_5dice(name, measures::quantile(0.5)),
        _ => panic!(),
    }
}

fn solve_5dice<O: Ord + Add<f32, Output = O> + Clone + Copy>(
    name: &str,
    measure: impl Fn(&[f32]) -> O + Send + Sync,
) {
    create_dir_all(format!("{}/5_{name}/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/5_{name}/strats/", *PREFIX)).unwrap();

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    for na in (0..7).rev() {
        for nb in (0..10).rev() {
            println!("=============================");
            println!("na: {na:2}, nb: {nb:2}, nt: 0");

            let mut layer = Layer::<5, false> {
                id: name.to_owned(),
                na,
                nb,
                nt: 0,
                scores: None,
                strats: None,
            };

            todo!()
        }
    }
}
