// src/analysis/limit.rs

use crate::expr::prelude::*;

pub fn limit_left(e: Expr, towards: f64, iters: usize) {}
pub fn limit_right(e: Expr, towards: f64, iters: usize) {}
pub fn limit(e: Expr, sym: char, towards: f64, iters: usize) {

    let xp = towards;
    let delta = 0.1;
    for i in 0..iters {
	let x = xp + (delta / (10.0_f64).powf(i as f64));

	let result = evaluate(e.clone(), sym, x);
	println!("f({}) = {}", x, result.to_string());
    }

    println!("f(0) = {}", evaluate(e.clone(), sym, towards).to_string());

    for i in (0..iters).into_iter().rev() {
	let x = xp - (delta / (10.0_f64).powf(i as f64));

	let result = evaluate(e.clone(), sym, x);
	println!("f({}) = {}", x, result.to_string());
    }
}



// end src/analysis/limit.rs
