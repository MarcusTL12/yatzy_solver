// This is the module for dealing with a proper system that saves incrementally
// to disk and can be paused and restarted.

use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Read, Write},
    path::Path,
    time::{Duration, Instant},
};

use ndarray::Array3;
use once_cell::sync::Lazy;

use crate::{
    dice_distributions::DICE_DISTR,
    level_ordering::{
        ABOVE_LEVELS_5, ABOVE_LEVELS_6, BELOW_LEVELS_5, BELOW_LEVELS_6,
    },
    solver::{
        solve_layer_type1_5dice, solve_layer_type1_6dice,
        solve_layer_type2_5dice, solve_layer_type2_6dice,
    },
};

pub static PREFIX: Lazy<String> =
    Lazy::new(|| std::env::var("YATZY_CACHE").unwrap_or("cache".to_owned()));

pub struct Layer<const N: usize, const X: bool> {
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
    pub fn empty() -> Self {
        Self {
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
            format!("{}/{N}x/scores/{}", &*PREFIX, &self.name())
        } else {
            format!("{}/{N}/scores/{}", &*PREFIX, &self.name())
        }
    }

    pub fn strats_path(&self) -> String {
        if X {
            format!("{}/{N}x/strats/{}", &*PREFIX, &self.name())
        } else {
            format!("{}/{N}/strats/{}", &*PREFIX, &self.name())
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

pub fn make_thin_layers_5dice() -> Array3<Option<Layer<5, false>>> {
    Array3::from_shape_fn([7, 10, 3], |(na, nb, nt)| {
        Some(Layer {
            na,
            nb,
            nt,
            scores: None,
            strats: None,
        })
    })
}

pub fn make_thin_layers_6dice() -> Array3<Option<Layer<6, false>>> {
    Array3::from_shape_fn([7, 15, 3], |(na, nb, nt)| {
        Some(Layer {
            na,
            nb,
            nt,
            scores: None,
            strats: None,
        })
    })
}

pub fn solve_5dice() {
    create_dir_all(format!("{}/5/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/5/strats/", *PREFIX)).unwrap();

    let mut layers = make_thin_layers_5dice();

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    for na in (0..7).rev() {
        for nb in (0..10).rev() {
            println!("=============================");
            println!("na: {na:2}, nb: {nb:2}, nt: 0");

            let mut layer: Layer<5, false> =
                layers[[na, nb, 0]].take().unwrap();

            if layer.is_done() {
                println!("Already done!");
            } else {
                let mut prev_above_layer = layers
                    .get_mut([na + 1, nb, 2])
                    .unwrap_or(&mut Some(Layer::empty()))
                    .take()
                    .unwrap();
                let mut prev_below_layer = layers
                    .get_mut([na, nb + 1, 2])
                    .unwrap_or(&mut Some(Layer::empty()))
                    .take()
                    .unwrap();

                let timer = Instant::now();

                prev_above_layer.load_scores();
                prev_below_layer.load_scores();

                let t = timer.elapsed();
                println!("Loading took {t:.2?}");
                load_timer += t;

                let timer = Instant::now();

                let (scores, strats) = solve_layer_type1_5dice(
                    na,
                    nb,
                    &prev_above_layer.scores.unwrap(),
                    &prev_below_layer.scores.unwrap(),
                );

                let t = timer.elapsed();
                println!("Solving took {t:.2?}");
                compute_timer += t;

                prev_above_layer.scores = None;
                prev_below_layer.scores = None;

                if let Some(x) = layers.get_mut([na + 1, nb, 2]) {
                    *x = Some(prev_above_layer);
                }

                if let Some(x) = layers.get_mut([na, nb + 1, 2]) {
                    *x = Some(prev_below_layer);
                }

                layer.scores = Some(scores);
                layer.strats = Some(strats);

                let timer = Instant::now();

                layer.save_scores();
                layer.save_strats();

                let t = timer.elapsed();
                println!("Saving took  {t:.2?}");
                save_timer += t;
            }

            layer.strats = None;
            layers[[na, nb, 0]] = Some(layer);

            for nt in 1..3 {
                println!("--------------------------------");
                println!("na: {na:2}, nb: {nb:2}, nt: {nt}");

                let mut layer = layers[[na, nb, nt]].take().unwrap();

                if layer.is_done() {
                    println!("Already done!");
                } else {
                    let mut prev_layer =
                        layers[[na, nb, nt - 1]].take().unwrap();

                    let timer = Instant::now();

                    prev_layer.load_scores();

                    let t = timer.elapsed();
                    println!("Loading took {t:.2?}");
                    load_timer += t;

                    let timer = Instant::now();

                    let (scores, strats) = solve_layer_type2_5dice(
                        na,
                        nb,
                        &prev_layer.scores.unwrap(),
                    );

                    let t = timer.elapsed();
                    println!("Solving took {t:.2?}");
                    compute_timer += t;

                    prev_layer.scores = None;

                    layers[[na, nb, nt - 1]] = Some(prev_layer);

                    layer.scores = Some(scores);
                    layer.strats = Some(strats);

                    let timer = Instant::now();

                    layer.save_scores();
                    layer.save_strats();

                    let t = timer.elapsed();
                    println!("Saving took  {t:.2?}");
                    save_timer += t;
                }

                layer.strats = None;
                layers[[na, nb, nt]] = Some(layer);
            }
        }
    }

    println!("\n\nTotal   time: {:.2?}", global_timer.elapsed());
    println!("Compute time: {compute_timer:.2?}");
    println!("Loading time: {load_timer:.2?}");
    println!("Saving  time: {save_timer:.2?}");
}

pub fn solve_6dice() {
    create_dir_all(format!("{}/6/scores/", *PREFIX)).unwrap();
    create_dir_all(format!("{}/6/strats/", *PREFIX)).unwrap();

    let mut layers = make_thin_layers_6dice();

    let global_timer = Instant::now();
    let mut load_timer = Duration::ZERO;
    let mut save_timer = Duration::ZERO;
    let mut compute_timer = Duration::ZERO;

    for na in (0..7).rev() {
        for nb in (0..15).rev() {
            println!("=============================");
            println!("na: {na:2}, nb: {nb:2}, nt: 0");

            let mut layer = layers[[na, nb, 0]].take().unwrap();

            if layer.is_done() {
                println!("Already done!");
            } else {
                let mut prev_above_layer = layers
                    .get_mut([na + 1, nb, 2])
                    .unwrap_or(&mut Some(Layer::empty()))
                    .take()
                    .unwrap();
                let mut prev_below_layer = layers
                    .get_mut([na, nb + 1, 2])
                    .unwrap_or(&mut Some(Layer::empty()))
                    .take()
                    .unwrap();

                let timer = Instant::now();

                prev_above_layer.load_scores();
                prev_below_layer.load_scores();

                let t = timer.elapsed();
                println!("Loading took {t:.2?}");
                load_timer += t;

                let timer = Instant::now();

                let (scores, strats) = solve_layer_type1_6dice(
                    na,
                    nb,
                    &prev_above_layer.scores.unwrap(),
                    &prev_below_layer.scores.unwrap(),
                );

                let t = timer.elapsed();
                println!("Solving took {t:.2?}");
                compute_timer += t;

                prev_above_layer.scores = None;
                prev_below_layer.scores = None;

                if let Some(x) = layers.get_mut([na + 1, nb, 2]) {
                    *x = Some(prev_above_layer);
                }

                if let Some(x) = layers.get_mut([na, nb + 1, 2]) {
                    *x = Some(prev_below_layer);
                }

                layer.scores = Some(scores);
                layer.strats = Some(strats);

                let timer = Instant::now();

                layer.save_scores();
                layer.save_strats();

                let t = timer.elapsed();
                println!("Saving  took {t:.2?}");
                save_timer += t;
            }

            layer.strats = None;
            layers[[na, nb, 0]] = Some(layer);

            for nt in 1..3 {
                println!("--------------------------------");
                println!("na: {na:2}, nb: {nb:2}, nt: {nt}");

                let mut layer: Layer<6, false> =
                    layers[[na, nb, nt]].take().unwrap();

                if layer.is_done() {
                    println!("Already done!");
                } else {
                    let mut prev_layer =
                        layers[[na, nb, nt - 1]].take().unwrap();

                    let timer = Instant::now();

                    prev_layer.load_scores();

                    let t = timer.elapsed();
                    println!("Loading took {t:.2?}");
                    load_timer += t;

                    let timer = Instant::now();

                    let (scores, strats) = solve_layer_type2_6dice(
                        na,
                        nb,
                        &prev_layer.scores.unwrap(),
                    );

                    let t = timer.elapsed();
                    println!("Solving took {t:.2?}");
                    compute_timer += t;

                    prev_layer.scores = None;

                    layers[[na, nb, nt - 1]] = Some(prev_layer);

                    layer.scores = Some(scores);
                    layer.strats = Some(strats);

                    let timer = Instant::now();

                    layer.save_scores();
                    layer.save_strats();

                    let t = timer.elapsed();
                    println!("Saving  took {t:.2?}");
                    save_timer += t;
                }

                layer.strats = None;
                layers[[na, nb, nt]] = Some(layer);
            }
        }
    }

    println!("\n\nTotal   time: {:.2?}", global_timer.elapsed());
    println!("Compute time: {compute_timer:.2?}");
    println!("Loading time: {load_timer:.2?}");
    println!("Saving  time: {save_timer:.2?}");
}
