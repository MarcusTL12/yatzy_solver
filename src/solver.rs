use std::{array, cell::Cell, time::Instant};

use ndarray::{
    linalg::general_mat_mul,
    parallel::prelude::{
        IndexedParallelIterator, IntoParallelIterator, ParallelIterator,
    },
    s, Array2, Array3, Zip,
};
use once_cell::sync::Lazy;
use thread_local::ThreadLocal;

use crate::{
    dice_distributions::{
        dice_order_map_6, DICE_DISTR, DICE_DIVISOR, DICE_ORDER_MAP,
        DICE_REROLL_MATRICES,
    },
    dice_throw::DiceThrow,
    level_ordering::{
        ABOVE_LEVELS_5, ABOVE_LEVELS_6, BELOW_LEVELS_5, BELOW_LEVELS_6,
    },
    yatzy::State,
};

// This is the solver that finds which cell to put your points into when you
// have no throws left
// na and nb are the number of filled cells above and below the line.
pub fn solve_layer_type1_5dice(
    na: usize,
    nb: usize,
    prev_above_layer_scores: &Array3<f32>,
    prev_below_layer_scores: &Array3<f32>,
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

            // This is the inner loop of which states that need to
            // be "solved".

            let mut best_score = 0.0;
            let mut best_cell_i = 255;

            // Looping over the choices to make
            for (cell_i, _) in
                state.cells.iter().enumerate().filter(|(_, &cell)| !cell)
            {
                let (new_state, extra_score) = state.set_cell(cell_i, throw);

                let new_ai = new_state.get_above_index();
                let new_bi = new_state.get_below_index();

                let prev_layer = if cell_i < 6 {
                    prev_above_layer_scores
                } else {
                    prev_below_layer_scores
                };

                // Expected score will be the extra score (guaranteed)
                // plus the expected score based on what you might roll next
                let mut expected_score = extra_score as f64;

                // For a given choice of cell, looping over possible
                // throws to find it's expected score.
                for (new_ti, &(_, prob)) in DICE_DISTR.5.iter().enumerate() {
                    let prob = prob as f64 / DICE_DIVISOR[5] as f64;

                    expected_score = prob.mul_add(
                        *prev_layer
                            .get([new_ai, new_bi, new_ti])
                            .unwrap_or(&0.0) as f64,
                        expected_score,
                    );
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

fn loop_rerolls_5<const M: usize, const N: usize>(
    dice_distr: &[([u8; N], u32); M],
    throw: &DiceThrow,
    reroll: u8,
    prev_layer_scores: &Array3<f32>,
    ai: usize,
    bi: usize,
) -> f64 {
    let mut expected_score = 0.0;
    for &(rethrow, prob) in dice_distr {
        let new_throw = throw.overwrite_reroll::<5, N>(reroll, rethrow);

        let new_ti = DICE_ORDER_MAP.5[&new_throw.collect_dice()];

        let prob = prob as f64 / DICE_DIVISOR[N] as f64;

        expected_score = prob.mul_add(
            prev_layer_scores[[ai, bi, new_ti]] as f64,
            expected_score,
        );
    }
    expected_score
}

// This is the solver that finds dice to re-throw when having some number of
// throws left
pub fn solve_layer_type2_5dice(
    na: usize,
    nb: usize,
    prev_layer_scores: &Array3<f32>,
) -> (Array3<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();

    let n_ai = ABOVE_LEVELS_5[na].len();
    let n_bi = BELOW_LEVELS_5[nb].len();

    let shape = [n_ai, n_bi, N_DICE_THROWS];

    let mut scores = Array3::zeros(shape);
    let mut strats = Array3::zeros(shape);

    Zip::indexed(&mut scores).and(&mut strats).par_for_each(
        |(ai, bi, ti), cur_score, cur_strat| {
            let throw = DiceThrow::from(DICE_DISTR.5[ti].0);

            // This is the inner loop of which states that need to
            // be "solved".

            // Starting out with no rerolled
            let mut best_score = prev_layer_scores[[ai, bi, ti]] as f64;
            let mut best_reroll = 0;

            // Looping over possible rerolls
            for reroll in throw.into_mask_iter().skip(1) {
                let expected_score = match reroll.count_ones() {
                    1 => loop_rerolls_5(
                        &DICE_DISTR.1,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    2 => loop_rerolls_5(
                        &DICE_DISTR.2,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    3 => loop_rerolls_5(
                        &DICE_DISTR.3,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    4 => loop_rerolls_5(
                        &DICE_DISTR.4,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    5 => loop_rerolls_5(
                        &DICE_DISTR.5,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    _ => unreachable!(),
                };
                if expected_score > best_score {
                    best_score = expected_score;
                    best_reroll = reroll;
                }
            }

            *cur_score = best_score as f32;
            *cur_strat = best_reroll;
        },
    );

    (scores, strats)
}

// This returns the dimensions of the matrix for layer (6, 9, 0)
pub fn bottom_layer_dimensions_5dice() -> [usize; 3] {
    [
        ABOVE_LEVELS_5.last().unwrap().len(),
        BELOW_LEVELS_5.last().unwrap().len(),
        DICE_DISTR.5.len(),
    ]
}

// This is the solver that finds which cell to put your points into when you
// have no throws left
// na and nb are the number of filled cells above and below the line.
pub fn solve_layer_type1_6dice(
    na: usize,
    nb: usize,
    prev_above_layer_scores: &Array3<f32>,
    prev_below_layer_scores: &Array3<f32>,
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

            // This is the inner loop of which states that need to
            // be "solved".

            let mut best_score = 0.0;
            let mut best_cell_i = 255;

            // Looping over the choices to make
            for (cell_i, _) in
                state.cells.iter().enumerate().filter(|(_, &cell)| !cell)
            {
                let (new_state, extra_score) = state.set_cell(cell_i, throw);

                let new_ai = new_state.get_above_index();
                let new_bi = new_state.get_below_index();

                let prev_layer = if cell_i < 6 {
                    prev_above_layer_scores
                } else {
                    prev_below_layer_scores
                };

                // Expected score will be the extra score (guaranteed)
                // plus the expected score based on what you might roll next
                let mut expected_score = extra_score as f64;

                // For a given choice of cell, looping over possible
                // throws to find it's expected score.
                for (new_ti, &(_, prob)) in DICE_DISTR.6.iter().enumerate() {
                    let prob = prob as f64 / DICE_DIVISOR[6] as f64;

                    expected_score = prob.mul_add(
                        *prev_layer
                            .get([new_ai, new_bi, new_ti])
                            .unwrap_or(&0.0) as f64,
                        expected_score,
                    );
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

// Major bottleneck for computing strats
fn loop_rerolls_6<const M: usize, const N: usize>(
    dice_distr: &[([u8; N], u32); M],
    throw: &DiceThrow,
    reroll: u8,
    prev_layer_scores: &Array3<f32>,
    ai: usize,
    bi: usize,
) -> f64 {
    let mut expected_score = 0.0;
    for &(rethrow, prob) in dice_distr {
        let new_throw = throw.overwrite_reroll::<6, N>(reroll, rethrow);

        let new_ti = dice_order_map_6(new_throw.collect_dice());

        let prob = prob as f64 / DICE_DIVISOR[N] as f64;

        expected_score = prob.mul_add(
            prev_layer_scores[[ai, bi, new_ti]] as f64,
            expected_score,
        );
    }
    expected_score
}

// This is the solver that finds dice to re-throw when having some number of
// throws left
pub fn solve_layer_type2_6dice(
    na: usize,
    nb: usize,
    prev_layer_scores: &Array3<f32>,
) -> (Array3<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.6.len();

    let n_ai = ABOVE_LEVELS_6[na].len();
    let n_bi = BELOW_LEVELS_6[nb].len();

    let shape = [n_ai, n_bi, N_DICE_THROWS];

    let mut scores = Array3::zeros(shape);
    let mut strats = Array3::zeros(shape);

    Zip::indexed(&mut scores).and(&mut strats).par_for_each(
        |(ai, bi, ti), cur_score, cur_strat| {
            let throw = DiceThrow::from(DICE_DISTR.6[ti].0);

            // This is the inner loop of which states that need to
            // be "solved".

            // Starting out with no rerolled
            let mut best_score = prev_layer_scores[[ai, bi, ti]] as f64;
            let mut best_reroll = 0;

            // Looping over possible rerolls
            for reroll in throw.into_mask_iter().skip(1) {
                let expected_score = match reroll.count_ones() {
                    1 => loop_rerolls_6(
                        &DICE_DISTR.1,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    2 => loop_rerolls_6(
                        &DICE_DISTR.2,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    3 => loop_rerolls_6(
                        &DICE_DISTR.3,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    4 => loop_rerolls_6(
                        &DICE_DISTR.4,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    5 => loop_rerolls_6(
                        &DICE_DISTR.5,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    6 => loop_rerolls_6(
                        &DICE_DISTR.6,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    _ => unreachable!(),
                };
                if expected_score > best_score {
                    best_score = expected_score;
                    best_reroll = reroll;
                }
            }

            *cur_score = best_score as f32;
            *cur_strat = best_reroll;
        },
    );

    (scores, strats)
}

pub fn solve_layer_5dicex(
    na: usize,
    nb: usize,
    prev_above_layer_scores: &Array3<f32>,
    prev_below_layer_scores: &Array3<f32>,
    prev_throw_layer_scores: Option<&Array3<f32>>,
) -> (Array3<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();

    let n_ai = ABOVE_LEVELS_5[na].len();
    let n_bi = BELOW_LEVELS_5[nb].len();

    let shape = [n_ai, n_bi, N_DICE_THROWS];

    let mut scores = Array3::zeros(shape);
    let mut strats = Array3::zeros(shape);

    let timer = Instant::now();

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

                let new_ai = new_state.get_above_index();
                let new_bi = new_state.get_below_index();

                let prev_layer = if cell_i < 6 {
                    prev_above_layer_scores
                } else {
                    prev_below_layer_scores
                };

                // Expected score will be the extra score (guaranteed)
                // plus the expected score based on what you might roll next
                let mut expected_score = extra_score as f64;

                // For a given choice of cell, looping over possible
                // throws to find it's expected score.
                for (new_ti, &(_, prob)) in DICE_DISTR.5.iter().enumerate() {
                    let prob = prob as f64 / DICE_DIVISOR[5] as f64;

                    expected_score = prob.mul_add(
                        *prev_layer
                            .get([new_ai, new_bi, new_ti])
                            .unwrap_or(&0.0) as f64,
                        expected_score,
                    );
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

    println!("Cells took {:.2?}", timer.elapsed());

    let timer = Instant::now();

    // x_b,t2 * A_t1r,t2 = b_b,t1r

    let a_mat = DICE_REROLL_MATRICES[4]
        .view()
        .into_shape([N_DICE_THROWS * 32, N_DICE_THROWS])
        .unwrap()
        .reversed_axes();

    if let Some(prev_scores) = prev_throw_layer_scores {
        scores
            .outer_iter_mut()
            .into_par_iter()
            .zip(strats.outer_iter_mut())
            .zip(prev_scores.outer_iter())
            .for_each_init(
                || {
                    Array2::from_shape_simple_fn(
                        [n_bi, N_DICE_THROWS * 32],
                        || 0.0,
                    )
                },
                |buf, ((mut scores, mut strats), prev_scores)| {
                    general_mat_mul(1.0, &prev_scores, &a_mat, 0.0, buf);

                    let buf3 = buf
                        .view()
                        .into_shape([n_bi, N_DICE_THROWS, 32])
                        .unwrap();

                    for ((score, strat), row) in scores
                        .iter_mut()
                        .zip(strats.iter_mut())
                        .zip(buf3.rows())
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
                },
            );
    }

    println!("Rerolls took {:.2?}", timer.elapsed());

    (scores, strats)
}

type AboveLookupType = [Array2<[Option<[usize; 2]>; 6]>; 6];

static ABOVE_LOOKUP6: Lazy<AboveLookupType> = Lazy::new(|| {
    let mut res = array::from_fn(|na| {
        Array2::from_shape_simple_fn(
            [ABOVE_LEVELS_6[na].len(), DICE_DISTR.6.len()],
            || [None; 6],
        )
    });

    for na in 0..6 {
        for (ai, &(points_above, above_level)) in
            ABOVE_LEVELS_6[na].iter().enumerate()
        {
            let state =
                State::<20>::from((above_level, [false; 14], points_above));

            for (ti, &(tr, _)) in DICE_DISTR.6.iter().enumerate() {
                let throw = DiceThrow::from(tr);

                for cell_i in (0..6).filter(|&i| !above_level[i]) {
                    let (new_state, extra_score) =
                        state.set_cell(cell_i, throw);

                    let new_ai = new_state.get_above_index();

                    res[na][[ai, ti]][cell_i] = Some([new_ai, extra_score]);
                }
            }
        }
    }

    res
});

static BELOW_LOOKUP6: Lazy<[Array2<Option<usize>>; 14]> = Lazy::new(|| {
    let mut res = array::from_fn(|nb| {
        Array2::from_shape_simple_fn([BELOW_LEVELS_6[nb].len(), 14], || None)
    });

    for nb in 0..14 {
        for (bi, &below_level) in BELOW_LEVELS_6[nb].iter().enumerate() {
            for cell_i in (6..14 + 6).filter(|&i| !below_level[i - 6]) {
                let mut new_level = below_level;
                new_level[cell_i - 6] = true;
                let state = State::<20>::from(([false; 6], new_level, 0));

                let new_bi = state.get_below_index();

                res[nb][[bi, cell_i - 6]] = Some(new_bi);
            }
        }
    }

    res
});

static BELOW_PTS_LOOKUP6: Lazy<Array2<usize>> = Lazy::new(|| {
    Array2::from_shape_fn([DICE_DISTR.6.len(), 14], |(ti, cell_i)| {
        let throw = DiceThrow::from(DICE_DISTR.6[ti].0);

        throw.cell_score::<6>(cell_i + 6)
    })
});

pub fn solve_layer_6dicex(
    na: usize,
    nb: usize,
    prev_above_layer_scores: &Array3<f32>,
    prev_below_layer_scores: &Array3<f32>,
    prev_throw_layer_scores: Option<&Array3<f32>>,
) -> (Array3<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.6.len();

    let n_ai = ABOVE_LEVELS_6[na].len();
    let n_bi = BELOW_LEVELS_6[nb].len();

    let shape = [n_ai, n_bi, N_DICE_THROWS];

    let mut scores = Array3::zeros(shape);
    let mut strats = Array3::zeros(shape);

    let timer = Instant::now();

    let above_lookup = ABOVE_LOOKUP6[na].view();
    let below_lookup = BELOW_LOOKUP6[nb].view();

    Zip::indexed(&mut scores).and(&mut strats).par_for_each(
        |(ai, bi, ti), cur_score, cur_strat| {
            for (cell_i, [new_ai, extra_score]) in above_lookup[[ai, ti]]
                .iter()
                .enumerate()
                .filter_map(|(i, x)| x.map(|x| (i, x)))
            {
                let prev_scores =
                    prev_above_layer_scores.slice(s![new_ai, bi, ..]);

                let expected_score = DICE_DISTR
                    .6
                    .iter()
                    .zip(&prev_scores)
                    .fold(extra_score as f32, |score, (&(_, prob), x)| {
                        x.mul_add(prob as f32, score)
                    });

                if expected_score > *cur_score {
                    *cur_score = expected_score;
                    *cur_strat = cell_i as u8;
                }
            }

            let extra_scores = BELOW_PTS_LOOKUP6.slice(s![ti, ..]);

            for (cell_i_below, new_bi, extra_score) in below_lookup
                .slice(s![bi, ..])
                .iter()
                .zip(extra_scores)
                .enumerate()
                .filter_map(|(i, (x, &y))| x.map(|x| (i, x, y)))
            {
                let prev_scores =
                    prev_below_layer_scores.slice(s![ai, new_bi, ..]);

                let expected_score = DICE_DISTR
                    .6
                    .iter()
                    .zip(&prev_scores)
                    .fold(extra_score as f32, |score, (&(_, prob), x)| {
                        x.mul_add(prob as f32, score)
                    });

                if expected_score > *cur_score {
                    *cur_score = expected_score;
                    *cur_strat = (cell_i_below + 6) as u8;
                }
            }
        },
    );

    println!("Cells took {:.2?}", timer.elapsed());

    let timer = Instant::now();

    // x_b,t2 * A_t1r,t2 = b_b,t1r

    let a_mat = DICE_REROLL_MATRICES[5]
        .view()
        .into_shape([N_DICE_THROWS * 64, N_DICE_THROWS])
        .unwrap()
        .reversed_axes();

    let tls = ThreadLocal::new();

    if let Some(prev_scores) = prev_throw_layer_scores {
        scores
            .outer_iter_mut()
            .into_par_iter()
            .zip(strats.outer_iter_mut())
            .zip(prev_scores.outer_iter())
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

    println!("Rerolls took {:.2?}", timer.elapsed());

    (scores, strats)
}
