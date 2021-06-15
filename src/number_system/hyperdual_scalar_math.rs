/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use crate::number_system::HyperDualScalar as HDual;


impl HDual {
    pub fn sin(&self) -> HDual {
        HDual {
            re: self.re.sin(),
            e1: self.e1 * self.re.cos(),
            e2: self.e2 * self.re.cos(),
            e1e2: self.re.cos() * self.e1e2
                 -self.re.sin() * self.e1 * self.e2,
        }
    }

    pub fn cos(&self) -> HDual {
        HDual {
            re: self.re.cos(),
            e1: -self.e1 * self.re.sin(),
            e2: -self.e2 * self.re.sin(),
            e1e2: -self.re.sin() * self.e1e2
                  -self.re.cos() * self.e1 * self.e2 ,
        }
    }

    pub fn powi(&self, n: i32) -> HDual {
        let m = f64::from(n);
        HDual {
            re: self.re.powi(n),
            e1: m * self.re.powi(n-1) * self.e1,
            e2: m * self.re.powi(n-1) * self.e2,
            e1e2: m * (m-1.0) * self.re.powi(n-2) * self.e1 * self.e2
                 + m * self.re.powi(n-1) * self.e1e2,
        }
    }

}
