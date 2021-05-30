/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use std::ops::{Add, Sub, Neg, Mul, Div};
use ndarray::{Array1, Array};


#[derive(Debug, Clone)]
pub struct Dual {
    pub re: f64,
    pub du: Array1<f64>,
}


impl Dual {
    pub fn new(n: usize) -> Dual {
        Dual {
            re: 0.0,
            du: Array::zeros((n,)),
        }
    }
}


impl Add<Dual> for Dual {
    type Output = Dual;
    fn add(self, other: Dual) -> Dual {
        Dual {
            re: self.re + other.re,
            du: self.du + other.du,
        }
    }
}

impl Add<&Dual> for &Dual {
    type Output = Dual;
    fn add(self, other: &Dual) -> Dual {
        Dual {
            re: self.re + other.re,
            du: &self.du + &other.du,
        }
    }
}

impl Add<&Dual> for Dual {
    type Output = Dual;
    fn add(self, other: &Dual) -> Dual {
        Dual {
            re: self.re + other.re,
            du: self.du + &other.du,
        }
    }
}

impl Add<Dual> for &Dual {
    type Output = Dual;
    fn add(self, other: Dual) -> Dual {
        Dual {
            re: self.re + other.re,
            du: &self.du + other.du,
        }
    }
}

impl Add<f64> for Dual {
    type Output = Dual;
    fn add(self, other: f64) -> Dual {
        Dual {
            re: self.re + other,
            du: self.du,
        }
    }
}

impl Add<f64> for &Dual {
    type Output = Dual;
    fn add(self, other: f64) -> Dual {
        let mut grad = Array::zeros(self.du.raw_dim());
        grad.assign(&self.du);
        Dual {
            re: self.re + other,
            du: grad,
        }
    }
}

impl Add<Dual> for f64 {
    type Output = Dual;
    fn add(self, other: Dual) -> Dual {
        Dual {
            re: self + other.re,
            du: other.du,
        }
    }
}

impl Add<&Dual> for f64 {
    type Output = Dual;
    fn add(self, other: &Dual) -> Dual {
        let mut grad = Array::zeros(other.du.raw_dim());
        grad.assign(&other.du);
        Dual {
            re: self + other.re,
            du: grad,
        }
    }
}


impl Sub<Dual> for Dual {
    type Output = Dual;
    fn sub(self, other: Dual) -> Dual {
        Dual {
            re: self.re - other.re,
            du: self.du - other.du,
        }
    }
}

impl Sub<&Dual> for &Dual {
    type Output = Dual;
    fn sub(self, other: &Dual) -> Dual {
        Dual {
            re: self.re - other.re,
            du: &self.du - &other.du,
        }
    }
}

impl Sub<&Dual> for Dual {
    type Output = Dual;
    fn sub(self, other: &Dual) -> Dual {
        Dual {
            re: self.re - other.re,
            du: self.du - &other.du,
        }
    }
}

impl Sub<Dual> for &Dual {
    type Output = Dual;
    fn sub(self, other: Dual) -> Dual {
        Dual {
            re: self.re - other.re,
            du: &self.du - other.du,
        }
    }
}

impl Sub<f64> for Dual {
    type Output = Dual;
    fn sub(self, other: f64) -> Dual {
        Dual {
            re: self.re - other,
            du: self.du,
        }
    }
}

impl Sub<f64> for &Dual {
    type Output = Dual;
    fn sub(self, other: f64) -> Dual {
        let mut grad = Array::zeros(self.du.raw_dim());
        grad.assign(&self.du);
        Dual {
            re: self.re - other,
            du: grad,
        }
    }
}

impl Sub<Dual> for f64 {
    type Output = Dual;
    fn sub(self, other: Dual) -> Dual {
        Dual {
            re: self - other.re,
            du: -other.du,
        }
    }
}

