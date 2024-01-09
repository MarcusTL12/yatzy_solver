use std::{cmp::Ordering, ops::Add};

#[derive(PartialEq, Clone, Copy)]
pub struct Orderedf32(f32);

impl Eq for Orderedf32 {}

impl PartialOrd for Orderedf32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Orderedf32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Add<f32> for Orderedf32 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

fn mean_kernel(dist: &[f32]) -> f32 {
    dist.iter()
        .enumerate()
        .fold(0.0, |avg, (pts, prob)| prob.mul_add(pts as f32, avg))
}

fn quantile_kernel(dist: &[f32], q: f32) -> f32 {
    let mut acc = 0.0;

    let i = dist
        .iter()
        .enumerate()
        .find_map(|(i, x)| {
            acc += x;

            (acc > q).then_some(i)
        })
        .unwrap();

    if i == 0 {
        0.0
    } else {
        let l = dist[i - 1];
        let h = dist[i];
        let x = (q - l) / (h - l);

        (i - 1) as f32 + x
    }
}

pub fn mean(dist: &[f32]) -> Orderedf32 {
    Orderedf32(mean_kernel(dist))
}

#[derive(PartialEq, Clone, Copy)]
pub struct QuantileMean {
    q: f32,
    m: f32,
}

impl Eq for QuantileMean {}

impl PartialOrd for QuantileMean {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QuantileMean {
    fn cmp(&self, other: &Self) -> Ordering {
        const TOL: f32 = 1e-3;
        if (self.q - other.q).abs() > TOL {
            self.q.partial_cmp(&other.q)
        } else {
            self.m.partial_cmp(&other.m)
        }
        .unwrap()
    }
}

impl Add<f32> for QuantileMean {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            q: self.q + rhs,
            m: self.m + rhs,
        }
    }
}

pub fn quantile(q: f32) -> impl Fn(&[f32]) -> QuantileMean {
    move |dist| QuantileMean {
        q: quantile_kernel(dist, q),
        m: mean_kernel(dist),
    }
}
