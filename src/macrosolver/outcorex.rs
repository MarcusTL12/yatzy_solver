use std::{
    fs::create_dir_all,
    thread,
    time::{Duration, Instant},
};

use crossbeam::channel::bounded;
use ndarray::Array3;

use crate::{
    macrosolver::outcore::{Layer, PREFIX},
    solver::{solve_layer_5dicex, solve_layer_6dicex},
};

pub fn make_thin_layers_5dicex() -> Array3<Option<Layer<5, true>>> {
    Array3::from_shape_fn([7, 10, 3 + 15 * 2], |(na, nb, nt)| {
        Some(Layer {
            na,
            nb,
            nt,
            scores: None,
            strats: None,
        })
    })
}

pub fn make_thin_layers_6dicex() -> Array3<Option<Layer<6, true>>> {
    Array3::from_shape_fn([7, 15, 3 + 20 * 2], |(na, nb, nt)| {
        Some(Layer {
            na,
            nb,
            nt,
            scores: None,
            strats: None,
        })
    })
}

pub fn solve_5dicex() {
    create_dir_all(format!("{}/5x/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/5x/strats/", *PREFIX)).unwrap();

    let mut layers = make_thin_layers_5dicex();

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    for na in (0..=6).rev() {
        for nb in (0..=9).rev() {
            let nt_max = (na + nb) * 2 + 2;

            for nt in 0..=nt_max {
                println!("=============================");
                println!("na: {na:2}, nb: {nb:2}, nt: {nt:2}");

                let mut layer = layers[[na, nb, nt]].take().unwrap();

                if layer.is_done() {
                    println!("Already done!");
                } else {
                    let mut prev_above_layer = layers
                        .get_mut([na + 1, nb, nt + 2])
                        .map_or_else(Layer::empty, |l| l.take().unwrap());
                    let mut prev_below_layer = layers
                        .get_mut([na, nb + 1, nt + 2])
                        .map_or_else(Layer::empty, |l| l.take().unwrap());
                    let mut prev_throw_layer = layers
                        .get_mut([na, nb, nt.wrapping_sub(1)])
                        .map(|x| x.take().unwrap());

                    let timer = Instant::now();

                    prev_above_layer.load_scores();
                    prev_below_layer.load_scores();

                    if let Some(l) = prev_throw_layer.as_mut() {
                        l.load_scores();
                    }

                    let t = timer.elapsed();
                    println!("Loading took {t:.2?}");
                    load_timer += t;

                    let timer = Instant::now();

                    let (scores, strats) = solve_layer_5dicex(
                        na,
                        nb,
                        &prev_above_layer.scores.unwrap(),
                        &prev_below_layer.scores.unwrap(),
                        prev_throw_layer
                            .as_ref()
                            .map(|l| l.scores.as_ref().unwrap()),
                    );

                    let t = timer.elapsed();
                    println!("Solving took {t:.2?}");
                    compute_timer += t;

                    prev_above_layer.scores = None;
                    prev_below_layer.scores = None;

                    if let Some(l) = prev_throw_layer.as_mut() {
                        l.scores = None;
                    }

                    if let Some(l) = layers.get_mut([na + 1, nb, nt + 2]) {
                        *l = Some(prev_above_layer);
                    }

                    if let Some(l) = layers.get_mut([na, nb + 1, nt + 2]) {
                        *l = Some(prev_below_layer);
                    }

                    if let Some(l) =
                        layers.get_mut([na, nb, nt.wrapping_sub(1)])
                    {
                        *l = prev_throw_layer;
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
                if nt == nt_max {
                    layer.scores = None;
                }

                layers[[na, nb, nt]] = Some(layer);
            }
        }
    }

    println!("\n\nTotal   time: {:.2?}", global_timer.elapsed());
    println!("Compute time: {compute_timer:.2?}");
    println!("Loading time: {load_timer:.2?}");
    println!("Saving  time: {save_timer:.2?}");
}

pub fn solve_6dicex() {
    create_dir_all(format!("{}/6x/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/6x/strats/", *PREFIX)).unwrap();

    let mut layers = make_thin_layers_6dicex();

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    let (ins_send, ins_rec) = bounded::<Layer<6, true>>(2);
    let (res_send, res_rec) = bounded::<Layer<6, true>>(2);

    thread::spawn(move || {
        for mut l in ins_rec {
            l.load_scores();
            res_send.send(l).unwrap();
        }
    });

    let mut loading = 0;

    for na in (0..=6).rev() {
        for nb in (0..=14).rev() {
            let nt_max = (na + nb) * 2 + 2;

            for nt in 0..=nt_max {
                println!("=============================");
                println!("na: {na:2}, nb: {nb:2}, nt: {nt:2}");

                let mut layer = layers[[na, nb, nt]].take().unwrap();

                if layer.is_done() {
                    println!("Already done!");
                } else {
                    while loading > 0 {
                        let l = res_rec.recv().unwrap();
                        let na = l.na;
                        let nb = l.nb;
                        let nt = l.nt;
                        layers[[na, nb, nt]] = Some(l);
                        loading -= 1;
                    }

                    let mut prev_above_layer = layers
                        .get_mut([na + 1, nb, nt + 2])
                        .map_or_else(Layer::empty, |l| l.take().unwrap());
                    let mut prev_below_layer = layers
                        .get_mut([na, nb + 1, nt + 2])
                        .map_or_else(Layer::empty, |l| l.take().unwrap());
                    let mut prev_throw_layer = layers
                        .get_mut([na, nb, nt.wrapping_sub(1)])
                        .map(|x| x.take().unwrap());

                    if nt < nt_max {
                        let next_above_layer = layers
                            .get_mut([na + 1, nb, nt + 3])
                            .map_or_else(Layer::empty, |l| l.take().unwrap());
                        let next_below_layer = layers
                            .get_mut([na, nb + 1, nt + 3])
                            .map_or_else(Layer::empty, |l| l.take().unwrap());

                        ins_send.send(next_above_layer).unwrap();
                        ins_send.send(next_below_layer).unwrap();

                        loading += 2;
                    }

                    let timer = Instant::now();

                    prev_above_layer.load_scores();
                    prev_below_layer.load_scores();

                    if let Some(l) = prev_throw_layer.as_mut() {
                        l.load_scores();
                    }

                    let t = timer.elapsed();
                    println!("Loading took {t:.2?}");
                    load_timer += t;

                    let timer = Instant::now();

                    let (scores, strats) = solve_layer_6dicex(
                        na,
                        nb,
                        &prev_above_layer.scores.unwrap(),
                        &prev_below_layer.scores.unwrap(),
                        prev_throw_layer
                            .as_ref()
                            .map(|l| l.scores.as_ref().unwrap()),
                    );

                    let t = timer.elapsed();
                    println!("Solving took {t:.2?}");
                    compute_timer += t;

                    prev_above_layer.scores = None;
                    prev_below_layer.scores = None;

                    if let Some(l) = prev_throw_layer.as_mut() {
                        l.scores = None;
                    }

                    if let Some(l) = layers.get_mut([na + 1, nb, nt + 2]) {
                        *l = Some(prev_above_layer);
                    }

                    if let Some(l) = layers.get_mut([na, nb + 1, nt + 2]) {
                        *l = Some(prev_below_layer);
                    }

                    if let Some(l) =
                        layers.get_mut([na, nb, nt.wrapping_sub(1)])
                    {
                        *l = prev_throw_layer;
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
                if nt == nt_max {
                    layer.scores = None;
                }

                layers[[na, nb, nt]] = Some(layer);
            }
        }
    }

    println!("\n\nTotal   time: {:.2?}", global_timer.elapsed());
    println!("Compute time: {compute_timer:.2?}");
    println!("Loading time: {load_timer:.2?}");
    println!("Saving  time: {save_timer:.2?}");
}
