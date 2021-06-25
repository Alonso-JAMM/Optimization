/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use std::ops::{Add, Sub, Neg, Mul, Div};

use crate::number_system::HyperDualScalar as HDual;


/// Quaternion where all of its components are scalar hyper dual numbers.
#[derive(Debug, Clone, Copy)]
pub struct HDQuaternion {
    pub q0: HDual,
    pub q1: HDual,
    pub q2: HDual,
    pub q3: HDual,
}


impl HDQuaternion {
    pub fn new() -> HDQuaternion {
        HDQuaternion {
            q0: HDual::new(),
            q1: HDual::new(),
            q2: HDual::new(),
            q3: HDual::new(),
        }
    }

    /// Creates a new quaternion using the rotation angle about the x-axis
    pub fn from_x_angle(phi: HDual) -> HDQuaternion {
        HDQuaternion {
            q0: (phi/2.0).sin(),
            q1: HDual::new(),
            q2: HDual::new(),
            q3: (phi/2.0).cos(),
        }
    }

    /// Creates a new quaternion using the rotation angle about the y-axis
    pub fn from_y_angle(theta: HDual) -> HDQuaternion {
        HDQuaternion {
            q0: HDual::new(),
            q1: (theta/2.0).sin(),
            q2: HDual::new(),
            q3: (theta/2.0).cos(),
        }
    }

    /// Creates a new quaternion using the rotation angle about the z-axis
    pub fn from_z_angle(psi: HDual) -> HDQuaternion {
        HDQuaternion {
            q0: HDual::new(),
            q1: HDual::new(),
            q2: (psi/2.0).sin(),
            q3: (psi/2.0).cos(),
        }
    }

    /// Creates a new quaternion using the 3 rotation angles about x, y, and z
    /// axes
    pub fn from_angles(phi: HDual, theta: HDual, psi: HDual) -> HDQuaternion {
        let qz = HDQuaternion::from_z_angle(psi);
        let qy = HDQuaternion::from_z_angle(theta);
        let qx = HDQuaternion::from_z_angle(phi);
        qz*qy*qx
    }

    /// Returns the inverse of the quaternion. IT ASSUMES THE QUATERNION IS
    /// UNIT QUATERNION
    pub fn inv(&self) -> HDQuaternion {
        HDQuaternion {
            q0: -self.q0,
            q1: -self.q1,
            q2: -self.q2,
            q3: self.q3,
        }
    }
}


impl Add<HDQuaternion> for HDQuaternion {
    type Output = HDQuaternion;
    fn add(self, other: HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 + other.q0,
            q1: self.q1 + other.q1,
            q2: self.q2 + other.q2,
            q3: self.q3 + other.q3,
        }
    }
}

impl Add<&HDQuaternion> for &HDQuaternion {
    type Output = HDQuaternion;
    fn add(self, other: &HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 + other.q0,
            q1: self.q1 + other.q1,
            q2: self.q2 + other.q2,
            q3: self.q3 + other.q3,
        }
    }
}

impl Add<HDQuaternion> for &HDQuaternion {
    type Output = HDQuaternion;
    fn add(self, other: HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 + other.q0,
            q1: self.q1 + other.q1,
            q2: self.q2 + other.q2,
            q3: self.q3 + other.q3,
        }
    }
}

impl Add<&HDQuaternion> for HDQuaternion {
    type Output = HDQuaternion;
    fn add(self, other: &HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 + other.q0,
            q1: self.q1 + other.q1,
            q2: self.q2 + other.q2,
            q3: self.q3 + other.q3,
        }
    }
}


impl Sub<HDQuaternion> for HDQuaternion {
    type Output = HDQuaternion;
    fn sub(self, other: HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 - other.q0,
            q1: self.q1 - other.q1,
            q2: self.q2 - other.q2,
            q3: self.q3 - other.q3,
        }
    }
}

impl Sub<&HDQuaternion> for &HDQuaternion {
    type Output = HDQuaternion;
    fn sub(self, other: &HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 - other.q0,
            q1: self.q1 - other.q1,
            q2: self.q2 - other.q2,
            q3: self.q3 - other.q3,
        }
    }
}

impl Sub<&HDQuaternion> for HDQuaternion {
    type Output = HDQuaternion;
    fn sub(self, other: &HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 - other.q0,
            q1: self.q1 - other.q1,
            q2: self.q2 - other.q2,
            q3: self.q3 - other.q3,
        }
    }
}

impl Sub<HDQuaternion> for &HDQuaternion {
    type Output = HDQuaternion;
    fn sub(self, other: HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 - other.q0,
            q1: self.q1 - other.q1,
            q2: self.q2 - other.q2,
            q3: self.q3 - other.q3,
        }
    }
}


