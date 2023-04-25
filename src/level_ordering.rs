use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use itertools::iproduct;
use once_cell::sync::Lazy;

pub const fn points_above<const N: i32>() -> i32 {
    match N {
        5 => 63,
        6 => 84,
        _ => panic!("Amount of dice not supported!"),
    }
}

fn a_levels<const N: i32>() -> AboveLevelsType {
    let mut levels = [(); 7].map(|_| HashSet::new());

    for (i1, i2, i3, i4, i5, i6) in
        iproduct!(-1..=N, -1..=N, -1..=N, -1..=N, -1..=N, -1..=N)
    {
        let x = [i1, i2, i3, i4, i5, i6];
        let xb = x.map(|x| x >= 0);
        let xv = x.zip(xb).map(|(x, b)| x * b as i32);
        let xs = xv.zip([1, 2, 3, 4, 5, 6]).map(|(x, n)| x * n);

        let n = xb.into_iter().filter(|&x| x).count();
        let s = points_above::<N>().min(xs.iter().sum()) as usize;

        levels[n].insert((s, xb));
    }

    levels.map(|s| {
        let mut v: Vec<_> = s.into_iter().collect();
        v.sort_unstable();
        v
    })
}

fn extend_map<T: Eq + Hash + Copy>(map: &mut HashMap<T, usize>, level: &[T]) {
    map.extend(level.iter().enumerate().map(|(i, &x)| (x, i)))
}

fn make_map<const N: usize, T: Eq + Hash + Copy>(
    levels: &[Vec<T>; N],
) -> HashMap<T, usize> {
    let mut map = HashMap::new();

    for l in levels {
        extend_map(&mut map, l);
    }

    map
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

type AboveLevelsMapType = HashMap<(usize, [bool; 6]), usize>;

pub static ABOVE_LEVELS_5_MAP: Lazy<AboveLevelsMapType> =
    Lazy::new(|| make_map(&ABOVE_LEVELS_5));

pub static ABOVE_LEVELS_6_MAP: Lazy<AboveLevelsMapType> =
    Lazy::new(|| make_map(&ABOVE_LEVELS_6));

pub static BELOW_LEVELS_5: Lazy<[Vec<[bool; 9]>; 10]> = Lazy::new(b_levels);
pub static BELOW_LEVELS_6: Lazy<[Vec<[bool; 14]>; 15]> = Lazy::new(b_levels);

pub static BELOW_LEVELS_5_MAP: Lazy<HashMap<[bool; 9], usize>> =
    Lazy::new(|| make_map(&BELOW_LEVELS_5));
pub static BELOW_LEVELS_6_MAP: Lazy<HashMap<[bool; 14], usize>> =
    Lazy::new(|| make_map(&BELOW_LEVELS_6));
