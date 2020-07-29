// src/bin/newtons_sqrt.rs

/// Attempt to guess the square root of a number.

use mathy::expr::prelude::*;
use mathy::analysis::newton::*;

fn newtons_sqrt(x: f64) {
    println!("Attempting to find sqrt of {}", x);
    println!("Initial guess: {}", x/10.0);

    // we are attempting to solve x^2 = a
    // so we rewrite it as f(x) = x^2 - a
    // then we use newton's method to find the root of the function
    let f = powf(var('x'), 2.0) - con(x);
    newton(f, 'x', x / 10.0, 10);
}

fn main() {
    newtons_sqrt(500.0);
}

// end src/bin/newtons_sqrt.rs
