/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use std::ops::{Add, Sub, Neg, Mul, Div};


#[derive(Debug, Clone, Copy)]
pub struct HyperDualScalar {
    pub re: f64,
    pub e1: f64,
    pub e2: f64,
    pub e1e2: f64,
}


impl HyperDualScalar {
    pub fn new() -> HyperDualScalar {
        HyperDualScalar {
            re: 0.0,
            e1: 0.0,
            e2: 0.0,
            e1e2: 0.0,
        }
    }
}


impl Add<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re + other.re,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e1e2: self.e1e2 + other.e1e2,
        }
    }
}

impl Add<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re + other.re,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e1e2: self.e1e2 + other.e1e2,
        }
    }
}

impl Add<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re + other.re,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e1e2: self.e1e2 + other.e1e2,
        }
    }
}

impl Add<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re + other.re,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e1e2: self.e1e2 + other.e1e2,
        }
    }
}

impl Add<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re + other,
            e1: self.e1,
            e2: self.e2,
            e1e2: self.e1e2,
        }
    }
}

impl Add<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn add(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re + other,
            e1: self.e1,
            e2: self.e2,
            e1e2: self.e1e2,
        }
    }
}

impl Add<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn add(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self+ other.re,
            e1: other.e1,
            e2: other.e2,
            e1e2: other.e1e2,
        }
    }
}

impl Add<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn add(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self+ other.re,
            e1: other.e1,
            e2: other.e2,
            e1e2: other.e1e2,
        }
    }
}


impl Sub<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re - other.re,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e1e2: self.e1e2 - other.e1e2,
        }
    }
}

impl Sub<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re - other.re,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e1e2: self.e1e2 - other.e1e2,
        }
    }
}

impl Sub<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re - other.re,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e1e2: self.e1e2 - other.e1e2,
        }
    }
}

impl Sub<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re - other.re,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e1e2: self.e1e2 - other.e1e2,
        }
    }
}

impl Sub<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re - other,
            e1: self.e1,
            e2: self.e2,
            e1e2: self.e1e2,
        }
    }
}

impl Sub<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn sub(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re - other,
            e1: self.e1,
            e2: self.e2,
            e1e2: self.e1e2,
        }
    }
}

impl Sub<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn sub(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self - other.re,
            e1: -other.e1,
            e2: -other.e2,
            e1e2: -other.e1e2,
        }
    }
}

impl Sub<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn sub(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self - other.re,
            e1: -other.e1,
            e2: -other.e2,
            e1e2: -other.e1e2,
        }
    }
}


impl Neg for HyperDualScalar {
    type Output = HyperDualScalar;
    fn neg(self) -> HyperDualScalar {
        HyperDualScalar {
            re: -self.re,
            e1: -self.e1,
            e2: -self.e2,
            e1e2: -self.e1e2,
        }
    }
}

impl Neg for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn neg(self) -> HyperDualScalar {
        HyperDualScalar {
            re: -self.re,
            e1: -self.e1,
            e2: -self.e2,
            e1e2: -self.e1e2,
        }
    }
}


impl Mul<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re * other.re,
            e1: self.re * other.e1 + self.e1 * other.re,
            e2: self.re * other.e2 + self.e2 * other.re,
            e1e2: self.re * other.e1e2
                  + self.e1 * other.e2
                  + self.e2 * other.e1
                  + other.re*self.e1e2,
        }
    }
}

impl Mul<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re * other.re,
            e1: self.re * other.e1 + self.e1 * other.re,
            e2: self.re * other.e2 + self.e2 * other.re,
            e1e2: self.re * other.e1e2
                  + self.e1 * other.e2
                  + self.e2 * other.e1
                  + other.re*self.e1e2,
        }
    }
}

impl Mul<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re * other.re,
            e1: self.re * other.e1 + self.e1 * other.re,
            e2: self.re * other.e2 + self.e2 * other.re,
            e1e2: self.re * other.e1e2
                  + self.e1 * other.e2
                  + self.e2 * other.e1
                  + other.re*self.e1e2,
        }
    }
}

impl Mul<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re * other.re,
            e1: self.re * other.e1 + self.e1 * other.re,
            e2: self.re * other.e2 + self.e2 * other.re,
            e1e2: self.re * other.e1e2
                  + self.e1 * other.e2
                  + self.e2 * other.e1
                  + other.re*self.e1e2,
        }
    }
}

