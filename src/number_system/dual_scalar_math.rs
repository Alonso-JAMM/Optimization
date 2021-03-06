/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use crate::number_system::DualScalar;


impl DualScalar {
    #[inline(always)]
    pub fn sin(&self) -> DualScalar {
        DualScalar {
            re: self.re.sin(),
            du: self.du * self.re.cos(),
        }
    }
    #[inline(always)]
    pub fn cos(&self) -> DualScalar {
        DualScalar {
            re: self.re.cos(),
            du: self.du * -self.re.sin(),
        }
    }

    #[inline(always)]
    pub fn powi(&self, n: i32) -> DualScalar {
        let m = f64::from(n);
        DualScalar {
            re: self.re.powi(n),
            du: self.du * m * self.re.powi(n-1),
        }
    }
}
