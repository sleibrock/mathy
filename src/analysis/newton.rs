// src/analysis/newton.rs
// an implementation of Newton's method

use crate::calc::prelude::*;

pub fn newton(f: Expr, sym: char, guess: f64, iters: usize) {

    let f1 = simplify(derive(f.clone(), sym));
    let e = var(sym) - div(f.clone(), f1.clone());
    println!("Newton function: {}", e.to_string());

    let mut xp : Number = real(guess);
    for x in 0..iters {
	let x1 = evaluate(e.clone(), sym, xp);
	println!("cycle {}: {}", x, x1.to_string());
	xp = x1.extract(); 
    }

}


// end src/analysis/newton.rs



