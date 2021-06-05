/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use crate::number_system::HyperDual;
use ndarray::Array;


impl HyperDual {
    pub fn sin(&self) -> HyperDual {
        let mut hess_left = Array::zeros(self.hess.raw_dim());
        hess_left += &self.grad;
        {
            let mut hess_left_t = hess_left.view_mut().reversed_axes();
            hess_left_t *= &self.grad;
        }
        HyperDual {
            real: self.real.sin(),
            grad: &self.grad * self.real.cos(),
            hess: &self.hess * self.real.cos() - hess_left * self.real.sin()
        }
    }

    pub fn cos(&self) -> HyperDual {
        let mut hess_left = Array::zeros(self.hess.raw_dim());
        hess_left += &self.grad;
        {
            let mut hess_left_t = hess_left.view_mut().reversed_axes();
            hess_left_t *= &self.grad;
        }
        HyperDual {
            real: self.real.cos(),
            grad: -&self.grad * self.real.sin(),
            hess: -&self.hess * self.real.sin() - hess_left * self.real.cos()
        }
    }

    pub fn powi(&self, n:i32) -> HyperDual {
        let m = f64::from(n);
        let mut hess_left = Array::zeros(self.hess.raw_dim());
        hess_left += &self.grad;
        {
            let mut hess_left_t = hess_left.view_mut().reversed_axes();
            hess_left_t *= &self.grad;
        }
        HyperDual {
            real: self.real.powi(n),
            grad: &self.grad * m * self.real.powi(n-1),
            hess: (m * self.real.powi(n-2))
                    * (self.real * &self.hess + (m-1.0) * hess_left)
        }
    }

}
