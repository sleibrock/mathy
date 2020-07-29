// src/calc/deriver.rs

use crate::expr::expr::*;
use crate::expr::expr::Expr::*;

/// Execute a one-variable differentiaton.
/// Hold other variables constant at zero.
pub fn derive(e: Expr, sym: char) -> Expr {
    match e {
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

	Pow(ref l, ref r) => {
	    let left = unpack(l);
	    let right = unpack(r);

	    match (left, right) {
		// var raised to a power
		(Var(c), Const(n)) if c == sym => {
		    if n.real_eq(1.0) {
			con(0.0)
		    } else if n.real_eq(2.0) {
			varf(sym, 2.0)
		    } else {
			Const(n) * Pow(pack(Var(sym)), pack(con(n.real()-1.0)))
		    }
		},
		// function raised to a power
		(f, Const(n)) => {
		    let fp = derive(f.clone(), sym);
		    let np = n.clone().real() - 1.0;

		    mul(mul(Const(n), pow(f, con(np))), fp)
		},
		(f, g) => {
		    mul(f, g)
		}

	    }
	},

	Exp(ref i) => {
	    let inner = unpack(i);

	    match inner {
		Const(_) => con(0.0), 
		Var(c) if c == sym => exp(var(sym)),
		f => {
		    let fp = derive(f.clone(), sym);
		    mul(fp, exp(f.clone()))
		}
	    }
	}

        _ => con(0.0),
    }

}



#[cfg(test)]
mod test {

    use super::*;
    use crate::expr::simplify::*;

    #[test]
    fn derive_square_test() {
	let square = powf(var('x'), 2.0);
	let deriv = con(2.0) * var('x');
	let square_d = simplify(derive(square, 'x'));

	assert_eq!(deriv, square_d);
    }

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
	let simple_deriv = simplify(derivative);

	let expected = con(1.0) / square(cos(var('x')));
	assert_eq!(simple_deriv, expected, "Uhoh division rule messed up?");
    }

    #[test]
    fn derive_exp_funcs() {
	// Will require more tests but this is basic enough for now
	let f1 = exp(powf(var('x'), 2.0));
	let f2 = varf('x', 2.0) * exp(powf(var('x'), 2.0));
	let f1d = derive(f1, 'x');

	assert_eq!(f1d, f2);
    }
}


// end src/calc/deriver.rs
