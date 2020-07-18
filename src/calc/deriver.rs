// src/calc/deriver.rs

use crate::calc::expr::*;
use crate::calc::expr::Expr::*;

pub fn derive(e: Expr) -> Expr {
    match e {
        NaN => NaN,
        Const(_) => Const(0.0),
        Var(_) => Const(1.0),
        _ => NaN,
    }

}


// end src/calc/deriver.rs
