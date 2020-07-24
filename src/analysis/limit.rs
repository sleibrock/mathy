// src/analysis/limit.rs

use crate::expr::prelude::*;

// Run a limit from the left side (negative)
pub fn limit_left(e: Expr, sym: char, towards: Number, iters: usize) {
    for i in (0..iters).into_iter().rev() {
	let x = towards - (real(0.1) / real(10.0).powf(i as f64));
	let result = evaluate(e.clone(), sym, x);
	println!("f({}) = {}", x.to_string(), result.to_string());
    }
}

// Run a limit from the right side (positive)
pub fn limit_right(e: Expr, sym: char, towards: Number, iters: usize) {
    for i in 0..iters {
	let x = towards + (real(0.1) / real(10.0).powf(i as f64));
	let result = evaluate(e.clone(), sym, x);
	println!("f({}) = {}", x.to_string(), result.to_string());
    }
}


// Run a limit from both sides
pub fn limit(e: Expr, sym: char, towards: Number, iters: usize) {
    limit_left(e.clone(), sym, towards, iters);
    println!("f({}) = {}", towards.to_string(),
	     evaluate(e.clone(), sym, towards).to_string());
    limit_right(e.clone(), sym, towards, iters);
}



// end src/analysis/limit.rs
