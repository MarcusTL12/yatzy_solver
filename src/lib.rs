#![feature(array_zip, split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use std::slice::from_raw_parts_mut;

use simulation::simulate_n_5;

pub mod dice_distributions;
pub mod dice_throw;
pub mod guide;
pub mod level_ordering;
pub mod macrosolver;
pub mod simulation;
pub mod solver;
pub mod util;
pub mod yatzy;

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_5(x: *mut u32, n: usize) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_5(x);
}
