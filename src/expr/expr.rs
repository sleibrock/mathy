// src/calc/expr.rs

use std::ops::{Add,Sub,Mul,Div,Neg};
use crate::number::number::*;


pub type E = Box<Expr>;

/// Expr enum type representing all mathematical operations.
/// For recursive enumerations, any children nodes must be
/// boxed for Rust sizing purposes.
///
/// Distinct functions will be used as variants, but
/// functions which are composed of other functions like
/// tangent will be reduced to simpler forms currently.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(Number),
    Var(char),
    Neg(E),
    Add(E, E),
    Sub(E, E),
    Mul(E, E),
    Div(E, E),
    Pow(E, E),
    Sin(E),
    Cos(E),
    Sinh(E),
    Cosh(E),
    Asin(E),
    Acos(E),
    Asinh(E),
    Acosh(E),
    Ln(E),
    Exp(E),
    Factorial(E),
    Gamma(E),
}

use self::Expr::*;

/// All Expr methods used to check for certain conditions
/// and perform certain operations. Evaluation and
/// simplifying is not done here, but in other source files
/// due to the length of the code.
impl Expr {

    /// Check if the top level of the Expr tree
    /// is a constant value and return true.
    pub fn is_const(&self) -> bool {
        match self {
            Const(_) => true,
            _ => false,
        }
    }

    /// Check if the top level of the Expr tree
    /// is a variable and return true.
    pub fn is_var(&self) -> bool {
        match self {
            Var(_) => true,
            _ => false,
        }
    }

    /// Check if the top level of the Expr tree is
    /// an an operation and return true.
    pub fn is_op(&self) -> bool {
        match self {
            Const(_) => false,
            Var(_) => false,
            _ => true,
        }
    }

