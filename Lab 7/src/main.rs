use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[derive(Debug, Copy, Clone, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}
impl Complex {
    fn new<R, I>(v1: R, v2: I) -> Complex
    where
        f64: From<R>,
        f64: From<I>,
    {
        Complex {
            real: f64::from(v1),
            imag: f64::from(v2),
        }
    }
    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}
impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Complex {
            real: value as f64,
            imag: 0.0,
        }
    }
}
impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex {
            real: value,
            imag: 0.0,
        }
    }
}
impl<T> Add<T> for Complex
where
    T: Into<Complex>,
    Complex: std::convert::From<T>,
{
    type Output = Complex;
    fn add(self, value: T) -> Complex {
        let value_comp = Complex::from(value);
        let new_real = self.real + value_comp.real;
        let new_imag = self.imag + value_comp.imag;
        Complex::new(new_real, new_imag)
    }
}
impl<T> Sub<T> for Complex
where
    T: Into<Complex>,
    Complex: std::convert::From<T>,
{
    type Output = Complex;
    fn sub(self, value: T) -> Complex {
        let value_comp = Complex::from(value);
        let new_real = self.real - value_comp.real;
        let new_imag = self.imag - value_comp.imag;
        Complex::new(new_real, new_imag)
    }
}
impl<T> Mul<T> for Complex
where
    T: Into<Complex>,
    Complex: std::convert::From<T>,
{
    type Output = Complex;
    fn mul(self, v: T) -> Complex {
        let value = Complex::from(v);
        let new_real = self.real * value.real - self.imag * value.imag;
        let new_imag = self.real * value.imag + self.imag * value.real;
        Complex::new(new_real, new_imag)
    }
}
impl<T> AddAssign<T> for Complex
where
    T: Into<Complex>,
    Complex: std::convert::From<T>,
{
    fn add_assign(&mut self, value: T) {
        let value_comp = Complex::from(value);
        let new_real = self.real + value_comp.real;
        let new_imag = self.imag + value_comp.imag;
        self.real = new_real;
        self.imag = new_imag;
    }
}
impl<T> SubAssign<T> for Complex
where
    T: Into<Complex>,
    Complex: std::convert::From<T>,
{
    fn sub_assign(&mut self, value: T) {
        let value_comp = Complex::from(value);
        let new_real = self.real - value_comp.real;
        let new_imag = self.imag - value_comp.imag;
        self.real = new_real;
        self.imag = new_imag;
    }
}
impl<T> MulAssign<T> for Complex
where
    T: Into<Complex>,
    Complex: std::convert::From<T>,
{
    fn mul_assign(&mut self, v: T) {
        let value = Complex::from(v);
        let new_real = self.real * value.real - self.imag * value.imag;
        let new_imag = self.real * value.imag + self.imag * value.real;
        self.real = new_real;
        self.imag = new_imag;
    }
}
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        let new_real = -self.real;
        let new_imag = -self.imag;
        Complex::new(new_real, new_imag)
    }
}
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match (self.real, self.imag) {
            (0.0, 0.0) => write!(f, "0"),
            (_, 0.0) => write!(f, "{}", self.real),
            (0.0, imag) => write!(f, "{}i", imag),
            (real, imag) => {
                if imag > 0.0 {
                    write!(f, "{}+{}i", real, imag)
                } else {
                    write!(f, "{}{}i", real, imag)
                }
            }
        }
    }
}
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    //Testam AddAssign SubAssign si MulAssign

    let mut x = Complex::new(1, 2);
    x += 5;
    x += Complex::new(2, 3);
    assert_eq!(x, Complex::new(8, 5));

    let mut y = Complex::new(10, 5);
    y -= 3;
    y -= Complex::new(2, 1);
    assert_eq!(y, Complex::new(5, 4));

    let mut z = Complex::new(1, 1);
    z *= Complex::new(2, 3);
    assert_eq!(z, Complex::new(-1, 5));

    let mut a = Complex::new(2, 1);
    a += 3;
    a *= 2.0;
    a -= Complex::new(4, 2);
    assert_eq!(a, Complex::new(6, 0));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
