// src/calc/evaluator.rs

use crate::calc::expr::*;
use crate::calc::expr::Expr::*;
use crate::number::number::*;
//use crate::number::number::Number::*;

// execute a one-var evaluation on an expression tree
pub fn evaluate(e: Expr, sym: char, v: f64) -> Expr {
    let value = real(v);
    match e {
        NaN => NaN,
        Const(c) => Const(c),
        Var(x) => {
            if x == sym {
                con(v)
            } else {
                Var(x)
            }
        },
		
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
                (Const(lv), Var(c)) => {
                    if c == sym {
                        Const(lv + value)
                    } else {
                        add(Const(lv), var(c))
                    }
                }
                (Var(c), Const(rv)) => {
                    if c == sym {
                        Const(rv + value)
                    } else {
                        add(Const(rv), var(c))
                    }
                }
                (a,b) => add(a, b),
            }
        },

        Sub(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv - rv),
                (Const(lv), Var(c)) => {
                    if c == sym {
                        Const(lv - value)
                    } else {
                        sub(Const(lv), var(c))
                    }
                }
                (Var(c), Const(rv)) => {
                    if c == sym {
                        Const(rv - value)
                    } else {
                        sub(Const(rv), var(c))
                    }
                }
                (a,b) => sub(a, b),
            }
        },

        Mul(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);

            match (left, right) {
                (Const(lv), Const(rv)) => Const(lv * rv),
                (Const(lv), Var(c)) => {
                    if c == sym {
                        Const(lv * value)
                    } else {
                        mul(Const(lv), var(c))
                    }
                }
                (Var(c), Const(rv)) => {
                    if c == sym {
                        Const(rv * value)
                    } else {
                        mul(Const(rv), var(c))
                    }
                }
                (a,b) => mul(a, b),
            }
        },

        Div(ref l, ref r) => {
            let left = evaluate(unpack(l), sym, v);
            let right = evaluate(unpack(r), sym, v);

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
                (a,b) => div(a, b),
            }
        },

        Sin(ref i) => {
            let inner = evaluate(unpack(i), sym, v);

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

        Cos(ref i) => {
            let inner = evaluate(unpack(i), sym, v);

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

		Pow(ref l, ref r) => {
			let left = evaluate(unpack(l), sym, v);
			let right = evaluate(unpack(r), sym, v);		

			match (left, right) {
				(Const(x), Const(y)) => Const(x.pow(y)),
				(Var(c), Const(y)) => {
					if c == sym {
						Const(value.pow(y))
					} else {
						pow(var(c), Const(y))
					}
				},
				(Const(x), Var(c)) => {
					if c == sym {
						Const(x.pow(value))
					} else {
						pow(Const(x), var(c))
					}
				},
				(a,b) => Pow(pack(a), pack(b)),
			}
		}

		Exp(ref i) => {
			let inner = evaluate(unpack(i), sym, v);

			match inner {
				Const(x) => Const(x.exp()),
				Var(c) => {
					if c == sym {
						Const(value.exp())
					} else {
						exp(var(c))
					}
				},
				a => exp(a)
			}
		},

		Ln(ref i) => {
			let inner = evaluate(unpack(i), sym, v);

			match inner {
				Const(x) => Const(x.ln()),
				Var(c) => {
					if sym == c {
						Const(value.ln())
					} else {
						ln(var(c))
					} 
				},
				a => ln(a),
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
        assert_eq!(con(10.0), value);
    }
}
// end evaluator.rs
