use ndarray::{Array3, Zip};

use crate::{
    dice_distributions::{DICE_DISTR, DICE_DIVISOR, DICE_ORDER_MAP},
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

// This is the solver that finds dice to re-throw when having some number of
// throws left
pub fn solve_layer_type2_5dice(
    na: usize,
    nb: usize,
    prev_layer_scores: &Array3<f32>,
) -> (Array3<f32>, Array3<u8>) {
    fn loop_rerolls<const M: usize, const N: usize>(
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
                    1 => loop_rerolls(
                        &DICE_DISTR.1,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    2 => loop_rerolls(
                        &DICE_DISTR.2,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    3 => loop_rerolls(
                        &DICE_DISTR.3,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    4 => loop_rerolls(
                        &DICE_DISTR.4,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    5 => loop_rerolls(
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

// This is the solver that finds dice to re-throw when having some number of
// throws left
pub fn solve_layer_type2_6dice(
    na: usize,
    nb: usize,
    prev_layer_scores: &Array3<f32>,
) -> (Array3<f32>, Array3<u8>) {
    fn loop_rerolls<const M: usize, const N: usize>(
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

            let new_ti = DICE_ORDER_MAP.6[&new_throw.collect_dice()];

            let prob = prob as f64 / DICE_DIVISOR[N] as f64;

            expected_score = prob.mul_add(
                prev_layer_scores[[ai, bi, new_ti]] as f64,
                expected_score,
            );
        }
        expected_score
    }

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
                    1 => loop_rerolls(
                        &DICE_DISTR.1,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    2 => loop_rerolls(
                        &DICE_DISTR.2,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    3 => loop_rerolls(
                        &DICE_DISTR.3,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    4 => loop_rerolls(
                        &DICE_DISTR.4,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    5 => loop_rerolls(
                        &DICE_DISTR.5,
                        &throw,
                        reroll,
                        prev_layer_scores,
                        ai,
                        bi,
                    ),
                    6 => loop_rerolls(
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
