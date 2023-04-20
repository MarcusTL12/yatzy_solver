use std::collections::HashSet;

use itertools::iproduct;
use once_cell::sync::Lazy;

pub static ABOVE_LEVELS_5: Lazy<[Vec<(u32, [bool; 6])>; 7]> = Lazy::new(|| {
    let mut levels = [(); 7].map(|_| HashSet::new());

    for (i1, i2, i3, i4, i5, i6) in
        iproduct!(-1..=5, -1..=5, -1..=5, -1..=5, -1..=5, -1..=5)
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
});

pub static ABOVE_LEVELS_6: Lazy<[Vec<(u32, [bool; 6])>; 7]> = Lazy::new(|| {
    let mut levels = [(); 7].map(|_| HashSet::new());

    for (i1, i2, i3, i4, i5, i6) in
        iproduct!(-1..=6, -1..=6, -1..=6, -1..=6, -1..=6, -1..=6)
    {
        let x = [i1, i2, i3, i4, i5, i6];
        let xb = x.map(|x| x >= 0);
        let xv = x.zip(xb).map(|(x, b)| x * b as i32);
        let xs = xv.zip([1, 2, 3, 4, 5, 6]).map(|(x, n)| x * n);

        let n = xb.into_iter().filter(|&x| x).count();
        let s = 84.min(xs.iter().sum()) as u32;

        levels[n].insert((s, xb));
    }

    levels.map(|s| {
        let mut v: Vec<_> = s.into_iter().collect();
        v.sort_unstable();
        v
    })
});
