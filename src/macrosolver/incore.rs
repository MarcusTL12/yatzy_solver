// This is a quick draft for the system to build the layers that does not
// deal with any. This is mainly for testing.

use std::time::Instant;

use ndarray::Array3;

use crate::solver::{solve_layer_type1_5dice, solve_layer_type2_5dice};

type SolvedType = (Array3<Option<Array3<f32>>>, Array3<Option<Array3<u8>>>);

pub fn solve_5dice() -> SolvedType {
    let mut scores = Array3::from_elem([7, 10, 3], None);
    let mut strats = Array3::from_elem([7, 10, 3], None);

    let global_timer = Instant::now();

    for na in (5..7).rev() {
        for nb in (8..10).rev() {
            let s0_1 = Some(Array3::zeros([0; 3]));
            let s0_2 = Some(Array3::zeros([0; 3]));
            let prev_above_layer_scores = scores
                .get([na + 1, nb, 2])
                .unwrap_or(&s0_1)
                .as_ref()
                .unwrap();
            let prev_below_layer_scores = scores
                .get([na, nb + 1, 2])
                .unwrap_or(&s0_2)
                .as_ref()
                .unwrap();

            println!("na: {na:2}, nb: {nb:2}, nt: 0");
            let timer = Instant::now();

            let (l_scores, l_strats) = solve_layer_type1_5dice(
                na,
                nb,
                prev_above_layer_scores,
                prev_below_layer_scores,
            );

            scores[[na, nb, 0]] = Some(l_scores);
            strats[[na, nb, 0]] = Some(l_strats);

            let t = timer.elapsed();
            println!("Took: {t:4.2?}");

            for nt in 1..3 {
                println!("na: {na:2}, nb: {nb:2}, nt: {nt}");
                let timer = Instant::now();

                let prev_layer_scores =
                    scores.get([na, nb, nt - 1]).unwrap().as_ref().unwrap();

                let (l_scores, l_strats) =
                    solve_layer_type2_5dice(na, nb, prev_layer_scores);

                scores[[na, nb, nt]] = Some(l_scores);
                strats[[na, nb, nt]] = Some(l_strats);

                let t = timer.elapsed();
                println!("Took: {t:4.2?}");
            }
        }
    }

    let t = global_timer.elapsed();
    println!("Total time: {t:6.2?}");

    (scores, strats)
}
