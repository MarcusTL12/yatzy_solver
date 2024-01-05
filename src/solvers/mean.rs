use std::cell::Cell;

use ndarray::{
    linalg::general_mat_mul,
    parallel::prelude::{
        IndexedParallelIterator, IntoParallelIterator, ParallelIterator,
    },
    s, Array2, Array3, ArrayView3, ArrayViewMut3, Zip,
};
use thread_local::ThreadLocal;

use crate::{
    dice_distributions::{DICE_DISTR, DICE_DIVISOR, DICE_REROLL_MATRICES},
    dice_throw::DiceThrow,
    level_ordering::{
        ABOVE_LEVELS_5, ABOVE_LEVELS_6, BELOW_LEVELS_5, BELOW_LEVELS_6,
    },
    yatzy::State,
};

pub fn solve_layer_5dice_cells(
    na: usize,
    nb: usize,
    prev_above_layer_scores: ArrayView3<f32>,
    prev_below_layer_scores: ArrayView3<f32>,
) -> (Array3<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();

    let n_ai = ABOVE_LEVELS_5[na].len();
    let n_bi = BELOW_LEVELS_5[nb].len();

    let shape = [n_ai, n_bi, N_DICE_THROWS];

    let mut scores = Array3::zeros(shape);
    let mut strats = Array3::zeros(shape);

    Zip::indexed(&mut scores).and(&mut strats).par_for_each(
        |(ai, bi, ti), cur_score, cur_strat| {
            let (points_above, above_level) = ABOVE_LEVELS_5[na][ai];
            let below_level = BELOW_LEVELS_5[nb][bi];
            let throw = DiceThrow::from(DICE_DISTR.5[ti].0);
            let state =
                State::<15>::from((above_level, below_level, points_above));

            let mut best_score = 0.0;
            let mut best_cell_i = 255;

            for (cell_i, _) in
                state.cells.iter().enumerate().filter(|(_, &cell)| !cell)
            {
                let (new_state, extra_score) = state.set_cell(cell_i, throw);

                let mut new_ai = ai;
                let mut new_bi = bi;

                let prev_layer = if cell_i < 6 {
                    new_ai = new_state.get_above_index();
                    prev_above_layer_scores.view()
                } else {
                    new_bi = new_state.get_below_index();
                    prev_below_layer_scores.view()
                };

                let prev_layer_col = prev_layer.slice(s![new_ai, new_bi, ..]);

                // Expected score will be the extra score (guaranteed)
                // plus the expected score based on what you might roll next
                let mut expected_score = extra_score as f64;

                // For a given choice of cell, looping over possible
                // throws to find it's expected score.
                for (&(_, prob), &s) in DICE_DISTR.5.iter().zip(prev_layer_col)
                {
                    let prob = prob as f64 / DICE_DIVISOR[5] as f64;

                    expected_score = prob.mul_add(s as f64, expected_score);
                }

                if expected_score >= best_score {
                    best_score = expected_score;
                    best_cell_i = cell_i;
                }
            }

            *cur_score = best_score as f32;
            *cur_strat = best_cell_i as u8;
        },
    );

    (scores, strats)
}

pub fn solve_layer_5dice_throws(
    nb: usize,
    prev_throw_layer_scores: ArrayView3<f32>,
    mut scores: ArrayViewMut3<f32>,
    mut strats: ArrayViewMut3<u8>,
) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();

    let n_bi = BELOW_LEVELS_5[nb].len();

    // x_b,t2 * A_t1r,t2 = b_b,t1r

    let a_mat = DICE_REROLL_MATRICES[4]
        .view()
        .into_shape([N_DICE_THROWS * 32, N_DICE_THROWS])
        .unwrap()
        .reversed_axes();

    let tls = ThreadLocal::new();

    scores
        .outer_iter_mut()
        .into_par_iter()
        .zip(strats.outer_iter_mut())
        .zip(prev_throw_layer_scores.outer_iter())
        .for_each(|((mut scores, mut strats), prev_scores)| {
            let buf_cell = tls.get_or(|| {
                Cell::new(Array2::from_shape_simple_fn(
                    [n_bi, N_DICE_THROWS * 32],
                    || 0.0,
                ))
            });

            let mut buf = buf_cell.take();

            general_mat_mul(1.0, &prev_scores, &a_mat, 0.0, &mut buf);

            let buf3 =
                buf.view().into_shape([n_bi, N_DICE_THROWS, 32]).unwrap();

            for ((score, strat), row) in
                scores.iter_mut().zip(strats.iter_mut()).zip(buf3.rows())
            {
                let mut best_score = *score;
                let mut do_reroll = false;
                let mut best_reroll = 0;

                for (i, &sc) in row.iter().enumerate() {
                    if sc > best_score {
                        best_score = sc;
                        do_reroll = true;
                        best_reroll = i;
                    }
                }

                if do_reroll {
                    *score = best_score;
                    *strat = best_reroll as u8 | 128;
                }
            }

            buf_cell.set(buf);
        });
}

