use std::ops;
/* Programming Drill 1.1.1: Write a program that takes two complex numbers and returns their sum and product. */
#[derive(Debug)]
pub struct Complex {
    real : f64,
    imaginary : f64,
}

impl Complex {
    pub fn new(r : f64, i : f64) -> Complex {
        Complex {
            real: r,
            imaginary: i
        }
    }
}

impl ops::Add<&Complex> for &Complex {
    type Output = Complex;

    fn add(self, rhs : &Complex) -> Complex {
        let rsum = self.real + rhs.real;
        let isum = self.imaginary + rhs.imaginary;

        Complex {
            real : rsum,
            imaginary : isum,
        }
    }
}

impl ops::Mul<&Complex> for &Complex {
    type Output = Complex;

    fn mul(self, rhs : &Complex) -> Complex {
        let r = (self.real * rhs.real) + (self.imaginary * rhs.imaginary * -1.0);
        let i = (self.real * rhs.imaginary) + (self.imaginary * rhs.real);

        Complex {
            real: r,
            imaginary: i
        }
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