    /// Recursively dive into the Expr tree and see
    /// if it contains a Var(c) type where c == s.
    pub fn has_var(&self, s: char) -> bool {
        match self {
            Var(x)     => { s == *x },
            Neg(ref e) => { e.has_var(s) },
            Exp(ref e) => { e.has_var(s) },
            Ln(ref e)  => { e.has_var(s) },
            Sin(ref e) => { e.has_var(s) },
            Cos(ref e) => { e.has_var(s) },
            Add(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Sub(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Mul(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Div(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Pow(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
	    _ => false,
        }
    }

    /// Use this to substitute any variable with another recurisvely.
    pub fn substitute(&self, sym1: char, sym2: char) -> Expr {
        match self {
            Var(x) => {
                if *x == sym1 {
                    Var(sym2)
                } else {
                    Var(*x)
                }
            },
            Neg(ref i) => neg(i.substitute(sym1, sym2)),
            Exp(ref i) => exp(i.substitute(sym1, sym2)),
            Sin(ref i) => sin(i.substitute(sym1, sym2)),
            Cos(ref i) => cos(i.substitute(sym1, sym2)),
            Sinh(ref i) => sinh(i.substitute(sym1, sym2)),
            Cosh(ref i) => cosh(i.substitute(sym1, sym2)),
            Asin(ref i) => asin(i.substitute(sym1, sym2)),
            Acos(ref i) => acos(i.substitute(sym1, sym2)),
            Asinh(ref i) => asinh(i.substitute(sym1, sym2)),
            Acosh(ref i) => acosh(i.substitute(sym1, sym2)),
            Add(ref l, ref r) => add(l.substitute(sym1, sym2), r.substitute(sym1, sym2)),
            Sub(ref l, ref r) => sub(l.substitute(sym1, sym2), r.substitute(sym1, sym2)),
            Mul(ref l, ref r) => mul(l.substitute(sym1, sym2), r.substitute(sym1, sym2)),
            Div(ref l, ref r) => div(l.substitute(sym1, sym2), r.substitute(sym1, sym2)),
            Pow(ref l, ref r) => pow(l.substitute(sym1, sym2), r.substitute(sym1, sym2)),

	    // fallback in case not everything is mapped
            e => e.clone(),
        }
    }

    /// Map an Expr to a String type with recursive formatting.
    /// Use this if you want better formula visualization.
    ///
    /// ```
    /// use mathy::calc::prelude::*;
    /// let e1 = var('x') * con(2.0);
    /// println!("My equation: {}", e1.to_string());
    /// // use this for raw debugging
    /// println!("My Expr: {:?}", e1);
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            Const(c) => String::from(format!("{}", c.to_string())),
            Var(s)   => String::from(format!("{}", s)),
            Neg(ref i) => String::from(format!("-({})",   i.to_string())),
            Ln(ref i)  => String::from(format!("ln({})",  i.to_string())),
            Exp(ref i) => String::from(format!("e^({})",  i.to_string())),
            Sin(ref i) => String::from(format!("sin({})", i.to_string())),
            Cos(ref i) => String::from(format!("cos({})", i.to_string())),
	    Sinh(ref i) => String::from(format!("sinh({})", i.to_string())),
	    Cosh(ref i) => String::from(format!("cosh({})", i.to_string())),
	    Asin(ref i) => String::from(format!("asin({})", i.to_string())),
	    Acos(ref i) => String::from(format!("acos({})", i.to_string())),
	    Asinh(ref i) => String::from(format!("asinh({})", i.to_string())),
	    Acosh(ref i) => String::from(format!("acosh({})", i.to_string())),
            Add(ref l, ref r) => {
		let left = unpack(l);
		let right = unpack(r);
		match (left, right) {
		    (Const(a), Const(b)) => {
			String::from(format!("{} + {}", a.to_string(), b.to_string()))
		    },
		    (Const(a), b) => {
			String::from(format!("{} + ({})", a.to_string(), b.to_string()))
		    },
		    (a, Const(b)) => {
			String::from(format!("({}) + {}", a.to_string(), b.to_string()))
		    },
		    (Var(c), b) => {
			String::from(format!("{} + ({})", c, b.to_string()))
		    },
		    (a, b) => {
			String::from(format!("({}) + ({})", a.to_string(), b.to_string()))
		    }
		}
            },
            Sub(ref l, ref r) => {
		let left = unpack(l);
		let right = unpack(r);
		match (left, right) {
		    (Const(a), Const(b)) => {
			String::from(format!("{} - {}", a.to_string(), b.to_string()))
		    },
		    (Const(a), b) => {
			String::from(format!("{} - ({})", a.to_string(), b.to_string()))
		    },
		    (a, Const(b)) => {
			String::from(format!("({}) - {}", a.to_string(), b.to_string()))
		    },
		    (Var(c), b) => {
			String::from(format!("{} - ({})", c, b.to_string()))
		    },
		    (a, b) => {
			String::from(format!("({}) - ({})", a.to_string(), b.to_string()))
		    }
		}
            },
            Mul(ref l, ref r) => {
		let left = unpack(l);
		let right = unpack(r);
		match (left, right) {
		    (Const(a), Const(b)) => {
			String::from(format!("{} * {}", a.to_string(), b.to_string()))
		    },
		    (Const(a), Var(c)) => {
			String::from(format!("{}{}", a.to_string(), c))
		    },
		    (Var(c), Const(b)) => {
			String::from(format!("{}{}", b.to_string(), c))
		    },
		    (Const(a), b) => {
			String::from(format!("{} * ({})", a.to_string(), b.to_string()))
		    },
		    (a, Const(b)) => {
			String::from(format!("({}) * {}", a.to_string(), b.to_string()))
		    },
		    (a, b) => {
			String::from(format!("({}) * ({})", a.to_string(), b.to_string()))
		    }
		}
            },
            Div(ref l, ref r) => {
		let left = unpack(l);
		let right = unpack(r);
		match (left, right) {
		    (Const(a), Const(b)) => {
			String::from(format!("{} / {}", a.to_string(), b.to_string()))
		    },
		    (Const(a), b) => {
			String::from(format!("{} / {}", a.to_string(), b.to_string()))
		    },
		    (a, Const(b)) => {
			String::from(format!("({}) / {}", a.to_string(), b.to_string()))
		    },
		    (a, b) => {
			String::from(format!("({}) / ({})", a.to_string(), b.to_string()))
		    }
		}
            },
            Pow(ref l, ref r) => {
		let left = unpack(l);
		let right = unpack(r);
		match (left, right) {
		    (Const(a), Const(b)) => {
			String::from(format!("{}^{}", a.to_string(), b.to_string()))
		    },
		    (Const(a), b) => {
			String::from(format!("{}^({})", a.to_string(), b.to_string()))
		    },
		    (a, Const(b)) => {
			String::from(format!("{}^{}", a.to_string(), b.to_string()))
		    },
		    (a, b) => {
			String::from(format!("({})^({})", a.to_string(), b.to_string()))
		    }
		}
            },
	    Factorial(ref i) => String::from(format!("({})!", i.to_string())), 
	    Gamma(ref i) => String::from(format!("Γ({})", i.to_string())),
	}
    }

    /// A quick way of wrapping an Expr type into a Box.
    /// Use this to chain Expr calls to wrap into a box quickly
    /// if you don't want to use the shortcut `pack()` function.
    pub fn pack(&self) -> E {
        Box::new(self.clone())
    }
}

// Boxing / unboxing functions

/// pack() provides an easy way of turning an Expr item
/// into a Box<Expr> (or E) type. This is used almost everywhere
/// in internal shortcut functions and pattern matching to quickly
/// hop between Boxed and unboxed Expr items.
///
/// For most purposes you shouldn't need to worry about this yourself,
/// as the shortcut functions defined later will do the packing for you.
pub fn pack(e: Expr) -> E { Box::new(e) }

/// unpack() is a way of unpacking Exprs from boxes.
/// This is heavily used when unwrapping references in pattern matching
/// to quickly turn boxed items into Expr types.
pub fn unpack(e: &E) -> Expr { *(e.clone()) }


// Expr + Expr = Expr
impl Add for Expr {
    type Output = Expr;
    fn add(self, other: Expr) -> Expr {
	Add(pack(self), pack(other))
    }
}

// f64 + Expr = Expr
impl Add<Expr> for f64 {
    type Output = Expr;
    fn add(self, other: Expr) -> Expr {
	Add(pack(con(self)), pack(other))
    }
}

// Expr + f64 = Expr
impl Add<f64> for Expr {
    type Output = Expr;
    fn add(self, other: f64) -> Expr {
	Add(pack(con(other)), pack(self))
    }
} 

// Expr - Expr = Expr
impl Sub for Expr {
    type Output = Expr;
    fn sub(self, other: Expr) -> Expr {
	Sub(pack(self), pack(other))
    }
}

// f64 - Expr = Expr
impl Sub<Expr> for f64 {
    type Output = Expr;
    fn sub(self, other: Expr) -> Expr {
	Sub(pack(con(self)), pack(other))
    }
}

// Expr - f64 = Expr
impl Sub<f64> for Expr {
    type Output = Expr;
    fn sub(self, other: f64) -> Expr {
	Sub(pack(con(other)), pack(self))
    }
}

// Expr * Expr = Expr
impl Mul for Expr {
    type Output = Expr;
    fn mul(self, other: Expr) -> Expr {
	Mul(pack(self), pack(other))
    }
}

// f64 * Expr = Expr
impl Mul<Expr> for f64 {
    type Output = Expr;
    fn mul(self, other: Expr) -> Expr {
	Mul(pack(con(self)), pack(other))
    }
}

// Expr * f64 = Expr
impl Mul<f64> for Expr {
    type Output = Expr;
    fn mul(self, other: f64) -> Expr {
	Mul(pack(con(other)), pack(self))
    }
}

// Expr / Expr = Expr
impl Div for Expr {
    type Output = Expr;
    fn div(self, other: Expr) -> Expr {
	Div(pack(self), pack(other))
    }
}

// f64 / Expr = Expr
impl Div<Expr> for f64 {
    type Output = Expr;
    fn div(self, other: Expr) -> Expr {
	Div(pack(con(self)), pack(other))
    }
}

// Expr / f64 = Expr
impl Div<f64> for Expr {
    type Output = Expr;
    fn div(self, other: f64) -> Expr {
	Div(pack(self), pack(con(other)))
    }
}

// -Expr = Expr
impl Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
	Mul(pack(con(-1.0)), pack(self))
    }
}


// Shortcut functions for quick access
// These are quick functions to create Expr elements, using very
// minimal function names for easy writing. With these, expressions
// can be written as simply as
// let e1 = var('x') * powf(e(), 2.0) - con(5.0)

// con = constant value
// var = variable
// varf = variable that is multiplied by a number
// pow = power function to raise an Expr by an Expr
// powf = power function except raise Expr by a float

pub fn zero()       -> Expr { Const(real(0.0)) }
pub fn one()        -> Expr { Const(real(1.0)) }
pub fn two()        -> Expr { Const(real(2.0)) }
pub fn pi()         -> Expr { Const(real(3.14159265359)) }
pub fn e()          -> Expr { Const(real(2.71828182845)) }
pub fn con(v: f64)  -> Expr { Const(real(v)) }
pub fn var(c: char) -> Expr { Var(c) }
pub fn neg(e: Expr) -> Expr { Neg(pack(e)) }
pub fn exp(e: Expr) -> Expr { Exp(pack(e)) }
pub fn varf(c: char, v: f64) -> Expr { mul(con(v),  var(c)) }
pub fn add(l: Expr, r: Expr) -> Expr { Add(pack(l), pack(r)) }
pub fn sub(l: Expr, r: Expr) -> Expr { Sub(pack(l), pack(r)) }
pub fn mul(l: Expr, r: Expr) -> Expr { Mul(pack(l), pack(r)) }
pub fn div(l: Expr, r: Expr) -> Expr { Div(pack(l), pack(r)) }
pub fn pow(l: Expr, r: Expr) -> Expr { Pow(pack(l), pack(r)) }
pub fn powf(l: Expr, f: f64) -> Expr { Pow(pack(l), pack(con(f))) }
pub fn sqrt(e: Expr)   -> Expr { Pow(pack(e), pack(con(0.5))) }
pub fn square(e: Expr) -> Expr { powf(e, 2.0) }
pub fn cube(e: Expr)   -> Expr { powf(e, 3.0) }
pub fn sin(e: Expr)    -> Expr { Sin(pack(e)) }
pub fn cos(e: Expr)    -> Expr { Cos(pack(e)) }
pub fn sinh(e: Expr)   -> Expr { Sinh(pack(e)) }
pub fn cosh(e: Expr)   -> Expr { Cosh(pack(e)) }
pub fn asin(e: Expr)   -> Expr { Asin(pack(e)) }
pub fn acos(e: Expr)   -> Expr { Acos(pack(e)) }
pub fn asinh(e: Expr)  -> Expr { Asinh(pack(e)) }
pub fn acosh(e: Expr)  -> Expr { Acosh(pack(e)) }
pub fn tan(e: Expr)    -> Expr {
    let e2 = e.clone();
    div(sin(e), cos(e2))
}
pub fn ln(e: Expr)     -> Expr { Ln(pack(e)) }
pub fn log(base: f64, e: Expr) -> Expr {
    div(ln(e), ln(con(base)))
}

pub fn base2(e: Expr) -> Expr {
    div(ln(e), ln(con(2.0)))
}

pub fn base10(e: Expr) -> Expr {
    div(ln(e), ln(con(10.0)))
}

pub fn factorial(e: Expr) -> Expr {
    Factorial(pack(e))
}

pub fn gamma(e: Expr) -> Expr {
    Gamma(pack(e))
}

// unit tests and other such things
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_var_test() {
        let expr  = varf('x', 3.0);
        let expr2 = varf('y', 3.0);
        let expr3 = con(3.0);
        assert_eq!(true,  expr.has_var('x')); 
        assert_eq!(false, expr2.has_var('x'));
        assert_eq!(false, expr3.has_var('x'));
    }

    #[test]
    fn is_equal_test() {
        let f1 = add(con(2.0), con(2.0));
        let f2 = add(con(2.0), con(2.0));
        assert_eq!(f1, f2);

	let s1 = sin(var('x'));
	let s2 = sin(var('x'));
	assert_eq!(s1, s2);
    }
}

// end src/calc/expr.rs

