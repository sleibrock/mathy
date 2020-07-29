// src/calc/evaluator.rs

use crate::expr::expr::*;
use crate::expr::expr::Expr::*;
use crate::number::number::*;
//use crate::number::number::Number::*;

/// Execute an evaluation against a given `Expr` tree.
/// Takes three arguments; an `Expr` tree, a symbol char,
/// and a `Number` type. 
///
/// ```
/// use mathy::expr::prelude::*;
/// let f = powf(var('x'), 2.0);
/// let y = evaluate(f, 'x', real(10.0));
/// assert_eq!(y, real(100.0));
/// ```
pub fn evaluate(e: Expr, sym: char, v: Number) -> Number {
    match e {
        Const(c) => c,
        Var(x) if x == sym => v, 
		
	Neg(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    -inner
	},

        Add(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);
	    left + right
        },

        Sub(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);
	    left - right
        },

        Mul(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);
	    left * right
        },

        Div(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);
	    left / right
        },

        Sin(ref i) => {
            let inner = evaluate(unpack(i), sym, v);
	    inner.sin()
        },

        Cos(ref i) => {
            let inner = evaluate(unpack(i), sym, v);
	    inner.cos()
        },

	Sinh(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.sinh()
	},

	Cosh(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.cosh()
	},


	Asin(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.asin()
	},


	Acos(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.acos()
	},


	Asinh(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.asinh()
	},


	Acosh(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.acosh()
	},

	Pow(ref l, ref r) => {
	    let left = evaluate(unpack(l), sym, v);
	    let right = evaluate(unpack(r), sym, v);		
	    left.pow(right)
	}
	
	Exp(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.exp()
	},
	
	Ln(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.ln()
	},

	Factorial(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    inner.factorial()
	},

	/*
	Gamma(ref i) => {

	},
	*/

	// Leftover, can't evaluate?
	_ => nan(), 
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluate_test() {
        let expr = add(con(3.0), var('x'));
        let value = evaluate(expr, 'x', real(7.0));
        assert_eq!(real(10.0), value);
    }

    #[test]
    fn square_test() {
	let f = powf(var('x'), 2.0);
	let v = evaluate(f, 'x', real(7.0));
	assert_eq!(real(49.0), v);
    }

    #[test]
    fn test_complex_sqrt() {
	let expr = sqrt(var('x'));
	let value = evaluate(expr, 'x', real(-1.0));
	assert_eq!(value, complex(0.0, 1.0));

	let expr2 = sqrt(var('x'));
	let value2 = evaluate(expr2, 'x', real(-3.0));
	assert_eq!(value2, complex(0.0, 1.7320508075688772));

	let expr3 = sqrt(var('x'));
	let value3 = evaluate(expr3, 'x', real(-4.0));
	assert_eq!(value3, complex(0.0, 2.0));
    }
}

// end evaluator.rs
