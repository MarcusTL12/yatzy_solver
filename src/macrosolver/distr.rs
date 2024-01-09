use std::{
    cell::Cell,
    fs::{create_dir_all, OpenOptions},
    io::{Read, Write},
    ops::Add,
    time::{Duration, Instant},
};

use ndarray::{
    linalg::general_mat_vec_mul,
    parallel::prelude::{
        IndexedParallelIterator, IntoParallelIterator, ParallelIterator,
    },
    Array1, Array3, Array4, ArrayView4,
};
use thread_local::ThreadLocal;

use crate::{
    dice_distributions::{DICE_DISTR, DICE_PROBS_VECTORS},
    level_ordering::{ABOVE_LEVELS_5, BELOW_LEVELS_5},
    macrosolver::Layer,
    solvers::full::{
        make_zero_dists, solve_layer_5dice_cells, solve_layer_5dice_throws,
    },
};

use super::{floats_to_bytes, floats_to_bytes_mut, measures, PREFIX};

const MAX_SCORE_5: usize = 375;

pub fn solve_5dice_dyn(name: &str) {
    match name {
        "mean" => solve_5dice(name, measures::mean),
        "median" => solve_5dice(name, measures::quantile(0.5)),
        _ => panic!(),
    }
}

fn contract_throws<const N: usize>(scores: ArrayView4<f32>) -> Array3<f32> {
    let (n_ai, n_bi, n_t, n_s) = scores.dim();

    let scores_packed = scores.into_shape([n_ai * n_bi, n_t, n_s]).unwrap();

    let mut contracted_scores = Array3::zeros([n_ai, n_bi, n_s]);

    let mut contracted_scores_packed = contracted_scores
        .view_mut()
        .into_shape([n_ai * n_bi, n_s])
        .unwrap();

    let a_vec = DICE_PROBS_VECTORS[N - 1].view();

    let tls = ThreadLocal::new();

    contracted_scores_packed
        .outer_iter_mut()
        .into_par_iter()
        .zip(scores_packed.outer_iter())
        .for_each(|(mut contr, scores)| {
            let buf_cell = tls.get_or(|| Cell::new(Array1::zeros([n_s])));

            let mut buf = buf_cell.take();

            // b_s = A_t * x_t,s
            general_mat_vec_mul(
                1.0,
                &scores.reversed_axes(),
                &a_vec,
                0.0,
                &mut buf,
            );

            contr.assign(&buf);

            buf_cell.set(buf);
        });

    contracted_scores
}

fn _save_full_scores(path: &str, scores: ArrayView4<f32>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .ok()
        .unwrap();

    let data = scores.as_slice().unwrap();
    let bytes = floats_to_bytes(data);

    file.write_all(bytes).unwrap();
}

fn _load_full_scores5(
    path: &str,
    na: usize,
    nb: usize,
    ns: usize,
) -> Array4<f32> {
    let mut file = OpenOptions::new().read(true).open(path).ok().unwrap();

    let mut scores = Array4::zeros([
        ABOVE_LEVELS_5[na].len(),
        BELOW_LEVELS_5[nb].len(),
        DICE_DISTR.5.len(),
        ns,
    ]);

    let data = scores.as_slice_mut().unwrap();

    let bytes: &mut [u8] = floats_to_bytes_mut(data);

    file.read_exact(bytes).unwrap();

    scores
}

