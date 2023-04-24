use crate::{
    dice_distributions::DICE_DISTR,
    dice_throw::DiceThrow,
    level_ordering::{ABOVE_LEVELS_5, BELOW_LEVELS_5},
    yatzy::State,
};

// generated using the 'make_counts_x()' functions in
// 'creating_lookup_tables.jl'
const LAYER_SIZES_5: [usize; 16] = [
    1, 45, 756, 5932, 27224, 82211, 174355, 269592, 310209, 267883, 173274,
    82860, 28486, 6677, 959, 64,
];
// const LAYER_SIZES_6: [usize; 21] = [
//     1, 56, 1162, 12149, 78245, 346302, 1123144, 2779030, 5385198, 8313214,
//     10332036, 10395320, 8475974, 5582474, 2946168, 1228570, 395857, 95106,
//     16050, 1699, 85,
// ];

pub fn solve_layer_type1_5dice(
    layer_n: usize,
    prev_layer_scores: &[f32],
) -> (Vec<f32>, Vec<u8>) {
    const N_DICE_THROWS: usize = DICE_DISTR.5.len();

    let mut scores = vec![0.0; LAYER_SIZES_5[layer_n] * N_DICE_THROWS];
    let mut strats = vec![0; LAYER_SIZES_5[layer_n] * N_DICE_THROWS];

    // TODO: Find nice way of parallellizing this

    for above_level_n in 0..=6 {
        if layer_n >= above_level_n
            && layer_n - above_level_n < ABOVE_LEVELS_5.len()
        {
            let below_level_n = layer_n - above_level_n;

            for (above_level_i, &(points_above, above_level)) in
                ABOVE_LEVELS_5[above_level_n].iter().enumerate()
            {
                for (below_level_i, &below_level) in
                    BELOW_LEVELS_5[below_level_n].iter().enumerate()
                {
                    let state = State::<15>::from((
                        above_level,
                        below_level,
                        points_above,
                    ));

                    // let state_i = state.get_index();
                    let state_i = below_level_i
                        * ABOVE_LEVELS_5[above_level_n].len()
                        + above_level_i;

                    for (throw_i, &(throw, _)) in
                        DICE_DISTR.5.iter().enumerate()
                    {
                        let throw = DiceThrow::from(throw);

                        // This is the inner loop of which states that need to
                        // be "solved".

                        let total_i = state_i * N_DICE_THROWS + throw_i;

                        let mut best_score = -f32::INFINITY;
                        let mut best_cell_i = 255;

                        // Looping over the choices to make
                        for (cell_i, _) in state
                            .cells
                            .iter()
                            .enumerate()
                            .filter(|(_, &cell)| cell)
                        {
                            let (new_state, extra_score) =
                                state.set_cell(cell_i, throw);

                            let new_state_i = new_state.get_index();

                            let mut expected_score = extra_score as f32;

                            if extra_score > 0 {
                                println!("{extra_score}");
                            }

                            // For a given choice of cell, looping over possible
                            // throws to find it's expected score.
                            for (new_throw_i, &(_, prob)) in
                                DICE_DISTR.5.iter().enumerate()
                            {
                                let new_total_i =
                                    new_state_i * N_DICE_THROWS + new_throw_i;

                                let prob = prob as f32 / N_DICE_THROWS as f32;

                                expected_score +=
                                    prob * prev_layer_scores[new_total_i];
                            }

                            if expected_score > best_score {
                                best_score = expected_score;
                                best_cell_i = cell_i;
                            }
                        }

                        scores[total_i] = best_score;
                        strats[total_i] = best_cell_i as u8;
                    }
                }
            }
        }
    }

    (scores, strats)
}

pub fn make_bottom_layer_5dice() -> Vec<f32> {
    vec![0.0; DICE_DISTR.5.len() * LAYER_SIZES_5.last().unwrap()]
}