impl Sub<&Dual> for f64 {
    type Output = Dual;
    fn sub(self, other: &Dual) -> Dual {
        let mut grad = Array::zeros(other.du.raw_dim());
        grad.assign(&other.du);
        Dual {
            re: self - other.re,
            du: -grad,
        }
    }
}


impl Neg for Dual {
    type Output = Dual;
    fn neg(self) -> Dual {
        Dual {
            re: -self.re,
            du: -self.du,
        }
    }
}

impl Neg for &Dual {
    type Output = Dual;
    fn neg(self) -> Dual {
        let mut grad = Array::zeros(self.du.raw_dim());
        grad.assign(&self.du);
        Dual {
            re: -self.re,
            du: -grad,
        }
    }
}

impl Mul<Dual> for Dual {
    type Output = Dual;
    fn mul(self, other: Dual) -> Dual {
        Dual {
            re: self.re * other.re,
            du: self.re * other.du + other.re * self.du,
        }
    }
}

impl Mul<&Dual> for &Dual {
    type Output = Dual;
    fn mul(self, other: &Dual) -> Dual {
        Dual {
            re: self.re * other.re,
            du: self.re * &other.du + other.re * &self.du,
        }
    }
}

impl Mul<&Dual> for Dual {
    type Output = Dual;
    fn mul(self, other: &Dual) -> Dual {
        Dual {
            re: self.re * other.re,
            du: self.re * &other.du + other.re * self.du,
        }
    }
}

impl Mul<Dual> for &Dual {
    type Output = Dual;
    fn mul(self, other: Dual) -> Dual {
        Dual {
            re: self.re * other.re,
            du: self.re * other.du + other.re * &self.du,
        }
    }
}

impl Mul<f64> for Dual {
    type Output = Dual;
    fn mul(self, other: f64) -> Dual {
        Dual {
            re: self.re * other,
            du: self.du * other,
        }
    }
}

impl Mul<f64> for &Dual {
    type Output = Dual;
    fn mul(self, other: f64) -> Dual {
        Dual {
            re: self.re * other,
            du: &self.du * other,
        }
    }
}

impl Mul<Dual> for f64 {
    type Output = Dual;
    fn mul(self, other: Dual) -> Dual {
        Dual {
            re: self * other.re,
            du: self * other.du,
        }
    }
}

impl Mul<&Dual> for f64 {
    type Output = Dual;
    fn mul(self, other: &Dual) -> Dual {
        Dual {
            re: self * other.re,
            du: self * &other.du,
        }
    }
}

impl Div<Dual> for Dual {
    type Output = Dual;
    fn div(self, other: Dual) -> Dual {
        Dual {
            re: self.re/other.re,
            du: self.du/other.re - self.re*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<&Dual> for &Dual {
    type Output = Dual;
    fn div(self, other: &Dual) -> Dual {
        Dual {
            re: self.re/other.re,
            du: &self.du/other.re - self.re*&other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<&Dual> for Dual {
    type Output = Dual;
    fn div(self, other: &Dual) -> Dual {
        Dual {
            re: self.re/other.re,
            du: self.du/other.re - self.re*&other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<Dual> for &Dual {
    type Output = Dual;
    fn div(self, other: Dual) -> Dual {
        Dual {
            re: self.re/other.re,
            du: &self.du/other.re - self.re*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<f64> for Dual {
    type Output = Dual;
    fn div(self, other: f64) -> Dual {
        Dual {
            re: self.re/other,
            du: self.du/other,
        }
    }
}

impl Div<f64> for &Dual {
    type Output = Dual;
    fn div(self, other: f64) -> Dual {
        Dual {
            re: self.re/other,
            du: &self.du/other,
        }
    }
}

impl Div<Dual> for f64 {
    type Output = Dual;
    fn div(self, other: Dual) -> Dual {
        Dual {
            re: self/other.re,
            du: -self*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<&Dual> for f64 {
    type Output = Dual;
    fn div(self, other: &Dual) -> Dual {
        Dual {
            re: self/other.re,
            du: -self*&other.du/f64::powi(other.re, 2),
        }
    }
}
