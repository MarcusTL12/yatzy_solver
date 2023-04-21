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

type AboveLevelsMapType = HashMap<(u32, [bool; 6]), usize>;

fn level_to_map(map: &mut AboveLevelsMapType, level: &[(u32, [bool; 6])]) {
    map.extend(level.iter().enumerate().map(|(i, &x)| (x, i)))
}

pub static ABOVE_LEVELS_5_MAP: Lazy<AboveLevelsMapType> = Lazy::new(|| {
    let mut map = AboveLevelsMapType::new();

    for l in ABOVE_LEVELS_5.iter() {
        level_to_map(&mut map, l);
    }

    map
});

pub static ABOVE_LEVELS_6_MAP: Lazy<AboveLevelsMapType> = Lazy::new(|| {
    let mut map = AboveLevelsMapType::new();

    for l in ABOVE_LEVELS_6.iter() {
        level_to_map(&mut map, l);
    }

    map
});
