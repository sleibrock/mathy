// src/calc/evaluator.rs

use crate::calc::expr::*;
use crate::calc::expr::Expr::*;

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
    fn evaluate_test() {
        let expr = add(con(3.0), var('x'));
        let value = evaluate(expr, 'x', 7.0);
        assert_eq!(Const(10.0), value);
    }
}
// end evaluator.rs
