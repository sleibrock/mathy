// src/number/number.rs

use std::ops::{Add,Sub,Mul,Div,Neg};

use self::Number::*;

/// The primary Number type to use throughout the project.
/// The Number holds three variants: NaN, Real and Complex.
/// The `Expr` type will use `Number` as a placeholder for
/// when equations need to be evaluated.
///
/// ```
/// use mathy::number::number::*;
/// let ten = real(10.0);
/// let hundred = ten * ten;
/// assert_eq!(hundred, real(100.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    NaN,
    Real(f64),
    Complex(f64, f64),
}

// Defining our own number enumeration
impl Number {
    /// Check if a number is NaN.
    pub fn is_nan(&self) -> bool {
	match self {
	    NaN => true,
	    _ => false,
	}
    }

    /// Check if a number is zero (works for both Real/Complex)
    pub fn is_zero(&self) -> bool {
        match self {
	    NaN => false,
            Real(x) => *x == 0.0,
            Complex(x, z) => *x == 0.0 && *z == 0.0,
        }
    }

    /// Check if a number is a whole number (no fractional
    /// component needed to express the number). This checks
    /// if all components have no remainder values of 0.0 by doing
    /// rounded delta checks. NaN will return false.
    pub fn is_whole(&self) -> bool {
	match self {
	    NaN => false,
	    Real(x) => x - (x.round()) == 0.0,
	    Complex(x, z) => x - (x.round()) == 0.0 && z - (z.round()) == 0.0,
	}
    }

    /// Access the raw f64 real component of a Number. NaN will return 0.0
    pub fn real(&self) -> f64 {
        match self {
	    NaN => 0.0,
            Real(x) => *x,
            Complex(r, _) => *r,
        }
    }

    /// Access the raw f64 imaginary component of a Number. NaN returns 0.0
    pub fn imag(&self) -> f64 {
        match self {
            Complex(_, i) => *i,
            _ => 0.0,
        }
    }

    /// Format a Number into a std::String.
    pub fn to_string(&self) -> String {
        match self {
	    NaN => "NaN".into(),
            Real(x) => format!("{}", x),
            Complex(x, z) => {
                if *z < 0.0 {
                    format!("{}{}j", x, z)
                } else {
                    format!("{}+{}j", x, z)
                }
            }
        }
    }

    /// Quick function to compare a Number to an f64
    pub fn real_eq(&self, v: f64) -> bool {
        match self {
            Real(x) => *x == v,
            _ => false,
        }
    }

    /// Another quick function to compare a Number to two f64 values
    pub fn complex_eq(self, v1: f64, v2: f64) -> bool {
        match self {
            Complex(x, z) => x == v1 && z == v2,
            _ => false,
        }
    }
    
    // Big TODO: map all these functions onwards to their complex variants // 

    // substitute number functions to cover both real/imag
    // complex:
    // sin(z) = sin(x+iy) = sin(x)cosh(y) + icos(x)sinh(y)
    pub fn sin(&self) -> Number { 
        match self {
            Real(x) => Real(x.sin()),
            _ => Real(0.0),
        }
    }
    
    // complex:
    // cos(z) = cos(x+iy) = cos(x)cosh(y) - isin(x)sinh(y)
    pub fn cos(&self) -> Number {
        match self {
            Real(x) => Real(x.cos()),
            _ => Real(0.0),
        }
    }

    pub fn tan(&self) -> Number {
	match self {
	    Real(x) => Real(x.tan()),
	    _ => Real(0.0),
	}
    }

    pub fn ln(&self) -> Number {
        match self {
            Real(x) => Real(x.ln()),
            _ => Real(0.0),
        }
    }

    /// Apply an exponentiation operation to a Number. This can
    /// result in imaginary numbers based on the power the Number is
    /// raised to.
    pub fn pow(&self, other: Number) -> Number {
        match (self, other) {
	    (Real(base), Real(power)) => {
		let log_test = power.log2();
		
		// check to see if we are raising to a negative power,
		// which will give us a complex number if the power
		// is of base log2, ie 1/2, 1/4, 1/8, 1/16, ... etc
		if *base < 0.0 && log_test < 0.0 && (log_test - log_test.round()) == 0.0 {
		    let new_base = base.abs();
		    Complex(0.0, new_base.powf(power))
		} else {
		    Real(base.powf(power))
		}
	    },
            _ => Real(0.0),
        }
    }

    /// Raise a Number to a power, except this takes an f64.
    /// Serves as a shortcut function to avoid having to manually
    /// wrap floats into Number variants.
    pub fn powf(&self, power: f64) -> Number {
        match self {
            Real(_) => self.pow(Real(power)), 
            _ => Real(0.0),
        }
    }

    pub fn exp(&self) -> Number {
        match self {
            Real(x) => Real(x.exp()),
            _ => Real(0.0),
        }
    }

    pub fn sinh(&self) -> Number {
	match self {
	    Real(x) => Real(x.sinh()),
	    _ => Real(0.0),
	}
    }

    pub fn cosh(&self) -> Number {
	match self {
	    Real(x) => Real(x.cosh()),
	    _ => Real(0.0),
	}
    }

    pub fn tanh(&self) -> Number {
	match self {
	    Real(x) => Real(x.tanh()),
	    _ => Real(0.0),
	}
    }

    pub fn asin(&self) -> Number {
	match self {
	    Real(x) => Real(x.asin()),
	    _ => Real(0.0),
	}
    }

