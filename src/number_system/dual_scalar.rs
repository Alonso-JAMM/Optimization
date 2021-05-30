/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use std::ops::{Add, Sub, Neg, Mul, Div};


#[derive(Debug, Clone, Copy)]
pub struct DualScalar {
    pub re: f64,
    pub du: f64,
}

impl DualScalar {
    pub fn new() -> DualScalar {
        DualScalar {
            re: 0.0,
            du: 0.0,
        }
    }
}


impl Add<DualScalar> for DualScalar {
    type Output = DualScalar;
    fn add(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re + other.re,
            du: self.du + other.du,
        }
    }
}

impl Add<&DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn add(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re + other.re,
            du: self.du + other.du,
        }
    }
}

impl Add<&DualScalar> for DualScalar {
    type Output = DualScalar;
    fn add(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re + other.re,
            du: self.du + other.du,
        }
    }
}

impl Add<DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn add(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re + other.re,
            du: self.du + other.du,
        }
    }
}

impl Add<f64> for DualScalar {
    type Output = DualScalar;
    fn add(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re + other,
            du: self.du,
        }
    }
}

impl Add<DualScalar> for f64 {
    type Output = DualScalar;
    fn add(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self + other.re,
            du: other.du,
        }
    }
}

impl Add<f64> for &DualScalar {
    type Output = DualScalar;
    fn add(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re + other,
            du: self.du,
        }
    }
}

impl Add<&DualScalar> for f64 {
    type Output = DualScalar;
    fn add(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self + other.re,
            du: other.du,
        }
    }
}


impl Sub<DualScalar> for DualScalar {
    type Output = DualScalar;
    fn sub(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re - other.re,
            du: self.du - other.du,
        }
    }
}

impl Sub<&DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn sub(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re - other.re,
            du: self.du - other.du,
        }
    }
}

impl Sub<&DualScalar> for DualScalar {
    type Output = DualScalar;
    fn sub(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re - other.re,
            du: self.du - other.du,
        }
    }
}

impl Sub<DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn sub(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re - other.re,
            du: self.du - other.du,
        }
    }
}

impl Sub<f64> for DualScalar {
    type Output = DualScalar;
    fn sub(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re - other,
            du: self.du,
        }
    }
}

impl Sub<f64> for &DualScalar {
    type Output = DualScalar;
    fn sub(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re - other,
            du: self.du,
        }
    }
}

impl Sub<DualScalar> for f64 {
    type Output = DualScalar;
    fn sub(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self - other.re,
            du: -other.du,
        }
    }
}

impl Sub<&DualScalar> for f64 {
    type Output = DualScalar;
    fn sub(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self - other.re,
            du: -other.du,
        }
    }
}


impl Neg for DualScalar {
    type Output = DualScalar;
    fn neg(self) -> DualScalar {
        DualScalar {
            re: -self.re,
            du: -self.du,
        }
    }
}

impl Neg for &DualScalar {
    type Output = DualScalar;
    fn neg(self) -> DualScalar {
        DualScalar {
            re: -self.re,
            du: -self.du,
        }
    }
}


impl Mul<DualScalar> for DualScalar {
    type Output = DualScalar;
    fn mul(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re * other.re,
            du: self.re * other.du + other.re * self.du,
        }
    }
}

impl Mul<&DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn mul(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re * other.re,
            du: self.re * other.du + other.re * self.du,
        }
    }
}

impl Mul<&DualScalar> for DualScalar {
    type Output = DualScalar;
    fn mul(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re * other.re,
            du: self.re * other.du + other.re * self.du,
        }
    }
}

impl Mul<DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn mul(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re * other.re,
            du: self.re * other.du + other.re * self.du,
        }
    }
}

impl Mul<f64> for DualScalar {
    type Output = DualScalar;
    fn mul(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re * other,
            du: other * self.du,
        }
    }
}

impl Mul<f64> for &DualScalar {
    type Output = DualScalar;
    fn mul(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re * other,
            du: other * self.du,
        }
    }
}

impl Mul<DualScalar> for f64 {
    type Output = DualScalar;
    fn mul(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self * other.re,
            du: self * other.du,
        }
    }
}

impl Mul<&DualScalar> for f64 {
    type Output = DualScalar;
    fn mul(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self * other.re,
            du: self * other.du,
        }
    }
}


impl Div<DualScalar> for DualScalar {
    type Output = DualScalar;
    fn div(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re/other.re,
            du: self.du/other.re - self.re*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<&DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn div(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re/other.re,
            du: self.du/other.re - self.re*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<&DualScalar> for DualScalar {
    type Output = DualScalar;
    fn div(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self.re/other.re,
            du: self.du/other.re - self.re*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<DualScalar> for &DualScalar {
    type Output = DualScalar;
    fn div(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self.re/other.re,
            du: self.du/other.re - self.re*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<f64> for DualScalar {
    type Output = DualScalar;
    fn div(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re/other,
            du: self.du/other,
        }
    }
}

impl Div<f64> for &DualScalar {
    type Output = DualScalar;
    fn div(self, other: f64) -> DualScalar {
        DualScalar {
            re: self.re/other,
            du: self.du/other,
        }
    }
}

impl Div<DualScalar> for f64 {
    type Output = DualScalar;
    fn div(self, other: DualScalar) -> DualScalar {
        DualScalar {
            re: self/other.re,
            du: self*other.du/f64::powi(other.re, 2),
        }
    }
}

impl Div<&DualScalar> for f64 {
    type Output = DualScalar;
    fn div(self, other: &DualScalar) -> DualScalar {
        DualScalar {
            re: self/other.re,
            du: self*other.du/f64::powi(other.re, 2),
        }
    }
}