impl Neg for HDQuaternion {
    type Output = HDQuaternion;
    fn neg(self) -> HDQuaternion {
        HDQuaternion {
            q0: -self.q0,
            q1: -self.q1,
            q2: -self.q2,
            q3: -self.q3,
        }
    }
}

impl Neg for &HDQuaternion {
    type Output = HDQuaternion;
    fn neg(self) -> HDQuaternion {
        HDQuaternion {
            q0: -self.q0,
            q1: -self.q1,
            q2: -self.q2,
            q3: -self.q3,
        }
    }
}


impl Mul<HDQuaternion> for HDQuaternion {
    type Output = HDQuaternion;
    fn mul(self, other: HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q3*other.q0 + self.q0*other.q3 + self.q1*other.q2 - self.q2*other.q1,
            q1: self.q3*other.q1 - self.q0*other.q2 + self.q1*other.q3 + self.q2*other.q0,
            q2: self.q3*other.q2 + self.q0*other.q1 - self.q1*other.q0 + self.q2*other.q3,
            q3: self.q3*other.q3 - self.q0*other.q0 - self.q1*other.q1 - self.q2*other.q2,
        }
    }
}

impl Mul<&HDQuaternion> for &HDQuaternion {
    type Output = HDQuaternion;
    fn mul(self, other: &HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q3*other.q0 + self.q0*other.q3 + self.q1*other.q2 - self.q2*other.q1,
            q1: self.q3*other.q1 - self.q0*other.q2 + self.q1*other.q3 + self.q2*other.q0,
            q2: self.q3*other.q2 + self.q0*other.q1 - self.q1*other.q0 + self.q2*other.q3,
            q3: self.q3*other.q3 - self.q0*other.q0 - self.q1*other.q1 - self.q2*other.q2,
        }
    }
}

impl Mul<&HDQuaternion> for HDQuaternion {
    type Output = HDQuaternion;
    fn mul(self, other: &HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q3*other.q0 + self.q0*other.q3 + self.q1*other.q2 - self.q2*other.q1,
            q1: self.q3*other.q1 - self.q0*other.q2 + self.q1*other.q3 + self.q2*other.q0,
            q2: self.q3*other.q2 + self.q0*other.q1 - self.q1*other.q0 + self.q2*other.q3,
            q3: self.q3*other.q3 - self.q0*other.q0 - self.q1*other.q1 - self.q2*other.q2,
        }
    }
}


impl Mul<HDQuaternion> for &HDQuaternion {
    type Output = HDQuaternion;
    fn mul(self, other: HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self.q3*other.q0 + self.q0*other.q3 + self.q1*other.q2 - self.q2*other.q1,
            q1: self.q3*other.q1 - self.q0*other.q2 + self.q1*other.q3 + self.q2*other.q0,
            q2: self.q3*other.q2 + self.q0*other.q1 - self.q1*other.q0 + self.q2*other.q3,
            q3: self.q3*other.q3 - self.q0*other.q0 - self.q1*other.q1 - self.q2*other.q2,
        }
    }
}

impl Mul<f64> for HDQuaternion {
    type Output = HDQuaternion;
    fn mul(self, other: f64) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 * other,
            q1: self.q1 * other,
            q2: self.q2 * other,
            q3: self.q3 * other,
        }
    }
}

impl Mul<f64> for &HDQuaternion {
    type Output = HDQuaternion;
    fn mul(self, other: f64) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 * other,
            q1: self.q1 * other,
            q2: self.q2 * other,
            q3: self.q3 * other,
        }
    }
}

impl Mul<HDQuaternion> for f64 {
    type Output = HDQuaternion;
    fn mul(self, other: HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self * other.q0,
            q1: self * other.q1,
            q2: self * other.q2,
            q3: self * other.q3,
        }
    }
}

impl Mul<&HDQuaternion> for f64 {
    type Output = HDQuaternion;
    fn mul(self, other: &HDQuaternion) -> HDQuaternion {
        HDQuaternion {
            q0: self * other.q0,
            q1: self * other.q1,
            q2: self * other.q2,
            q3: self * other.q3,
        }
    }
}


// I don't think division of quaternion by quaternion is going to be used anyways
// so I don't include them

impl Div<f64> for HDQuaternion {
    type Output = HDQuaternion;
    fn div(self, other: f64) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 / other,
            q1: self.q1 / other,
            q2: self.q2 / other,
            q3: self.q3 / other,
        }
    }
}

impl Div<f64> for &HDQuaternion {
    type Output = HDQuaternion;
    fn div(self, other: f64) -> HDQuaternion {
        HDQuaternion {
            q0: self.q0 / other,
            q1: self.q1 / other,
            q2: self.q2 / other,
            q3: self.q3 / other,
        }
    }
}
