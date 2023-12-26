use ndarray::{
    parallel::prelude::{IntoParallelRefMutIterator, ParallelIterator},
    Array3,
};

use crate::{
    dice_throw::DiceThrow,
    guide::{get_state_indices5, get_state_indices6, get_total_score},
    macrosolver::outcore::{
        make_thin_layers_5dice, make_thin_layers_6dice, Layer,
    },
    yatzy::cell_from_dice,
};

pub fn simulate_n_5(scores: &mut [u32]) {
    let mut layers = make_thin_layers_5dice();

    for layer in &mut layers {
        layer.as_mut().unwrap().load_strats();
    }

    scores.par_iter_mut().for_each(|score| {
        *score = get_total_score::<5>(&simulate_5(&layers)) as u32
    });
}

pub fn simulate_n_5_full(scores: &mut [[u32; 15]]) {
    let mut layers = make_thin_layers_5dice();

    for layer in &mut layers {
        layer.as_mut().unwrap().load_strats();
    }

    scores.par_iter_mut().for_each(|score| {
        for (score, somescore) in score.iter_mut().zip(simulate_5(&layers)) {
            *score = somescore.unwrap() as u32;
        }
    });
}

pub fn simulate_n_6(scores: &mut [u32]) {
    let mut layers = make_thin_layers_6dice();

    for layer in &mut layers {
        layer.as_mut().unwrap().load_strats();
    }

    scores.par_iter_mut().for_each(|score| {
        *score = get_total_score::<6>(&simulate_6(&layers)) as u32
    });
}

pub fn simulate_n_6_full(scores: &mut [[u32; 20]]) {
    let mut layers = make_thin_layers_6dice();

    for layer in &mut layers {
        layer.as_mut().unwrap().load_strats();
    }

    scores.par_iter_mut().for_each(|score| {
        for (score, somescore) in score.iter_mut().zip(simulate_6(&layers)) {
            *score = somescore.unwrap() as u32;
        }
    });
}

fn get_rethrow_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    throws_left: usize,
    points_above: usize,
    layers: &Array3<Option<Layer<N>>>,
) -> u8 {
    let [na, nb, _, _, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let ti = dice.get_index();

    let layer = layers[[na, nb, throws_left]].as_ref().unwrap();

    layer.strats.as_ref().unwrap()[[ai, bi, ti]]
}

fn get_cell_strat<const N: usize>(
    cells: &[bool],
    dice: &DiceThrow,
    points_above: usize,
    layers: &Array3<Option<Layer<N>>>,
) -> usize
where
    [(); cell_from_dice::<N>()]:,
    [(); cell_from_dice::<N>() - 6]:,
{
    let [na, nb, _, _, ai, bi] = match N {
        5 => get_state_indices5(cells, points_above),
        6 => get_state_indices6(cells, points_above),
        _ => panic!(),
    };

    let ti = dice.get_index();

    let layer = layers[[na, nb, 0]].as_ref().unwrap();

    layer.strats.as_ref().unwrap()[[ai, bi, ti]] as usize
}

fn simulate_5(layers: &Array3<Option<Layer<5>>>) -> [Option<usize>; 15] {
    let mut points = [None; 15];

    let mut dice = DiceThrow::throw(5);

    for _ in 0..15 {
        for throws_left in [2, 1] {
            let filled_cells = points.map(|x| x.is_some());
            let points_above =
                points.iter().take(6).filter_map(|x| x.as_ref()).sum();

            let reroll = get_rethrow_strat::<5>(
                &filled_cells,
                &dice,
                throws_left,
                points_above,
                layers,
            );

            let rethrow = DiceThrow::throw(reroll.count_ones() as usize);

            dice = dice.overwrite_reroll_dyn::<5>(
                reroll,
                &rethrow.into_ordered_dice().collect::<Vec<_>>(),
            );
        }

        let filled_cells = points.map(|x| x.is_some());
        let points_above =
            points.iter().take(6).filter_map(|x| x.as_ref()).sum();

        let ind =
            get_cell_strat::<5>(&filled_cells, &dice, points_above, layers);

        let score = dice.cell_score::<5>(ind);

        points[ind] = Some(score);

        dice = DiceThrow::throw(5);
    }

    points
}

fn simulate_6(layers: &Array3<Option<Layer<6>>>) -> [Option<usize>; 20] {
    let mut points = [None; 20];

    let mut dice = DiceThrow::throw(6);

    for _ in 0..20 {
        for throws_left in [2, 1] {
            let filled_cells = points.map(|x| x.is_some());
            let points_above =
                points.iter().take(6).filter_map(|x| x.as_ref()).sum();

            let reroll = get_rethrow_strat::<6>(
                &filled_cells,
                &dice,
                throws_left,
                points_above,
                layers,
            );

            let rethrow = DiceThrow::throw(reroll.count_ones() as usize);

            dice = dice.overwrite_reroll_dyn::<6>(
                reroll,
                &rethrow.into_ordered_dice().collect::<Vec<_>>(),
            );
        }

        let filled_cells = points.map(|x| x.is_some());
        let points_above =
            points.iter().take(6).filter_map(|x| x.as_ref()).sum();

        let ind =
            get_cell_strat::<6>(&filled_cells, &dice, points_above, layers);

        let score = dice.cell_score::<6>(ind);

        points[ind] = Some(score);

        dice = DiceThrow::throw(6);
    }

    points
}
