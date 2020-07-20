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
                    if x == 0.0 {
                        simplify(b)
                    } else {
                        add(con(x), simplify(b))
                    }
                },
                (a, Const(x)) => {
                    if x == 0.0 {
                        simplify(a)
                    } else {
                        add(con(x), simplify(a))
                    }
                },
                (a, b)  => add( simplify(a), simplify(b)),
            }
        }, // end addition addition logic 

        Sub(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (Const(a), Const(b)) => con(a - b),
                (Const(x), b) => {
                    if x == 0.0 {
                        neg(simplify(b))
                    } else {
                        sub(con(x), simplify(b))
                    }
                },
                (a, Const(x)) => {
                    if x == 0.0 {
                        simplify(a)
                    } else {
                        sub(simplify(a), con(x))
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
                (Const(x), Const(y)) => con(x * y),
                (Const(x), b) => {
                    if x == 0.0 {
                        zero()
                    } else if x == 1.0 {
                        simplify(b)
                    } else if x == -1.0 {
                        neg(simplify(b))
                    } else {
                        mul(con(x), simplify(b))
                    }
                },
                (a, Const(y)) => {
                    if y == 0.0 {
                        zero()
                    } else if y == 1.0 {
                        simplify(a)
                    } else if y == -1.0 {
                        neg(simplify(a))
                    } else {
                       mul(con(y), simplify(a)) 
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
                    if x == 1.0 {
                        simplify(numerator)   
                    } else if x == 0.0 {
                        NaN
                    } else {
                        div(simplify(numerator), con(x))
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
