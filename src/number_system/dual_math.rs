/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use crate::number_system::Dual;

impl Dual {
    pub fn sin(&self) -> Dual {
        Dual {
            re: self.re.sin(),
            du: &self.du * self.re.cos(),
        }
    }

    pub fn cos(&self) -> Dual {
        Dual {
            re: self.re.cos(),
            du: &self.du * -self.re.sin(),
        }
    }

    pub fn powi(&self, n:i32) -> Dual {
        let m = f64::from(n);
        Dual {
            re: self.re.powi(n),
            du: &self.du * m * self.re.powi(n-1),
        }
    }
}
