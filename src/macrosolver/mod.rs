use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};

use ndarray::Array3;
use once_cell::sync::Lazy;

use crate::{
    dice_distributions::DICE_DISTR,
    level_ordering::{
        ABOVE_LEVELS_5, ABOVE_LEVELS_6, BELOW_LEVELS_5, BELOW_LEVELS_6,
    },
};

pub mod distr;
mod measures;
pub mod normal;
pub mod saving;

pub static PREFIX: Lazy<String> =
    Lazy::new(|| std::env::var("YATZY_CACHE").unwrap_or("cache".to_owned()));

pub struct Layer<const N: usize, const X: bool> {
    pub id: String,
    pub na: usize,
    pub nb: usize,
    pub nt: usize,
    pub scores: Option<Array3<f32>>,
    pub strats: Option<Array3<u8>>,
}

fn floats_to_bytes(floats: &[f32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            floats.as_ptr() as *const u8,
            floats.len() * 4,
        )
    }
}

fn floats_to_bytes_mut(floats: &mut [f32]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut(
            floats.as_ptr() as *mut u8,
            floats.len() * 4,
        )
    }
}

impl<const N: usize, const X: bool> Layer<N, X> {
    pub fn empty(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            na: 0,
            nb: 0,
            nt: 0,
            scores: Some(Array3::zeros([0; 3])),
            strats: Some(Array3::zeros([0; 3])),
        }
    }

    pub fn name(&self) -> String {
        format!("{}_{}_{}.dat", self.na, self.nb, self.nt)
    }

    pub fn scores_path(&self) -> String {
        if X {
            format!("{}/{}{N}x/scores/{}", &*PREFIX, self.id, &self.name())
        } else {
            format!("{}/{}{N}/scores/{}", &*PREFIX, self.id, &self.name())
        }
    }

    pub fn strats_path(&self) -> String {
        if X {
            format!("{}/{}{N}x/strats/{}", &*PREFIX, self.id, &self.name())
        } else {
            format!("{}/{}{N}/strats/{}", &*PREFIX, self.id, &self.name())
        }
    }

    pub fn save_scores(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.scores_path())
            .unwrap();

        let data = self.scores.as_ref().unwrap().as_slice().unwrap();
        let bytes = floats_to_bytes(data);

        file.write_all(bytes).unwrap();
    }

    pub fn save_strats(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.strats_path())
            .unwrap();

        let data = self.strats.as_ref().unwrap().as_slice().unwrap();

        file.write_all(data).unwrap();
    }

    pub fn is_done(&self) -> bool {
        Path::new(&self.scores_path()).exists()
            && Path::new(&self.strats_path()).exists()
    }
}

impl<const X: bool> Layer<5, X> {
    pub fn load_scores(&mut self) -> Option<()> {
        if self.scores.is_none() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(self.scores_path())
                .ok()?;

            let mut scores = Array3::zeros([
                ABOVE_LEVELS_5[self.na].len(),
                BELOW_LEVELS_5[self.nb].len(),
                DICE_DISTR.5.len(),
            ]);

            let data = scores.as_slice_mut().unwrap();

            let bytes: &mut [u8] = floats_to_bytes_mut(data);

            file.read_exact(bytes).unwrap();

            self.scores = Some(scores);
        }

        Some(())
    }

    pub fn load_dists(&mut self, n_s: usize) -> Option<()> {
        if self.scores.is_none() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(self.scores_path())
                .ok()?;

            let mut scores = Array3::zeros([
                ABOVE_LEVELS_5[self.na].len(),
                BELOW_LEVELS_5[self.nb].len(),
                n_s,
            ]);

            let data = scores.as_slice_mut().unwrap();

            let bytes: &mut [u8] = floats_to_bytes_mut(data);

            file.read_exact(bytes).unwrap();

            self.scores = Some(scores);
        }

        Some(())
    }

    pub fn load_strats(&mut self) -> Option<()> {
        if self.strats.is_none() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(self.strats_path())
                .ok()?;

            let mut strats = Array3::zeros([
                ABOVE_LEVELS_5[self.na].len(),
                BELOW_LEVELS_5[self.nb].len(),
                DICE_DISTR.5.len(),
            ]);

            let data = strats.as_slice_mut().unwrap();

            file.read_exact(data).unwrap();

            self.strats = Some(strats);
        }

        Some(())
    }
}

impl<const X: bool> Layer<6, X> {
    pub fn load_scores(&mut self) -> Option<()> {
        if self.scores.is_none() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(self.scores_path())
                .ok()?;

            let mut scores = Array3::zeros([
                ABOVE_LEVELS_6[self.na].len(),
                BELOW_LEVELS_6[self.nb].len(),
                DICE_DISTR.6.len(),
            ]);

            let data = scores.as_slice_mut().unwrap();

            let bytes: &mut [u8] = floats_to_bytes_mut(data);

            file.read_exact(bytes).unwrap();

            self.scores = Some(scores);
        }

        Some(())
    }

    pub fn load_strats(&mut self) -> Option<()> {
        if self.strats.is_none() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(self.strats_path())
                .ok()?;

            let mut strats = Array3::zeros([
                ABOVE_LEVELS_6[self.na].len(),
                BELOW_LEVELS_6[self.nb].len(),
                DICE_DISTR.6.len(),
            ]);

            let data = strats.as_slice_mut().unwrap();

            file.read_exact(data).unwrap();

            self.strats = Some(strats);
        }

        Some(())
    }
}

pub fn make_thin_layers_5dice(id: &str) -> Array3<Option<Layer<5, false>>> {
    Array3::from_shape_fn([7, 10, 3], |(na, nb, nt)| {
        Some(Layer {
            id: id.to_owned(),
            na,
            nb,
            nt,
            scores: None,
            strats: None,
        })
    })
}

pub fn make_thin_layers_6dice(id: &str) -> Array3<Option<Layer<6, false>>> {
    Array3::from_shape_fn([7, 15, 3], |(na, nb, nt)| {
        Some(Layer {
            id: id.to_owned(),
            na,
            nb,
            nt,
            scores: None,
            strats: None,
        })
    })
}
