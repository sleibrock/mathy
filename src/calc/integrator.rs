// src/calc/integrator.rs

use crate::expr::expr::*;
use crate::expr::expr::Expr::*;

// integrate with respect to a single variable
pub fn integrate(e: Expr, sym: char) -> Expr {
    match e {
        NaN => NaN,
        Const(c) => add(mul(Const(c), Var(sym)), var('C')),
        _ => NaN,
    }
}



// end src/calc/integrator.rs
