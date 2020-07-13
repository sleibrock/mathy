// src/calc/expr.rs

use std::ops::{Add,Sub,Mul,Div,Neg};

pub type E = Box<Expr>;


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    NaN,
    Const(f64),
    Var(char),
    Neg(E),
    Add(E, E),
    Sub(E, E),
    Mul(E, E),
    Div(E, E),
    Pow(E, E),
    Sin(E),
    Cos(E),
    Tan(E),
    Ln(E),
    Exp(E),
}

use self::Expr::*;

impl Expr {
    pub fn is_zero(&self) -> bool {
        match self {
            Const(f) => *f == 0.0,
            _ => false,
        }
    }

    pub fn has_var(&self, s: char) -> bool {
        match self {
            Var(x)     => { s == *x },
            Neg(ref e) => { e.has_var(s) },
            Exp(ref e) => { e.has_var(s) },
            Ln(ref e)  => { e.has_var(s) },
            Sin(ref e) => { e.has_var(s) },
            Cos(ref e) => { e.has_var(s) },
            Tan(ref e) => { e.has_var(s) },
            Add(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Sub(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Mul(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Div(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            Pow(ref l, ref r) => { l.has_var(s) || r.has_var(s) },
            _ => false,
        }
    }
}

// Boxing tools to size out recursive structures
pub fn pack(e: Expr) -> E { Box::new(e) }
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

pub fn nan()        -> Expr { NaN }
pub fn con(v: f64)  -> Expr { Const(v) }
pub fn var(c: char) -> Expr { Var(c) }
pub fn neg(e: Expr) -> Expr { mul(con(-1.0), e) }
pub fn varf(v: f64, c: char) -> Expr { mul(con(v),  var(c)) }
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
pub fn tan(e: Expr)    -> Expr { Tan(pack(e)) }
pub fn ln(e: Expr)     -> Expr { Ln(pack(e)) }
pub fn log(base: f64, e: Expr) -> Expr {
    Div(pack(ln(e)), pack(ln(Const(base))))
}

// execute a one-var evaluation on an expression tree
pub fn evaluate(e: Expr, sym: char, value: f64) -> Expr {
    match e {
        NaN => NaN,
        Const(v) => Const(v),
        Var(x) => if x == sym { Const(value) } else { Var(x) },
		
		Neg(ref v) => {
			let inner = evaluate(unpack(v), sym, value);

			match inner {
				Const(x) => Const(-x),
				a => Neg(pack(a)),
			}
		},

        Add(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, value);
            let right = evaluate(unpack(r), sym, value);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv + rv),
                (Const(lv), Var(c)) => {
                    if c == sym {
                        Const(lv + value)
                    } else {
                        Add(pack(Const(lv)), pack(Var(c)))
                    }
                }
                (Var(c), Const(rv)) => {
                    if c == sym {
                        Const(rv + value)
                    } else {
                        Add(pack(Const(rv)), pack(Var(c)))
                    }
                }
                (a,b) => Add(pack(a), pack(b)),
            }
        },

        Sub(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, value);
            let right = evaluate(unpack(r), sym, value);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv - rv),
                (Const(lv), Var(c)) => {
                    if c == sym {
                        Const(lv - value)
                    } else {
                        Sub(pack(Const(lv)), pack(Var(c)))
                    }
                }
                (Var(c), Const(rv)) => {
                    if c == sym {
                        Const(rv - value)
                    } else {
                        Sub(pack(Const(rv)), pack(Var(c)))
                    }
                }
                (a,b) => Sub(pack(a), pack(b)),
            }
        },

        Mul(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, value);
            let right = evaluate(unpack(r), sym, value);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv * rv),
                (Const(lv), Var(c)) => {
                    if c == sym {
                        Const(lv * value)
                    } else {
                        Mul(pack(Const(lv)), pack(Var(c)))
                    }
                }
                (Var(c), Const(rv)) => {
                    if c == sym {
                        Const(rv * value)
                    } else {
                        Mul(pack(Const(rv)), pack(Var(c)))
                    }
                }
                (a,b) => Mul(pack(a), pack(b)),
            }
        },

        Div(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, value);
            let right = evaluate(unpack(r), sym, value);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv / rv),
                (Const(lv), Var(c)) => {
                    if c == sym {
                        Const(lv / value)
                    } else {
                        Div(pack(Const(lv)), pack(Var(c)))
                    }
                }
                (Var(c), Const(rv)) => {
                    if c == sym {
                        Const(rv / value)
                    } else {
                        Div(pack(Const(rv)), pack(Var(c)))
                    }
                }
                (a,b) => Div(pack(a), pack(b)),
            }
        },

        Sin(ref v) => {
            let inner = evaluate(unpack(v), sym, value);

            match inner {
                Const(x) => Const(x.sin()),
                Var(c) => {
                    if c == sym {
                        Const(value.sin())
                    } else { 
                        Sin(pack(var(c))) 
                    }
                },
                a => Sin(pack(a)),
            }
        }

        Cos(ref v) => {
            let inner = evaluate(unpack(v), sym, value);

            match inner {
                Const(x) => Const(x.cos()),
                Var(c) => {
                    if c == sym {
                        Const(value.cos())
                    } else {
                        Cos(pack(var(c)))
                    }
                },
                a => Cos(pack(a)),
            }
        }

        Tan(ref v) => {
            let inner = evaluate(unpack(v), sym, value);

            match inner {
                Const(x) => Const(x.tan()),
                Var(c) => {
                    if c == sym {
                        Const(value.tan())
                    } else {
                        Tan(pack(var(c)))
                    }
                },
                a => Tan(pack(a)),
            }
        }

		Pow(ref l, ref r) => {
			let left = evaluate(unpack(l), sym, value);
			let right = evaluate(unpack(r), sym, value);		

			match (left, right) {
				(Const(x), Const(y)) => Const(x.powf(y)),
				(Var(c), Const(y)) => {
					if c == sym {
						Const(value.powf(y))
					} else {
						Pow(pack(var(c)), pack(con(y)))
					}
				},
				(Const(x), Var(c)) => {
					if c == sym {
						Const(x.powf(value))
					} else {
						Pow(pack(con(x)), pack(var(c)))
					}
				},
				(a,b) => Pow(pack(a), pack(b)),
			}
		}

		Exp(ref v) => {
			let inner = evaluate(unpack(v), sym, value);

			match inner {
				Const(x) => Const(x.exp()),
				Var(c) => {
					if c == sym {
						Const(value.exp())
					} else {
						Exp(pack(var(c)))
					}
				},
				a => Exp(pack(a))
			}
		},

		Ln(ref v) => {
			let inner = evaluate(unpack(v), sym, value);

			match inner {
				Const(x) => Const(x.ln()),
				Var(c) => {
					if sym == c {
						Const(value.ln())
					} else {
						Ln(pack(var(c)))
					} 
				},
				a => Ln(pack(a)),
			}
		},
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_var_test() {
        let expr  = varf(3.0, 'x');
        let expr2 = varf(3.0, 'y');
        let expr3 = con(3.0);
        assert_eq!(true,  expr.has_var('x')); 
        assert_eq!(false, expr2.has_var('x'));
        assert_eq!(false, expr3.has_var('x'));
    }

    #[test]
    fn evaluate_test() {
        let expr = add(con(3.0), var('x'));
        let value = evaluate(expr, 'x', 7.0);
        assert_eq!(Const(10.0), value);
    }
}

// end src/calc/expr.rs

