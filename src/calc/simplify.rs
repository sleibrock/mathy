// src/calc/simplify.rs

use crate::calc::expr::*;
use crate::calc::expr::Expr::*;


pub fn simplify(e: Expr) -> Expr {
    match e {
        Neg(ref i) => {
            neg(simplify(unpack(i)))
        },
        Add(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (Const(l), Const(r)) => Const(l + r),
                (Const(x), b) => {
                    if x.is_zero() {
                        simplify(b)
                    } else {
                        add(Const(x), simplify(b))
                    }
                },
                (a, Const(x)) => {
                    if x.is_zero() {
                        simplify(a)
                    } else {
                        add(Const(x), simplify(a))
                    }
                },
                (a, b)  => add( simplify(a), simplify(b)),
            }
        }, // end addition addition logic 

        Sub(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (Const(a), Const(b)) => Const(a - b),
                (Const(x), b) => {
                    if x.is_zero() {
                        neg(simplify(b))
                    } else {
                        sub(Const(x), simplify(b))
                    }
                },
                (a, Const(x)) => {
                    if x.is_zero() {
                        simplify(a)
                    } else {
                        sub(simplify(a), Const(x))
                    }
                },
                (a, Neg(i)) => {
                    let inner = unpack(&i);
                    add(simplify(a), simplify(inner)) 
                },
                (a, b) => sub(simplify(a), simplify(b)),
            }
        }, // end subtraction logic 

        Mul(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (Const(x), Const(y)) => Const(x * y),
                (Const(x), b) => {
                    if x.is_zero() {
                        zero()
                    } else if x.real_eq(1.0) {
                        simplify(b)
                    } else if x.real_eq(-1.0) {
                        neg(simplify(b))
                    } else {
                        mul(Const(x), simplify(b))
                    }
                },
                (a, Const(y)) => {
                    if y.is_zero() {
                        zero()
                    } else if y.real_eq(1.0) {
                        simplify(a)
                    } else if y.real_eq(-1.0) {
                        neg(simplify(a))
                    } else {
                       mul(Const(y), simplify(a)) 
                    }
                },
                (a, b) => mul(simplify(a), simplify(b)),
            }
        }, // end multiplication logic

        Div(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (numerator, Const(x)) => {
                    if x.real_eq(1.0) {
                        simplify(numerator)   
                    } else if x.real_eq(0.0) {
                        NaN
                    } else {
                        div(simplify(numerator), Const(x))
                    }
                },

                (a, b) => div(simplify(a), simplify(b)),
            }
        }, // end division logic

        _ => e,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const ZERO : Expr = Const(0.0);
    const ONE : Expr = Const(1.0);

    #[test]
    fn test_add_simplify() {
        let tests = vec![
            zero() + con(1.0),
            con(1.0) + zero(),
            zero() + zero(),
            con(1.0) + con(1.0),
        ];

        let answers = vec![
            ONE, ONE, ZERO, con(2.0), 
        ];

        for (l, r) in tests.iter().zip(answers.iter()) {
            assert_eq!(simplify(l.clone()), r.clone());
        }
    }

}


// end src/calc/simplify.rs
