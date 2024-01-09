// This is the module for dealing with a proper system that saves incrementally
// to disk and can be paused and restarted.

use std::{
    fs::create_dir_all,
    time::{Duration, Instant},
};

use ndarray::Array3;

use crate::{
    macrosolver::{
        make_thin_layers_5dice, make_thin_layers_6dice, Layer, PREFIX,
    },
    solvers::mean::{
        solve_layer_5dice_cells, solve_layer_5dice_throws,
        solve_layer_6dice_cells, solve_layer_6dice_throws,
    },
};

pub fn solve_5dice() {
    create_dir_all(format!("{}/5/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/5/strats/", *PREFIX)).unwrap();

    let mut layers = make_thin_layers_5dice("");

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    for na in (0..7).rev() {
        for nb in (0..10).rev() {
            println!("=============================");
            println!("na: {na:2}, nb: {nb:2}, nt: 0");

            let mut layer: Layer<5, false> =
                layers[[na, nb, 0]].take().unwrap();

            if layer.is_done() {
                println!("Already done!");
            } else {
                let mut prev_above_layer = layers
                    .get_mut([na + 1, nb, 2])
                    .unwrap_or(&mut Some(Layer::empty("")))
                    .take()
                    .unwrap();
                let mut prev_below_layer = layers
                    .get_mut([na, nb + 1, 2])
                    .unwrap_or(&mut Some(Layer::empty("")))
                    .take()
                    .unwrap();

                let timer = Instant::now();

                prev_above_layer.load_scores();
                prev_below_layer.load_scores();

                let t = timer.elapsed();
                println!("Loading took {t:.2?}");
                load_timer += t;

                let timer = Instant::now();

                let (scores, strats) = solve_layer_5dice_cells(
                    na,
                    nb,
                    prev_above_layer.scores.as_ref().unwrap().view(),
                    prev_below_layer.scores.as_ref().unwrap().view(),
                );

                let t = timer.elapsed();
                println!("Solving took {t:.2?}");
                compute_timer += t;

                prev_above_layer.scores = None;
                prev_below_layer.scores = None;

                if let Some(x) = layers.get_mut([na + 1, nb, 2]) {
                    *x = Some(prev_above_layer);
                }

                if let Some(x) = layers.get_mut([na, nb + 1, 2]) {
                    *x = Some(prev_below_layer);
                }

                layer.scores = Some(scores);
                layer.strats = Some(strats);

                let timer = Instant::now();

                layer.save_scores();
                layer.save_strats();

                let t = timer.elapsed();
                println!("Saving took  {t:.2?}");
                save_timer += t;
            }

            layer.strats = None;
            layers[[na, nb, 0]] = Some(layer);

            for nt in 1..3 {
                println!("--------------------------------");
                println!("na: {na:2}, nb: {nb:2}, nt: {nt}");

                let mut layer = layers[[na, nb, nt]].take().unwrap();

                if layer.is_done() {
                    println!("Already done!");
                } else {
                    let mut prev_layer =
                        layers[[na, nb, nt - 1]].take().unwrap();

                    let timer = Instant::now();

                    prev_layer.load_scores();

                    let t = timer.elapsed();
                    println!("Loading took {t:.2?}");
                    load_timer += t;

                    let timer = Instant::now();

                    let mut scores = Array3::zeros(
                        prev_layer.scores.as_ref().unwrap().dim(),
                    );
                    let mut strats = Array3::zeros(
                        prev_layer.strats.as_ref().unwrap().dim(),
                    );

                    solve_layer_5dice_throws(
                        nb,
                        prev_layer.scores.as_ref().unwrap().view(),
                        scores.view_mut(),
                        strats.view_mut(),
                    );

                    let t = timer.elapsed();
                    println!("Solving took {t:.2?}");
                    compute_timer += t;

                    prev_layer.scores = None;

                    layers[[na, nb, nt - 1]] = Some(prev_layer);

                    layer.scores = Some(scores);
                    layer.strats = Some(strats);

                    let timer = Instant::now();

                    layer.save_scores();
                    layer.save_strats();

                    let t = timer.elapsed();
                    println!("Saving took  {t:.2?}");
                    save_timer += t;
                }

                layer.strats = None;
                layers[[na, nb, nt]] = Some(layer);
            }
        }
    }

    println!("\n\nTotal   time: {:.2?}", global_timer.elapsed());
    println!("Compute time: {compute_timer:.2?}");
    println!("Loading time: {load_timer:.2?}");
    println!("Saving  time: {save_timer:.2?}");
}

