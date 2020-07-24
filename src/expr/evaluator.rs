// src/calc/evaluator.rs

use crate::expr::expr::*;
use crate::expr::expr::Expr::*;
use crate::number::number::*;
//use crate::number::number::Number::*;

// execute a one-var evaluation on an expression tree
pub fn evaluate(e: Expr, sym: char, v: Number) -> Expr {
    match e {
        Const(c) => Const(c),
        Var(x) if x == sym => Const(v), 
		
	Neg(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    match inner {
		Const(x) => Const(-x),
		a => neg(a),
	    }
	},

        Add(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv + rv),
                (a,b) => add(a, b),
            }
        },

        Sub(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv - rv),
                (a,b) => sub(a, b),
            }
        },

        Mul(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv * rv),
                (a,b) => mul(a, b),
            }
        },

        Div(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv / rv), 
                (a,b) => div(a, b),
            }
        },

        Sin(ref i) => {
            let inner = evaluate(unpack(i), sym, v);

            match inner {
                Const(x) => Const(x.sin()),
                a => sin(a),
            }
        },

        Cos(ref i) => {
            let inner = evaluate(unpack(i), sym, v);

            match inner {
                Const(x) => Const(x.cos()),
                a => cos(a),
            }
        },

	Pow(ref l, ref r) => {
	    let left = evaluate(unpack(l), sym, v);
	    let right = evaluate(unpack(r), sym, v);		
	    
	    match (left, right) {
		(Const(x), Const(y)) => Const(x.pow(y)),
		(a,b) => Pow(pack(a), pack(b)),
	    }
	},
	
	Exp(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    
	    match inner {
		Const(x) => Const(x.exp()),
		a => exp(a)
	    }
	},
	
	Ln(ref i) => {
	    let inner = evaluate(unpack(i), sym, v);
	    
	    match inner {
		Const(x) => Const(x.ln()),
		a => ln(a),
	    }
	},

	f => f,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluate_test() {
        let expr = add(con(3.0), var('x'));
        let value = evaluate(expr, 'x', real(7.0));
        assert_eq!(con(10.0), value);
    }

    #[test]
    fn square_test() {
	let f = powf(var('x'), 2.0);
	let v = evaluate(f, 'x', real(7.0));
	assert_eq!(con(49.0), v);
    }

    #[test]
    fn test_complex_sqrt() {
	let expr = sqrt(var('x'));
	let value = evaluate(expr, 'x', real(-1.0));
	assert_eq!(value, Const(complex(0.0, 1.0)));

	let expr2 = sqrt(var('x'));
	let value2 = evaluate(expr2, 'x', real(-3.0));
	assert_eq!(value2, Const(complex(0.0, 1.7320508075688772)));

	let expr3 = sqrt(var('x'));
	let value3 = evaluate(expr3, 'x', real(-4.0));
	assert_eq!(value3, Const(complex(0.0, 2.0)));
    }
}

// end evaluator.rs
