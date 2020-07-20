// src/calc/deriver.rs

use crate::calc::expr::*;
use crate::number::number::*;
use crate::calc::expr::Expr::*;
use crate::number::number::Number::*;

pub fn derive(e: Expr, sym: char) -> Expr {
    match e {
        NaN => NaN,
        Const(_) => con(0.0),
        Var(s) => {
            if s == sym {
                con(1.0)
            } else {
                con(0.0)
            }
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