    pub fn acos(&self) -> Number {
	match self {
	    Real(x) => Real(x.acos()),
	    _ => Real(0.0),
	}
    }

    pub fn atan(&self) -> Number {
	match self {
	    Real(x) => Real(x.atan()),
	    _ => Real(0.0),
	}
    }

    pub fn atan2(&self, other: Number) -> Number {
	match (self, other) {
	    (Real(x), Real(y)) => Real(x.atan2(y)),
	    _ => Real(0.0),
	}
    }

    pub fn asinh(&self) -> Number {
	match self {
	    Real(x) => Real(x.asinh()),
	    _ => Real(0.0),
	}
    }

    pub fn acosh(&self) -> Number {
	match self {
	    Real(x) => Real(x.acosh()),
	    _ => Real(0.0),
	}
    }

    pub fn atanh(&self) -> Number {
	match self {
	    Real(x) => Real(x.atanh()),
	    _ => Real(0.0),
	}
    }

    // Give a complex conjugate of a number
    // For real numbers this probably doesn't do anything
    pub fn conjugate(&self) -> Number {
	match self {
	    Real(x) => Real(*x),
	    Complex(z, i) => Complex(*z, -i),
	    _ => NaN,
	}
    }

    // Give a reciprocal for a given number
    pub fn reciprocal(&self) -> Number {
	match self {
	    Real(x) => Real(1.0 / x),
	    Complex(x, y) => {
		let d = (x*x)+(y*y);
		if d <= 0.0 {
		    return NaN;
		}
		Complex(x/d, -(y/d))
	    },
	    _ => NaN,
	}
    }

    // TODO: add checks for floating point/complex numbers
    // if given (2.1)! or a complex number z!, defer it
    // to the Gamma function which uses integral estimation
    pub fn factorial(&self) -> Number {
	match self {
	    Real(x) => {
		let upper = (x+1.0) as usize;
		if upper > 100 {
		    panic!("Factorial too large for data type")
		}
		let mut xs : u64 = 1;
		for i in 1 .. upper {
		    xs *= i as u64;
		}
		Real(xs as f64)
	    },
	    Complex(x, z) => complex(*x, *z).gamma(10),
	    _ => NaN,
	}
    }

    // TODO: implement gamma function with estimation
    pub fn gamma(&self, _iters: usize) -> Number {
	match self {
	    Complex(_x, _z) => {
		real(0.0)
	    },
	    _ => NaN,
	}
    }
}

// TODO: finish arithmetic for pretty much everything
impl Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        match (self, other) {
            (Real(x), Real(y)) => Real(x + y),
            (Real(x), Complex(z, i)) => Complex(x + z, i),
	    (Complex(z, i), Real(x)) => Complex(x + z, i),
	    (Complex(z1, i1), Complex(z2, i2)) => Complex(z1 + z2, i1 + i2),
	    _ => NaN,
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Number {
        match (self, other) {
            (Real(x), Real(y)) => Real(x - y),
            (Real(x), Complex(z, i)) => Complex(x - z, i),
	    (Complex(z, i), Real(x)) => Complex(x - z, i),
	    (Complex(z1, i1), Complex(z2, i2)) => Complex(z1 - z2, i1 - i2),
	    _ => NaN,
        }
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, other: Number) -> Number {
        match (self, other) {
            (Real(x), Real(y)) => Real(x * y),
	    (Real(x), Complex(u, v)) => Complex(x*u, x*v),
	    (Complex(x, y), Real(u)) => Complex(x*u, y*u),
	    (Complex(x, y), Complex(u, v)) => Complex(x*u-y*v, x*v+y*u),
	    _ => NaN,
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, other: Number) -> Number {
        match (self, other) {
            (Real(x), Real(y)) => {
                if y == 0.0 {
		    return NaN;
                }
                Real(x / y)
            },
	    (Real(u), Complex(x, y)) => {
		let d = x*x + y*y;
		if d == 0.0 {
		    return NaN;
		}
		Complex((u*x)/d, -(u*y)/d)
	    },
	    (Complex(u, v), Real(x)) => {
		let d = x*x;
		if d == 0.0 {
		    return NaN;
		}
		Complex((u*x)/d, -(v*x)/d)
	    },
	    (Complex(u, v), Complex(x, y)) => {
		let d = (x*x) + (y*y);
		if d == 0.0 {
		    return NaN;
		}
		Complex((u*x+v*y)/d, (v*x-u*y)/d)
	    },
	    _ => NaN,
        }
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        match self {
            Real(x) => Real(-x),
            Complex(z, i) => Complex(-z, -i),
	    _ => NaN,
        }
    }
}


// Shortcut functions for ease of use
pub fn nan() -> Number { NaN }
pub fn real(x: f64) -> Number { Real(x) }
pub fn imag(x: f64) -> Number { Complex(0.0, x) }
pub fn complex(x: f64, z: f64) -> Number {
    Complex(x, z)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_add_test() {
        let r1 = real(2.0);
        let r2 = real(3.0);
        let r3 = real(5.0);

	assert_eq!(r1+r2, r3);
    }

    #[test]
    fn complex_division_test() {
	// 3+4j / 2+5j = 3+7j
	let c1 = complex(3.0, 4.0);
	let c2 = complex(2.0, 2.0);
	let c3 = complex(1.75, 0.25);

	assert_eq!(c1/c2, c3);
    }

    #[test]
    fn factorial_test() {
	let c1 = real(5.0);
	let c2 = real(120.0);
	assert_eq!(c2, c1.factorial());
    }
}

// end src/number/number.rs
