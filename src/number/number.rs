// src/number/number.rs

use std::ops::{Add,Sub,Mul,Div,Neg};

use self::Number::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Real(f64),
    Complex(f64, f64),
}

// Defining our own number enumeration
impl Number {
    pub fn is_zero(&self) -> bool {
        match self {
            Real(x) => *x == 0.0,
            Complex(x, z) => *x == 0.0 && *z == 0.0,
        }
    }

    pub fn real(&self) -> f64 {
        match self {
            Real(x) => *x,
            Complex(r, _) => *r,
        }
    }

    pub fn imag(&self) -> f64 {
        match self {
            Complex(_, i) => *i,
            _ => 0.0,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
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

    pub fn real_eq(&self, v: f64) -> bool {
        match self {
            Real(x) => *x == v,
            _ => false,
        }
    }

    pub fn complex_eq(self, v1: f64, v2: f64) -> bool {
        match self {
            Complex(x, z) => x == v1 && z == v2,
            _ => false,
        }
    }
    
    // substitute number functions to cover both real/imag
    // TODO: make these work for complex numbers as well
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
	    Real(_) => self.sin() / self.cos(),
	    _ => Real(0.0),
	}
    }

    pub fn ln(&self) -> Number {
        match self {
            Real(x) => Real(x.ln()),
            _ => Real(0.0),
        }
    }

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

    pub fn conjugate(&self) -> Number {
	match self {
	    Real(x) => Real(*x),
	    Complex(z, i) => Complex(*z, -i),
	}
    }

    pub fn reciprocal(&self) -> Number {
	match self {
	    Real(x) => Real(1.0 / x),
	    Complex(x, y) => {
		let d = (x*x)+(y*y);
		if d <= 0.0 {
		    panic!("Complex division by zero!!")
		}
		Complex(x/d, -(y/d))
	    },
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
        }
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, other: Number) -> Number {
        match (self, other) {
            (Real(x), Real(y)) => {
                if y == 0.0 {
                    panic!("Division by zero!");
                }
                Real(x / y)
            },
	    (Real(u), Complex(x, y)) => {
		let d = x*x + y*y;
		if d == 0.0 {
		    panic!("Complex Division by zero!");
		}
		Complex((u*x)/d, -(u*y)/d)
	    },
	    (Complex(u, v), Real(x)) => {
		let d = x*x;
		if d == 0.0 {
		    panic!("(Complex) Division by zero!");
		}
		Complex((u*x)/d, -(v*x)/d)
	    },
	    (Complex(u, v), Complex(x, y)) => {
		let d = (x*x) + (y*y);
		if d == 0.0 {
		    panic!("Complex Division by zero!");
		}
		Complex((u*x+v*y)/d, (v*x-u*y)/d)
	    },
        }
    }
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        match self {
            Real(x) => Real(-x),
            _ => Real(0.0),
        }
    }
}


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
    }

}
