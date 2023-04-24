use ndarray::Array3;

use crate::{
    dice_distributions::DICE_DISTR,
    dice_throw::DiceThrow,
    level_ordering::{ABOVE_LEVELS_5, BELOW_LEVELS_5},
    yatzy::State,
};

pub fn solve_layer_type1_5dice(
    na: usize,
    nb: usize,
    prev_layer_scores: &Array3<f32>,
) -> (Array3<f32>, Array3<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();

    let n_ai = ABOVE_LEVELS_5[na].len();
    let n_bi = BELOW_LEVELS_5[na].len();

    let shape = [n_ai, n_bi, N_DICE_THROWS];

    let mut scores = Array3::zeros(shape);
    let mut strats = Array3::zeros(shape);

    // TODO: Find nice way of parallellizing this

    for (ai, &(points_above, above_level)) in
        ABOVE_LEVELS_5[na].iter().enumerate()
    {
        for (bi, &below_level) in BELOW_LEVELS_5[nb].iter().enumerate() {
            let state =
                State::<15>::from((above_level, below_level, points_above));

            for (ti, &(throw, _)) in DICE_DISTR.5.iter().enumerate() {
                let throw = DiceThrow::from(throw);

                // This is the inner loop of which states that need to
                // be "solved".

                let mut best_score = -f32::INFINITY;
                let mut best_cell_i = 255;

                // Looping over the choices to make
                for (cell_i, _) in
                    state.cells.iter().enumerate().filter(|(_, &cell)| !cell)
                {
                    let (new_state, extra_score) =
                        state.set_cell(cell_i, throw);

                    let new_ai = new_state.get_above_index();
                    let new_bi = new_state.get_below_index();

                    // Expected score will be the extra score (guaranteed)
                    // plus the expected score based on what you might roll next
                    let mut expected_score = extra_score as f32;

                    // For a given choice of cell, looping over possible
                    // throws to find it's expected score.
                    for (new_ti, &(_, prob)) in DICE_DISTR.5.iter().enumerate()
                    {
                        let prob = prob as f32 / N_DICE_THROWS as f32;

                        expected_score +=
                            prob * prev_layer_scores[[new_ai, new_bi, new_ti]];
                    }

                    if expected_score > best_score {
                        best_score = expected_score;
                        best_cell_i = cell_i;
                    }
                }

                scores[[ai, bi, ti]] = best_score;
                strats[[ai, bi, ti]] = best_cell_i as u8;
            }
        }
    }

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
