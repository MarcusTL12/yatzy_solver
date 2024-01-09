use std::{cell::Cell, ops::Add};

use ndarray::{
    linalg::general_mat_mul,
    parallel::prelude::{
        IndexedParallelIterator, IntoParallelIterator, ParallelIterator,
    },
    s, Array2, Array3, Array4, ArrayView3, ArrayView4, ArrayViewMut3,
    ArrayViewMut4, Zip,
};
use thread_local::ThreadLocal;

use crate::{
    dice_distributions::{DICE_DISTR, DICE_REROLL_MATRICES},
    dice_throw::DiceThrow,
    level_ordering::{ABOVE_LEVELS_5, BELOW_LEVELS_5},
    yatzy::State,
};

pub fn solve_layer_5dice_cells<
    O: Ord + Add<f32, Output = O> + Clone + Copy,
>(
    na: usize,
    nb: usize,
    prev_above_layer_dists: ArrayView3<f32>,
    prev_below_layer_dists: ArrayView3<f32>,
    measure: impl Fn(&[f32]) -> O + Send + Sync,
) -> (Array4<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();
    let n_ai: usize = ABOVE_LEVELS_5[na].len();
    let n_bi: usize = BELOW_LEVELS_5[nb].len();

    let (_, _, n_s) = prev_above_layer_dists.dim();

    let mut dists = Array4::zeros([n_ai, n_bi, N_DICE_THROWS, n_s]);
    let mut strats = Array3::zeros([n_ai, n_bi, N_DICE_THROWS]);

    Zip::indexed(dists.rows_mut())
        .and(&mut strats)
        .par_for_each(|(ai, bi, ti), mut cur_dist, cur_strat| {
            let (points_above, above_level) = ABOVE_LEVELS_5[na][ai];
            let below_level = BELOW_LEVELS_5[nb][bi];
            let throw = DiceThrow::from(DICE_DISTR.5[ti].0);
            let state =
                State::<15>::from((above_level, below_level, points_above));

            let (new_m, cell_i, dist) = state
                .cells
                .iter()
                .enumerate()
                .filter(|(_, &cell)| !cell)
                .map(|(cell_i, _)| {
                    let (new_state, extra_score) =
                        state.set_cell(cell_i, throw);

                    let mut new_ai = ai;
                    let mut new_bi = bi;

                    let prev_layer = if cell_i < 6 {
                        new_ai = new_state.get_above_index();
                        prev_above_layer_dists.view()
                    } else {
                        new_bi = new_state.get_below_index();
                        prev_below_layer_dists.view()
                    };

                    let prev_layer_dist =
                        prev_layer.slice_move(s![new_ai, new_bi, ..]);

                    let m = measure(prev_layer_dist.as_slice().unwrap())
                        + extra_score as f32;

                    (m, cell_i, prev_layer_dist)
                })
                .max_by_key(|(m, _, _)| *m)
                .unwrap();

            if new_m > measure(cur_dist.as_slice().unwrap()) {
                *cur_strat = cell_i as u8;
                cur_dist.assign(&dist);
            }
        });

    (dists, strats)
}

pub fn solve_layer_5dice_throws<O: Ord>(
    prev_dists: ArrayView4<f32>,
    dists: ArrayViewMut4<f32>,
    strats: ArrayViewMut3<u8>,
    measure: impl Fn(&[f32]) -> O + Send + Sync,
) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();
    const N_DICE_CHOICE: usize = 2usize.pow(5);

    let (n_ai, n_bi, _, n_s) = prev_dists.dim();

    let prev_dists_packed = prev_dists
        .into_shape([n_ai * n_bi, N_DICE_THROWS, n_s])
        .unwrap();

    let mut dists_packed =
        dists.into_shape([n_ai * n_bi, N_DICE_THROWS, n_s]).unwrap();
    let mut strats_packed =
        strats.into_shape([n_ai * n_bi, N_DICE_THROWS]).unwrap();

    let a_mat = DICE_REROLL_MATRICES[4]
        .view()
        .into_shape([N_DICE_THROWS * N_DICE_CHOICE, N_DICE_THROWS])
        .unwrap();

    let tls = ThreadLocal::new();

    dists_packed
        .outer_iter_mut()
        .into_par_iter()
        .zip(strats_packed.outer_iter_mut())
        .zip(prev_dists_packed.outer_iter())
        .for_each(|((mut dists, strats), prev_dists)| {
            let buf_cell = tls.get_or(|| {
                Cell::new(Array2::from_shape_simple_fn(
                    [n_s, N_DICE_THROWS * N_DICE_CHOICE],
                    || 0.0,
                ))
            });

            let mut buf = buf_cell.take();

            // b_tr,s = A_tr,t2 * x_t2,s
            general_mat_mul(1.0, &a_mat, &prev_dists, 0.0, &mut buf);

            let buf3 = buf
                .view()
                .into_shape([N_DICE_THROWS, N_DICE_CHOICE, n_s])
                .unwrap();

            for ((mut dist, strat), buf2) in
                dists.outer_iter_mut().zip(strats).zip(buf3.outer_iter())
            {
                let (r, new_dist) = buf2
                    .outer_iter()
                    .enumerate()
                    .max_by_key(|(_, dist)| measure(dist.as_slice().unwrap()))
                    .unwrap();

                if measure(new_dist.as_slice().unwrap())
                    > measure(dist.as_slice().unwrap())
                {
                    *strat = r as u8 | 128;
                    dist.assign(&new_dist);
                }
            }

            buf_cell.set(buf);
        });
}
