use std::collections::{HashMap, HashSet};

use itertools::iproduct;
use once_cell::sync::Lazy;

fn make_levels(n: i32) -> [Vec<(u32, [bool; 6])>; 7] {
    let mut levels = [(); 7].map(|_| HashSet::new());

    for (i1, i2, i3, i4, i5, i6) in
        iproduct!(-1..=n, -1..=n, -1..=n, -1..=n, -1..=n, -1..=n)
    {
        let x = [i1, i2, i3, i4, i5, i6];
        let xb = x.map(|x| x >= 0);
        let xv = x.zip(xb).map(|(x, b)| x * b as i32);
        let xs = xv.zip([1, 2, 3, 4, 5, 6]).map(|(x, n)| x * n);

        let n = xb.into_iter().filter(|&x| x).count();
        let s = 63.min(xs.iter().sum()) as u32;

        levels[n].insert((s, xb));
    }

    levels.map(|s| {
        let mut v: Vec<_> = s.into_iter().collect();
        v.sort_unstable();
        v
    })
}

type AboveLevelsType = [Vec<(u32, [bool; 6])>; 7];

pub static ABOVE_LEVELS_5: Lazy<AboveLevelsType> = Lazy::new(|| make_levels(5));

pub static ABOVE_LEVELS_6: Lazy<AboveLevelsType> = Lazy::new(|| make_levels(6));

fn level_to_map(
    level: &[(u32, [bool; 6])],
) -> HashMap<(u32, [bool; 6]), usize> {
    level.iter().enumerate().map(|(i, &x)| (x, i)).collect()
}

type AboveLevelsMapType = [HashMap<(u32, [bool; 6]), usize>; 7];

pub static ABOVE_LEVELS_5_MAP: Lazy<AboveLevelsMapType> = Lazy::new(|| {
    [
        level_to_map(&ABOVE_LEVELS_5[0]),
        level_to_map(&ABOVE_LEVELS_5[1]),
        level_to_map(&ABOVE_LEVELS_5[2]),
        level_to_map(&ABOVE_LEVELS_5[3]),
        level_to_map(&ABOVE_LEVELS_5[4]),
        level_to_map(&ABOVE_LEVELS_5[5]),
        level_to_map(&ABOVE_LEVELS_5[6]),
    ]
});

pub static ABOVE_LEVELS_6_MAP: Lazy<AboveLevelsMapType> = Lazy::new(|| {
    [
        level_to_map(&ABOVE_LEVELS_6[0]),
        level_to_map(&ABOVE_LEVELS_6[1]),
        level_to_map(&ABOVE_LEVELS_6[2]),
        level_to_map(&ABOVE_LEVELS_6[3]),
        level_to_map(&ABOVE_LEVELS_6[4]),
        level_to_map(&ABOVE_LEVELS_6[5]),
        level_to_map(&ABOVE_LEVELS_6[6]),
    ]
});
