/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use std::ops::{Add, Sub, Neg, Mul, Div};

use crate::number_system::HyperDualScalar as HDual;
use crate::geometry::HDQuaternion;

/// Vector where all of its components are hyperdual scalars
#[derive(Debug, Clone, Copy)]
pub struct HDVector {
    pub x: HDual,
    pub y: HDual,
    pub z: HDual,
}


impl HDVector {
    pub fn new() -> HDVector {
        HDVector {
            x: HDual::new(),
            y: HDual::new(),
            z: HDual::new(),
        }
    }

    /// Creates a new vector from a quaternion. It takes the vector component of
    /// the quaternion
    pub fn from_quaternion(q: &HDQuaternion) -> HDVector {
        HDVector {
            x: q.q0,
            y: q.q1,
            z: q.q2,
        }
    }

    /// Dot product between two vectors
    pub fn dot(&self, other: &HDVector) -> HDual {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    /// Cross product between two vectors
    pub fn cross(&self, other: &HDVector) -> HDVector {
        HDVector {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
}


impl Add<HDVector> for HDVector {
    type Output = HDVector;
    fn add(self, other: HDVector) -> HDVector {
        HDVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&HDVector> for &HDVector {
    type Output = HDVector;
    fn add(self, other: &HDVector) -> HDVector {
        HDVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&HDVector> for HDVector {
    type Output = HDVector;
    fn add(self, other: &HDVector) -> HDVector {
        HDVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<HDVector> for &HDVector {
    type Output = HDVector;
    fn add(self, other: HDVector) -> HDVector {
        HDVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl Sub<HDVector> for HDVector {
    type Output = HDVector;
    fn sub(self, other: HDVector) -> HDVector {
        HDVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&HDVector> for &HDVector {
    type Output = HDVector;
    fn sub(self, other: &HDVector) -> HDVector {
        HDVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&HDVector> for HDVector {
    type Output = HDVector;
    fn sub(self, other: &HDVector) -> HDVector {
        HDVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<HDVector> for &HDVector {
    type Output = HDVector;
    fn sub(self, other: HDVector) -> HDVector {
        HDVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Neg for HDVector {
    type Output = HDVector;
    fn neg(self) -> HDVector {
        HDVector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &HDVector {
    type Output = HDVector;
    fn neg(self) -> HDVector {
        HDVector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


impl Mul<f64> for HDVector {
    type Output = HDVector;
    fn mul(self, other: f64) -> HDVector {
        HDVector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f64> for &HDVector {
    type Output = HDVector;
    fn mul(self, other: f64) -> HDVector {
        HDVector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<HDVector> for f64 {
    type Output = HDVector;
    fn mul(self, other: HDVector) -> HDVector {
        HDVector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<&HDVector> for f64 {
    type Output = HDVector;
    fn mul(self, other: &HDVector) -> HDVector {
        HDVector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}


impl Mul<HDual> for HDVector {
    type Output = HDVector;
    fn mul(self, other: HDual) -> HDVector {
        HDVector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<&HDual> for &HDVector {
    type Output = HDVector;
    fn mul(self, other: &HDual) -> HDVector {
        HDVector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<HDual> for &HDVector {
    type Output = HDVector;
    fn mul(self, other: HDual) -> HDVector {
        HDVector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<&HDual> for HDVector {
    type Output = HDVector;
    fn mul(self, other: &HDual) -> HDVector {
        HDVector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<HDVector> for HDual {
    type Output = HDVector;
    fn mul(self, other: HDVector) -> HDVector {
        HDVector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<&HDVector> for &HDual {
    type Output = HDVector;
    fn mul(self, other: &HDVector) -> HDVector {
        HDVector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<&HDVector> for HDual {
    type Output = HDVector;
    fn mul(self, other: &HDVector) -> HDVector {
        HDVector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<HDVector> for &HDual {
    type Output = HDVector;
    fn mul(self, other: HDVector) -> HDVector {
        HDVector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}


impl Div<f64> for HDVector {
    type Output = HDVector;
    fn div(self, other: f64) -> HDVector {
        HDVector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<f64> for &HDVector {
    type Output = HDVector;
    fn div(self, other: f64) -> HDVector {
        HDVector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
