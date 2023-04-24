use ndarray::{Array3, Zip};

use crate::{
    dice_distributions::DICE_DISTR,
    dice_throw::DiceThrow,
    level_ordering::{ABOVE_LEVELS_5, BELOW_LEVELS_5},
    yatzy::State,
};

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

            let mut best_score = -f32::INFINITY;
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
                let mut expected_score = extra_score as f32;

                // For a given choice of cell, looping over possible
                // throws to find it's expected score.
                for (new_ti, &(_, prob)) in DICE_DISTR.5.iter().enumerate() {
                    let prob = prob as f32 / N_DICE_THROWS as f32;

                    expected_score +=
                        prob * prev_layer[[new_ai, new_bi, new_ti]];
                }

                if expected_score > best_score {
                    best_score = expected_score;
                    best_cell_i = cell_i;
                }
            }

            *cur_score = best_score;
            *cur_strat = best_cell_i as u8;
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
