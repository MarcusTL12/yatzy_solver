#![feature(split_array, generic_const_exprs)]
#![allow(incomplete_features)]

use std::slice::from_raw_parts_mut;

use simulation::{
    simulate_n_5, simulate_n_5_full, simulate_n_5x, simulate_n_6,
    simulate_n_6_full, simulate_n_5x_full, simulate_n_6x, simulate_n_6x_full,
};

pub mod dice_distributions;
pub mod dice_throw;
pub mod guide;
pub mod level_ordering;
pub mod macrosolver;
pub mod simulation;
pub mod solvers;
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

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_5_full(x: *mut [u32; 15], n: usize) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_5_full(x);
}

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_6(x: *mut u32, n: usize) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_6(x);
}

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_6_full(x: *mut [u32; 20], n: usize) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_6_full(x);
}

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_5x(x: *mut u32, n: usize) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_5x(x);
}

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_5x_full(
    x: *mut [u32; 15],
    n: usize,
) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_5x_full(x);
}

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_6x(x: *mut u32, n: usize) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_6x(x);
}

/// # Safety
///
/// n needs to not be larger than the writable memory
#[no_mangle]
pub unsafe extern "C" fn extern_simulate_n_6x_full(
    x: *mut [u32; 20],
    n: usize,
) {
    let x = from_raw_parts_mut(x, n);

    simulate_n_6x_full(x);
}
