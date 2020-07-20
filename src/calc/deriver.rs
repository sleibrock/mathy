// src/calc/deriver.rs

use crate::calc::expr::*;
use crate::calc::expr::Expr::*;

pub fn derive(e: Expr, sym: char) -> Expr {
    match e {
        NaN => NaN,
        Const(_) => Const(0.0),
        Var(s) => {
            if s == sym { Const(1.0) } else { Const(0.0) }
        }

        Add(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (left, right) => add(derive(left, sym), derive(right, sym)),
            }
        },
        _ => NaN,
    }

}


// end src/calc/deriver.rs