fn solve_5dice<O: Ord + Add<f32, Output = O> + Clone + Copy>(
    name: &str,
    measure: impl Fn(&[f32]) -> O + Send + Sync + Copy,
) {
    create_dir_all(format!("{}/{name}5/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/{name}5/strats/", *PREFIX)).unwrap();

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    for na in (0..=6).rev() {
        for nb in (0..=9).rev() {
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

            if (Layer::<5, false> {
                id: name.to_owned(),
                na,
                nb,
                nt: 2,
                scores: None,
                strats: None,
            })
            .is_done()
            {
                println!("Stack is done!");
                continue;
            }

            let mut prev_above_layer = (na < 6)
                .then(|| Layer::<5, false> {
                    id: name.to_owned(),
                    na: na + 1,
                    nb,
                    nt: 2,
                    scores: None,
                    strats: None,
                })
                .unwrap_or(Layer::empty(""));
            let mut prev_below_layer = (nb < 9)
                .then(|| Layer::<5, false> {
                    id: name.to_owned(),
                    na,
                    nb: nb + 1,
                    nt: 2,
                    scores: None,
                    strats: None,
                })
                .unwrap_or(Layer::empty(""));

            let timer = Instant::now();

            prev_above_layer.load_dists(MAX_SCORE_5);
            prev_below_layer.load_dists(MAX_SCORE_5);

            let t = timer.elapsed();
            println!("Loading took {t:.2?}");
            load_timer += t;

            let timer = Instant::now();

            let (scores, strats) = solve_layer_5dice_cells(
                na,
                nb,
                MAX_SCORE_5,
                prev_above_layer.scores.as_ref().unwrap().view(),
                prev_below_layer.scores.as_ref().unwrap().view(),
                measure,
            );

            prev_above_layer.scores = None;
            prev_below_layer.scores = None;

            let t = timer.elapsed();
            println!("Solving took {t:.2?}");
            compute_timer += t;

            let scores_contracted = contract_throws::<5>(scores.view());

            layer.scores = Some(scores_contracted);
            layer.strats = Some(strats);

            let timer = Instant::now();

            let mut prev_scores_full = Some(scores);

            let t = timer.elapsed();
            println!("Contracting took  {t:.2?}");
            save_timer += t;

            let timer = Instant::now();

            layer.save_scores();
            layer.save_strats();

            layer.scores = None;
            layer.strats = None;

            let t = timer.elapsed();
            println!("Saving took  {t:.2?}");
            save_timer += t;

            for nt in 1..3 {
                println!("--------------------------------");
                println!("na: {na:2}, nb: {nb:2}, nt: {nt}");

                let mut layer = Layer::<5, false> {
                    id: name.to_owned(),
                    na,
                    nb,
                    nt,
                    scores: None,
                    strats: None,
                };

                if layer.is_done() {
                    println!("Already done!");
                } else {
                    let timer = Instant::now();

                    let prev_scores = prev_scores_full.take().unwrap();

                    let t = timer.elapsed();
                    println!("Loading took {t:.2?}");
                    load_timer += t;

                    let timer = Instant::now();

                    let mut scores = make_zero_dists([
                        ABOVE_LEVELS_5[na].len(),
                        BELOW_LEVELS_5[nb].len(),
                        DICE_DISTR.5.len(),
                        MAX_SCORE_5,
                    ]);
                    let mut strats = Array3::zeros([
                        ABOVE_LEVELS_5[na].len(),
                        BELOW_LEVELS_5[nb].len(),
                        DICE_DISTR.5.len(),
                    ]);

                    solve_layer_5dice_throws(
                        prev_scores.view(),
                        scores.view_mut(),
                        strats.view_mut(),
                        measure,
                    );

                    let t = timer.elapsed();
                    println!("Solving took {t:.2?}");
                    compute_timer += t;

                    let scores_contracted = contract_throws::<5>(scores.view());

                    layer.scores = Some(scores_contracted);
                    layer.strats = Some(strats);

                    let timer = Instant::now();

                    prev_scores_full = Some(scores);

                    let t = timer.elapsed();
                    println!("Contracting took  {t:.2?}");
                    save_timer += t;

                    let timer = Instant::now();

                    layer.save_scores();
                    layer.save_strats();

                    let t = timer.elapsed();
                    println!("Saving took  {t:.2?}");
                    save_timer += t;
                }
            }
        }
    }

    println!("\n\nTotal   time: {:.2?}", global_timer.elapsed());
    println!("Compute time: {compute_timer:.2?}");
    println!("Loading time: {load_timer:.2?}");
    println!("Saving  time: {save_timer:.2?}");
}