pub fn solve_6dice() {
    create_dir_all(format!("{}/6/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/6/strats/", *PREFIX)).unwrap();

    let mut layers = make_thin_layers_6dice("");

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    for na in (0..7).rev() {
        for nb in (0..15).rev() {
            println!("=============================");
            println!("na: {na:2}, nb: {nb:2}, nt: 0");

            let mut layer = layers[[na, nb, 0]].take().unwrap();

            if layer.is_done() {
                println!("Already done!");
            } else {
                let mut prev_above_layer = layers
                    .get_mut([na + 1, nb, 2])
                    .unwrap_or(&mut Some(Layer::empty("")))
                    .take()
                    .unwrap();
                let mut prev_below_layer = layers
                    .get_mut([na, nb + 1, 2])
                    .unwrap_or(&mut Some(Layer::empty("")))
                    .take()
                    .unwrap();

                let timer = Instant::now();

                prev_above_layer.load_scores();
                prev_below_layer.load_scores();

                let t = timer.elapsed();
                println!("Loading took {t:.2?}");
                load_timer += t;

                let timer = Instant::now();

                let (scores, strats) = solve_layer_6dice_cells(
                    na,
                    nb,
                    prev_above_layer.scores.as_ref().unwrap().view(),
                    prev_below_layer.scores.as_ref().unwrap().view(),
                );

                let t = timer.elapsed();
                println!("Solving took {t:.2?}");
                compute_timer += t;

                prev_above_layer.scores = None;
                prev_below_layer.scores = None;

                if let Some(x) = layers.get_mut([na + 1, nb, 2]) {
                    *x = Some(prev_above_layer);
                }

                if let Some(x) = layers.get_mut([na, nb + 1, 2]) {
                    *x = Some(prev_below_layer);
                }

                layer.scores = Some(scores);
                layer.strats = Some(strats);

                let timer = Instant::now();

                layer.save_scores();
                layer.save_strats();

                let t = timer.elapsed();
                println!("Saving  took {t:.2?}");
                save_timer += t;
            }

            layer.strats = None;
            layers[[na, nb, 0]] = Some(layer);

            for nt in 1..3 {
                println!("--------------------------------");
                println!("na: {na:2}, nb: {nb:2}, nt: {nt}");

                let mut layer: Layer<6, false> =
                    layers[[na, nb, nt]].take().unwrap();

                if layer.is_done() {
                    println!("Already done!");
                } else {
                    let mut prev_layer =
                        layers[[na, nb, nt - 1]].take().unwrap();

                    let timer = Instant::now();

                    prev_layer.load_scores();

                    let t = timer.elapsed();
                    println!("Loading took {t:.2?}");
                    load_timer += t;

                    let timer = Instant::now();

                    let mut scores = Array3::zeros(
                        prev_layer.scores.as_ref().unwrap().dim(),
                    );
                    let mut strats = Array3::zeros(
                        prev_layer.strats.as_ref().unwrap().dim(),
                    );

                    solve_layer_6dice_throws(
                        nb,
                        prev_layer.scores.as_ref().unwrap().view(),
                        scores.view_mut(),
                        strats.view_mut(),
                    );

                    let t = timer.elapsed();
                    println!("Solving took {t:.2?}");
                    compute_timer += t;

                    prev_layer.scores = None;

                    layers[[na, nb, nt - 1]] = Some(prev_layer);

                    layer.scores = Some(scores);
                    layer.strats = Some(strats);

                    let timer = Instant::now();

                    layer.save_scores();
                    layer.save_strats();

                    let t = timer.elapsed();
                    println!("Saving  took {t:.2?}");
                    save_timer += t;
                }

                layer.strats = None;
                layers[[na, nb, nt]] = Some(layer);
            }
        }
    }

    println!("\n\nTotal   time: {:.2?}", global_timer.elapsed());
    println!("Compute time: {compute_timer:.2?}");
    println!("Loading time: {load_timer:.2?}");
    println!("Saving  time: {save_timer:.2?}");
}
