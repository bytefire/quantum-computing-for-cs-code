use std::{ops, f64::consts::PI};

macro_rules! round_2_dec_places {
    ($a:expr) => {
        ($a * 100.0_f64).round() / 100.0_f64
    };
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Complex {
    real : f64,
    imaginary : f64,
    is_cartesian : bool,
}

impl Complex {
    pub fn new(r : f64, i : f64) -> Complex {
        Complex {
            real: r,
            imaginary: i,
            is_cartesian: true,
        }
    }

    pub fn new_polar(rho : f64, theta : f64) -> Complex {
        Complex {
            real: rho,
            imaginary: theta,
            is_cartesian: false,
        }
    }

    pub fn to_polar(&self) -> Complex {
        if !self.is_cartesian {
            return self.clone();
        }

        let theta = (self.imaginary / self.real).atan();
        let rho = self.imaginary / theta.sin();

        Complex::new_polar(rho, theta)
    }

    pub fn to_cartesian(&self) -> Complex {
        if self.is_cartesian {
            return self.clone();
        }

        let r = self.imaginary.cos() * self.real;
        let i = self.imaginary.sin() * self.real;

        Complex::new(r , i)
    }

    pub fn modulus(&self) -> f64 {
        (self.real.powi(2) + self.imaginary.powi(2)).sqrt()
    }

    pub fn conjugate(&self) -> Complex {
        Complex::new(self.real, -1.0 * self.imaginary)
    }
}

impl ops::Add<&Complex> for &Complex {
    type Output = Complex;

    fn add(self, rhs : &Complex) -> Complex {
        let rsum = self.real + rhs.real;
        let isum = self.imaginary + rhs.imaginary;

        Complex::new(rsum, isum)
    }
}

impl ops::Sub<&Complex> for &Complex {
    type Output = Complex;

    fn sub(self, rhs: &Complex) -> Complex {
        let rdiff = self.real - rhs.real;
        let idiff = self.imaginary - rhs.imaginary;

        Complex::new(rdiff, idiff)
    }
}

impl ops::Mul<&Complex> for &Complex {
    type Output = Complex;

    fn mul(self, rhs : &Complex) -> Complex {
        let r = (self.real * rhs.real) + (self.imaginary * rhs.imaginary * -1.0);
        let i = (self.real * rhs.imaginary) + (self.imaginary * rhs.real);

        Complex::new(r, i)
    }
}

impl ops::Div<&Complex> for &Complex {
    type Output = Complex;

    fn div(self, rhs: &Complex) -> Complex {
        let denominator = rhs.real.powi(2) + rhs.imaginary.powi(2);
        let rquotient = ((self.real * rhs.real) + (self.imaginary * rhs.imaginary)) / denominator;
        let iquotient = ((rhs.real * self.imaginary) - (self.real * rhs.imaginary)) / denominator;

        Complex::new(rquotient, iquotient)
    }
}

#[test]
fn test_add_and_multiply() {
    // As a unit test this shouldn't really combine add and multiply
    // so we should have separate tests for each. However, we should
    // keep this too as this chects that complex struct is indeed passed
    // as reference to + first and then *. By value wouldn't work and this
    // test tests that.
    let c1 = Complex::new(1.0, -2.0);
    let c2 = Complex::new(3.0, 5.0);
    let csum = &c1 + &c2;
    let cprod = &c1 * &c2;

    assert_eq!(csum.real, 4.0);
    assert_eq!(csum.imaginary, 3.0);
    assert_eq!(cprod.real, 13.0);
    assert_eq!(cprod.imaginary, -1.0);
}

#[test]
fn test_subtract() {
    let c1 = Complex::new(1.0, -2.0);
    let c2 = Complex::new(3.0, 5.0);

    let cdiff = &c1 - &c2;

    assert_eq!(cdiff.real, -2.0);
    assert_eq!(cdiff.imaginary, -7.0);
}

#[test]
fn test_divide() {
    let c1 = Complex::new(-2.0, 1.0);
    let c2 = Complex::new(1.0, 2.0);

    let cdiff = &c1 / &c2;

    assert_eq!(cdiff.real, 0.0);
    assert_eq!(cdiff.imaginary, 1.0);
}

#[test]
fn test_modulus() {
    let c = Complex::new(3.0, 4.0);

    assert_eq!(c.modulus(), 5.0);
}

#[test]
fn test_conjugate() {
    let c = Complex::new(3.0, 4.0);
    let conj = c.conjugate();

    assert_eq!(conj.real, c.real);
    assert_eq!(conj.imaginary, -1.0*c.imaginary);
}

#[test]
fn test_to_polar() {
    let cart = Complex::new(1.0, 1.0);
    let polar = cart.to_polar();

    assert_eq!(polar.real, 2.0_f64.sqrt());
    assert_eq!(polar.imaginary, PI/4.0_f64);
}

#[test]
fn test_to_cartesian() {
    let polar = Complex::new_polar(3.0, PI / 3.0_f64);
    let cart = polar.to_cartesian();

    assert_eq!(round_2_dec_places!(cart.real), 1.5);
    assert_eq!(cart.imaginary, (PI/3.0_f64).sin() * 3.0);
}