impl Mul<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re * other,
            e1: self.e1 * other,
            e2: self.e2 * other,
            e1e2: self.e1e2 * other,
        }
    }
}

impl Mul<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn mul(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re * other,
            e1: self.e1 * other,
            e2: self.e2 * other,
            e1e2: self.e1e2 * other,
        }
    }
}

impl Mul<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn mul(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self * other.re,
            e1: self * other.e1,
            e2: self * other.e2,
            e1e2: self * other.e1e2,
        }
    }
}

impl Mul<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn mul(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self * other.re,
            e1: self * other.e1,
            e2: self * other.e2,
            e1e2: self * other.e1e2,
        }
    }
}


impl Div<HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re / other.re,
            e1: self.e1 / other.re - self.re * other.e1 / other.re.powi(2),
            e2: self.e2 / other.re - self.re * other.e2 / other.re.powi(2),
            e1e2: self.e1e2 / other.re
                  - self.e2 * other.e1 / other.re.powi(2)
                  - self.e1 * other.e2 / other.re.powi(2)
                  + 2.0 * other.e1*other.e2*self.re / other.re.powi(3)
                  - self.re * other.e1e2 / other.re.powi(2),
        }
    }
}

impl Div<&HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re / other.re,
            e1: self.e1 / other.re - self.re * other.e1 / other.re.powi(2),
            e2: self.e2 / other.re - self.re * other.e2 / other.re.powi(2),
            e1e2: self.e1e2 / other.re
                  - self.e2 * other.e1 / other.re.powi(2)
                  - self.e1 * other.e2 / other.re.powi(2)
                  + 2.0 * other.e1*other.e2*self.re / other.re.powi(3)
                  - self.re * other.e1e2 / other.re.powi(2),
        }
    }
}

impl Div<HyperDualScalar> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re / other.re,
            e1: self.e1 / other.re - self.re * other.e1 / other.re.powi(2),
            e2: self.e2 / other.re - self.re * other.e2 / other.re.powi(2),
            e1e2: self.e1e2 / other.re
                  - self.e2 * other.e1 / other.re.powi(2)
                  - self.e1 * other.e2 / other.re.powi(2)
                  + 2.0 * other.e1*other.e2*self.re / other.re.powi(3)
                  - self.re * other.e1e2 / other.re.powi(2),
        }
    }
}

impl Div<&HyperDualScalar> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re / other.re,
            e1: self.e1 / other.re - self.re * other.e1 / other.re.powi(2),
            e2: self.e2 / other.re - self.re * other.e2 / other.re.powi(2),
            e1e2: self.e1e2 / other.re
                  - self.e2 * other.e1 / other.re.powi(2)
                  - self.e1 * other.e2 / other.re.powi(2)
                  + 2.0 * other.e1*other.e2*self.re / other.re.powi(3)
                  - self.re * other.e1e2 / other.re.powi(2),
        }
    }
}

impl Div<f64> for HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re / other,
            e1: self.e1 / other,
            e2: self.e2 / other,
            e1e2: self.e1e2 / other,
        }
    }
}

impl Div<f64> for &HyperDualScalar {
    type Output = HyperDualScalar;
    fn div(self, other: f64) -> HyperDualScalar {
        HyperDualScalar {
            re: self.re / other,
            e1: self.e1 / other,
            e2: self.e2 / other,
            e1e2: self.e1e2 / other,
        }
    }
}

impl Div<HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn div(self, other: HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self / other.re,
            e1: -self * other.e1 / other.re.powi(2),
            e2: -self * other.e2 / other.re.powi(2),
            e1e2: 2.0 * other.e1*other.e2*self / other.re.powi(3)
                  - self * other.e1e2 / other.re.powi(2),
        }
    }
}

impl Div<&HyperDualScalar> for f64 {
    type Output = HyperDualScalar;
    fn div(self, other: &HyperDualScalar) -> HyperDualScalar {
        HyperDualScalar {
            re: self / other.re,
            e1: -self * other.e1 / other.re.powi(2),
            e2: -self * other.e2 / other.re.powi(2),
            e1e2: 2.0 * other.e1*other.e2*self / other.re.powi(3)
                  - self * other.e1e2 / other.re.powi(2),
        }
    }
}
