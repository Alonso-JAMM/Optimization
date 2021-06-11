/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use std::ops::{Add, Sub, Neg, Mul, Div};


#[derive(Debug, Clone, Copy)]
pub struct HyperDualScalar {
    pub real: f64,
    pub grad: f64,
    pub hess: f64,
}


impl HyperDualScalar {
    pub fn new() -> HyperDualScalar {
        HyperDualScalar {
            real: 0.0,
            grad: 0.0,
            hess: 0.0,
        }
    }
}


impl Add<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real + other.real,
            grad: self.grad + other.grad,
            hess: self.hess + other.hess,
        }
    }
}

impl Add<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real + other.real,
            grad: self.grad + other.grad,
            hess: self.hess + other.hess,
        }
    }
}

impl Add<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real + other.real,
            grad: self.grad + other.grad,
            hess: self.hess + other.hess,
        }
    }
}

impl Add<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real + other.real,
            grad: self.grad + other.grad,
            hess: self.hess + other.hess,
        }
    }
}

impl Add<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real + other,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl Add<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real + other,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl Add<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn add(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self+ other.real,
            grad: other.grad,
            hess: other.hess,
        }
    }
}

impl Add<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn add(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self+ other.real,
            grad: other.grad,
            hess: other.hess,
        }
    }
}


impl Sub<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real - other.real,
            grad: self.grad - other.grad,
            hess: self.hess - other.hess,
        }
    }
}

impl Sub<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real - other.real,
            grad: self.grad - other.grad,
            hess: self.hess - other.hess,
        }
    }
}

impl Sub<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real - other.real,
            grad: self.grad - other.grad,
            hess: self.hess - other.hess,
        }
    }
}

impl Sub<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real - other.real,
            grad: self.grad - other.grad,
            hess: self.hess - other.hess,
        }
    }
}

impl Sub<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real - other,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl Sub<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real - other,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl Sub<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn sub(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self - other.real,
            grad: -other.grad,
            hess: -other.hess,
        }
    }
}

impl Sub<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn sub(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self - other.real,
            grad: -other.grad,
            hess: -other.hess,
        }
    }
}


impl Neg for HyperDualScalar {
    type Output = HyperDualScalar;
    fn neg(self) -> HyperDualScalar {
        HyperDualScalar {
            real: -self.real,
            grad: -self.grad,
            hess: -self.hess,
        }
    }
}

impl Neg for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn neg(self) -> HyperDualScalar {
        HyperDualScalar {
            real: -self.real,
            grad: -self.grad,
            hess: -self.hess,
        }
    }
}


impl Mul<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real * other.real,
            grad: self.real * other.grad + self.grad * other.real,
            hess: self.real * other.hess + 2.0*self.grad*other.grad + other.real*self.hess,
        }
    }
}

impl Mul<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real * other.real,
            grad: self.real * other.grad + self.grad * other.real,
            hess: self.real * other.hess + 2.0*self.grad*other.grad + other.real*self.hess,
        }
    }
}

impl Mul<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real * other.real,
            grad: self.real * other.grad + self.grad * other.real,
            hess: self.real * other.hess + 2.0*self.grad*other.grad + other.real*self.hess,
        }
    }
}

impl Mul<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real * other.real,
            grad: self.real * other.grad + self.grad * other.real,
            hess: self.real * other.hess + 2.0*self.grad*other.grad + other.real*self.hess,
        }
    }
}

impl Mul<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real * other,
            grad: self.grad * other,
            hess: self.hess * other,
        }
    }
}

impl Mul<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real * other,
            grad: self.grad * other,
            hess: self.hess * other,
        }
    }
}

impl Mul<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn mul(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self * other.real,
            grad: self * other.grad,
            hess: self * other.hess,
        }
    }
}

impl Mul<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn mul(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self * other.real,
            grad: self * other.grad,
            hess: self * other.hess,
        }
    }
}


impl Div<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real / other.real,
            grad: self.grad / other.real - self.real * other.grad / other.real.powi(2),
            hess: self.hess / other.real - 2.0*self.grad*other.grad/other.real.powi(2)
                  + 2.0*other.grad*other.grad*self.real/other.real.powi(3)
                  - self.real*other.hess/other.real.powi(2),
        }
    }
}

impl Div<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real / other.real,
            grad: self.grad / other.real - self.real * other.grad / other.real.powi(2),
            hess: self.hess / other.real - 2.0*self.grad*other.grad/other.real.powi(2)
                  + 2.0*other.grad*other.grad*self.real/other.real.powi(3)
                  - self.real*other.hess/other.real.powi(2),
        }
    }
}

impl Div<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real / other.real,
            grad: self.grad / other.real - self.real * other.grad / other.real.powi(2),
            hess: self.hess / other.real - 2.0*self.grad*other.grad/other.real.powi(2)
                  + 2.0*other.grad*other.grad*self.real/other.real.powi(3)
                  - self.real*other.hess/other.real.powi(2),
        }
    }
}

impl Div<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real / other.real,
            grad: self.grad / other.real - self.real * other.grad / other.real.powi(2),
            hess: self.hess / other.real - 2.0*self.grad*other.grad/other.real.powi(2)
                  + 2.0*other.grad*other.grad*self.real/other.real.powi(3)
                  - self.real*other.hess/other.real.powi(2),
        }
    }
}

impl Div<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real / other,
            grad: self.grad / other,
            hess: self.hess / other,
        }
    }
}

impl Div<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            real: self.real / other,
            grad: self.grad / other,
            hess: self.hess / other,
        }
    }
}

impl Div<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn div(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self / other.real,
            grad: -self * other.grad / other.real.powi(2),
            hess: 2.0*other.grad*other.grad*self/other.real.powi(3)
                  - self*other.hess/other.real.powi(2),
        }
    }
}

impl Div<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn div(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            real: self / other.real,
            grad: -self * other.grad / other.real.powi(2),
            hess: 2.0*other.grad*other.grad*self/other.real.powi(3)
                  - self*other.hess/other.real.powi(2),
        }
    }
}
