use std::collections::HashSet;

use arrayvec::ArrayVec;
use itertools::iproduct;
use ndarray::Array2;
use once_cell::sync::Lazy;

use crate::util::parse_binary;

pub const fn points_above<const N: usize>() -> usize {
    match N {
        5 => 63,
        6 => 84,
        _ => panic!("Amount of dice not supported!"),
    }
}

fn a_levels<const N: usize>() -> AboveLevelsType {
    let ni = N as i32;

    let mut levels = [(); 7].map(|_| HashSet::new());

    for (i1, i2, i3, i4, i5, i6) in
        iproduct!(-1..=ni, -1..=ni, -1..=ni, -1..=ni, -1..=ni, -1..=ni)
    {
        let x = [i1, i2, i3, i4, i5, i6];
        let xb = x.map(|x| x >= 0);
        let xv = x
            .iter()
            .zip(&xb)
            .map(|(x, &b)| x * b as i32)
            .collect::<ArrayVec<_, 6>>()
            .into_inner()
            .unwrap();

        let xs = xv
            .iter()
            .zip([1, 2, 3, 4, 5, 6])
            .map(|(x, n)| x * n)
            .collect::<ArrayVec<_, 6>>()
            .into_inner()
            .unwrap();

        let n = xb.into_iter().filter(|&x| x).count();
        let s = points_above::<N>().min(xs.iter().sum::<i32>() as usize);

        levels[n].insert((s, xb));
    }

    levels.map(|s| {
        let mut v: Vec<_> = s.into_iter().collect();
        v.sort_unstable();
        v
    })
}

fn next_state<const N: usize>(state: &mut [bool; N]) -> bool {
    for b in state.iter_mut() {
        *b = !*b;
        if *b {
            break;
        }
    }

    [false; N] == *state
}

fn b_levels<const N: usize>() -> [Vec<[bool; N]>; N + 1] {
    let mut levels = [(); N + 1].map(|_| Vec::new());

    let mut state = [false; N];

    loop {
        let n = state.into_iter().filter(|&x| x).count();

        levels[n].push(state);

        if next_state(&mut state) {
            break;
        }
    }

    levels
}

// Defining lookup tables:

type AboveLevelsType = [Vec<(usize, [bool; 6])>; 7];

pub static ABOVE_LEVELS_5: Lazy<AboveLevelsType> = Lazy::new(a_levels::<5>);
pub static ABOVE_LEVELS_6: Lazy<AboveLevelsType> = Lazy::new(a_levels::<6>);

pub static ABOVE_LEVELS_5_MAP: Lazy<Array2<usize>> = Lazy::new(|| {
    let mut map = Array2::zeros([64, 2usize.pow(6)]);

    for v in ABOVE_LEVELS_5.iter() {
        for (i, &(p, b)) in v.iter().enumerate() {
            let j = parse_binary(&b);

            map[[p, j]] = i;
        }
    }

    map
});

pub static ABOVE_LEVELS_6_MAP: Lazy<Array2<usize>> = Lazy::new(|| {
    let mut map = Array2::zeros([85, 2usize.pow(6)]);

    for v in ABOVE_LEVELS_6.iter() {
        for (i, &(p, b)) in v.iter().enumerate() {
            let j = parse_binary(&b);

            map[[p, j]] = i;
        }
    }

    map
});

pub static BELOW_LEVELS_5: Lazy<[Vec<[bool; 9]>; 10]> = Lazy::new(b_levels);
pub static BELOW_LEVELS_6: Lazy<[Vec<[bool; 14]>; 15]> = Lazy::new(b_levels);

pub static BELOW_LEVELS_5_MAP: Lazy<Vec<usize>> = Lazy::new(|| {
    let mut map = vec![0; 2usize.pow(9)];

    for v in BELOW_LEVELS_5.iter() {
        for (i, b) in v.iter().enumerate() {
            let j = parse_binary(b);

            map[j] = i;
        }
    }

    map
});

pub static BELOW_LEVELS_6_MAP: Lazy<Vec<usize>> = Lazy::new(|| {
    let mut map = vec![0; 2usize.pow(14)];

    for v in BELOW_LEVELS_6.iter() {
        for (i, b) in v.iter().enumerate() {
            let j = parse_binary(b);

            map[j] = i;
        }
    }

    map
});
