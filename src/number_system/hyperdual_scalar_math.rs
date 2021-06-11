/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use crate::number_system::HyperDualScalar as HDual;


impl HDual {
    pub fn sin(&self) -> HDual {
        HDual {
            real: self.real.sin(),
            grad: self.grad * self.real.cos(),
            hess: -self.real.sin()*self.grad*self.grad + self.real.cos()*self.hess,
        }
    }

    pub fn cos(&self) -> HDual {
        HDual {
            real: self.real.cos(),
            grad: -self.grad * self.real.sin(),
            hess: -self.real.cos()*self.grad*self.grad - self.real.sin()*self.hess,
        }
    }

    pub fn powi(&self, n: i32) -> HDual {
        let m = f64::from(n);
        HDual {
            real: self.real.powi(n),
            grad: m * self.real.powi(n-1) * self.grad,
            hess: m * (m-1.0) * self.real.powi(n-2) * self.grad * self.grad
                 + m * self.real.powi(n-1) * self.hess,
        }
    }

}