pub fn solve_layer_6dice_cells(
    na: usize,
    nb: usize,
    prev_above_layer_scores: ArrayView3<f32>,
    prev_below_layer_scores: ArrayView3<f32>,
) -> (Array3<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.6.len();

    let n_ai = ABOVE_LEVELS_6[na].len();
    let n_bi = BELOW_LEVELS_6[nb].len();

    let shape = [n_ai, n_bi, N_DICE_THROWS];

    let mut scores = Array3::zeros(shape);
    let mut strats = Array3::zeros(shape);

    Zip::indexed(&mut scores).and(&mut strats).par_for_each(
        |(ai, bi, ti), cur_score, cur_strat| {
            let (points_above, above_level) = ABOVE_LEVELS_6[na][ai];
            let below_level = BELOW_LEVELS_6[nb][bi];
            let throw = DiceThrow::from(DICE_DISTR.6[ti].0);
            let state =
                State::<20>::from((above_level, below_level, points_above));

            let mut best_score = 0.0;
            let mut best_cell_i = 255;

            for (cell_i, _) in
                state.cells.iter().enumerate().filter(|(_, &cell)| !cell)
            {
                let (new_state, extra_score) = state.set_cell(cell_i, throw);

                let mut new_ai = ai;
                let mut new_bi = bi;

                let prev_layer = if cell_i < 6 {
                    new_ai = new_state.get_above_index();
                    prev_above_layer_scores.view()
                } else {
                    new_bi = new_state.get_below_index();
                    prev_below_layer_scores.view()
                };

                let prev_layer_col = prev_layer.slice(s![new_ai, new_bi, ..]);

                // Expected score will be the extra score (guaranteed)
                // plus the expected score based on what you might roll next
                let mut expected_score = extra_score as f64;

                // For a given choice of cell, looping over possible
                // throws to find it's expected score.
                for (&(_, prob), &s) in DICE_DISTR.6.iter().zip(prev_layer_col)
                {
                    let prob = prob as f64 / DICE_DIVISOR[6] as f64;

                    expected_score = prob.mul_add(s as f64, expected_score);
                }

                if expected_score >= best_score {
                    best_score = expected_score;
                    best_cell_i = cell_i;
                }
            }

            *cur_score = best_score as f32;
            *cur_strat = best_cell_i as u8;
        },
    );

    (scores, strats)
}

pub fn solve_layer_6dice_throws(
    nb: usize,
    prev_throw_layer_scores: ArrayView3<f32>,
    mut scores: ArrayViewMut3<f32>,
    mut strats: ArrayViewMut3<u8>,
) {
    const N_DICE_THROWS: usize = DICE_DISTR.6.len();

    let n_bi = BELOW_LEVELS_6[nb].len();

    // x_b,t2 * A_t1r,t2 = b_b,t1r

    let a_mat = DICE_REROLL_MATRICES[5]
        .view()
        .into_shape([N_DICE_THROWS * 64, N_DICE_THROWS])
        .unwrap()
        .reversed_axes();

    let tls = ThreadLocal::new();

    scores
        .outer_iter_mut()
        .into_par_iter()
        .zip(strats.outer_iter_mut())
        .zip(prev_throw_layer_scores.outer_iter())
        .for_each(|((mut scores, mut strats), prev_scores)| {
            let buf_cell = tls.get_or(|| {
                Cell::new(Array2::from_shape_simple_fn(
                    [n_bi, N_DICE_THROWS * 64],
                    || 0.0,
                ))
            });

            let mut buf = buf_cell.take();

            general_mat_mul(1.0, &prev_scores, &a_mat, 0.0, &mut buf);

            let buf3 =
                buf.view().into_shape([n_bi, N_DICE_THROWS, 64]).unwrap();

            for ((score, strat), row) in
                scores.iter_mut().zip(strats.iter_mut()).zip(buf3.rows())
            {
                let mut best_score = *score;
                let mut do_reroll = false;
                let mut best_reroll = 0;

                for (i, &sc) in row.iter().enumerate() {
                    if sc > best_score {
                        best_score = sc;
                        do_reroll = true;
                        best_reroll = i;
                    }
                }

                if do_reroll {
                    *score = best_score;
                    *strat = best_reroll as u8 | 128;
                }
            }

            buf_cell.set(buf);
        });
}
