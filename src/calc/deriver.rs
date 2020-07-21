// src/calc/deriver.rs

use crate::calc::expr::*;
use crate::calc::expr::Expr::*;
//use crate::number::number::*;
//use crate::number::number::Number::*;

/// Execute a one-variable differentiaton.
/// Hold other variables constant at zero.
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
        },

        Add(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);
            add(derive(left, sym), derive(right, sym))
        },

        Sub(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);
            sub(derive(left, sym), derive(right, sym))
        },

        Mul(ref l, ref r) => {
            let left = unpack(l);
            let right = unpack(r);

            match (left, right) {
                (Const(x), Var(s)) => {
                    if s == sym {
                        Const(x)
                    } else {
                        con(0.0)
                    }
                },
                (Var(s), Const(x)) => {
                    if s == sym {
                        Const(x)
                    } else {
                        con(0.0)
                    }
                },
                (a, b) => {
                    let a1 = a.clone();
                    let b1 = b.clone();
                    add(mul(a, derive(b, sym)), mul(b1, derive(a1, sym)))
                },
            } 
        },

        Div(ref l, ref r) => {
            let f = unpack(l);
            let g = unpack(r);
            let f2 = unpack(l);
            let g2 = unpack(r);
            let g3 = unpack(r);

            let fp = derive(f, sym);
            let gp = derive(g, sym);

            div(sub(mul(g2, fp), mul(gp, f2)), pow(g3, con(2.0)))
        },

        Sin(ref i) => {
            let inner = unpack(i);

            match inner {
                Var(c) => {
                    if c == sym {
                        cos(var(c))
                    } else {
                        con(0.0)
                    }
                },
                f => {
                    let f2 = f.clone();
                    mul(derive(f, sym), cos(f2))
                },
            }
        },

        Cos(ref i) => {
            let inner = unpack(i);

            match inner {
                Var(c) => {
                    if c == sym {
                        neg(sin(var(c)))
                    } else {
                        con(0.0)
                    }
                }
                f => {
                    let f2 = f.clone();
                    mul(neg(derive(f, sym)), sin(f2))
                },
            }
        },
        
        _ => NaN,
    }

}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn derive_tangent_test() {
        // a very easy way to check if our division rule is working.
        // let tan() convert into a div(sin,cos) expression, then
        // see if the division rule works accordingly.
        // it ends up being 1/cos^2(x) because of how trig functions
        // differentiate, and the euler identity states that:
        // sin^2(x) + cos^2(x) = 1
        // which is entirely the numerator because the division rule is:
        // f(x)/g(x) = f'g - g'f / g^2
        let f1 = tan(var('x')); // tan(x) => sin(x) / cos(x)
        let derivative = derive(f1, 'x');
    }
}


// end src/calc/deriver.rs
