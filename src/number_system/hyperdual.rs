/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use std::ops::{Add, Sub, Neg, Mul, Div};
use ndarray::{Array2, Array1, Array};


#[derive(Debug, Clone)]
pub struct HyperDual {
    pub real: f64,
    pub grad: Array1<f64>,
    pub hess: Array2<f64>,
}


impl HyperDual {
    pub fn new(n: usize) -> HyperDual {
        HyperDual {
            real: 0.0,
            grad: Array::zeros((n,)),
            hess: Array::zeros((n,n)),
        }
    }
}


impl Add<HyperDual> for HyperDual {
    type Output = HyperDual;
    fn add(self, other: HyperDual) -> HyperDual {
        HyperDual {
            real: self.real + other.real,
            grad: self.grad + other.grad,
            hess: self.hess + other.hess,
        }
    }
}

impl Add<&HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn add(self, other: &HyperDual) -> HyperDual {
        HyperDual {
            real: self.real + other.real,
            grad: &self.grad + &other.grad,
            hess: &self.hess + &other.hess,
        }
    }
}

impl Add<HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn add(self, other: HyperDual) -> HyperDual {
        HyperDual {
            real: self.real + other.real,
            grad: &self.grad + other.grad,
            hess: &self.hess + other.hess,
        }
    }
}

impl Add<&HyperDual> for HyperDual {
    type Output = HyperDual;
    fn add(self, other: &HyperDual) -> HyperDual {
        HyperDual {
            real: self.real + other.real,
            grad: self.grad + &other.grad,
            hess: self.hess + &other.hess,
        }
    }
}

impl Add<f64> for HyperDual {
    type Output = HyperDual;
    fn add(self, other: f64) -> HyperDual {
        let mut new_grad = Array::zeros(self.grad.raw_dim());
        new_grad.assign(&self.grad);
        let mut new_hess = Array::zeros(self.hess.raw_dim());
        new_hess.assign(&self.hess);
        HyperDual {
            real: self.real + other,
            grad: new_grad,
            hess: new_hess,
        }
    }
}

impl Add<f64> for &HyperDual {
    type Output = HyperDual;
    fn add(self, other: f64) -> HyperDual {
        let mut new_grad = Array::zeros(self.grad.raw_dim());
        new_grad.assign(&self.grad);
        let mut new_hess = Array::zeros(self.hess.raw_dim());
        new_hess.assign(&self.hess);
        HyperDual {
            real: self.real + other,
            grad: new_grad,
            hess: new_hess,
        }
    }
}

impl Add<HyperDual> for f64 {
    type Output = HyperDual;
    fn add(self, other: HyperDual) -> HyperDual {
        let mut new_grad = Array::zeros(other.grad.raw_dim());
        new_grad.assign(&other.grad);
        let mut new_hess = Array::zeros(other.hess.raw_dim());
        new_hess.assign(&other.hess);
        HyperDual {
            real: self + other.real,
            grad: new_grad,
            hess: new_hess,
        }
    }
}

impl Add<&HyperDual> for f64 {
    type Output = HyperDual;
    fn add(self, other: &HyperDual) -> HyperDual {
        let mut new_grad = Array::zeros(other.grad.raw_dim());
        new_grad.assign(&other.grad);
        let mut new_hess = Array::zeros(other.hess.raw_dim());
        new_hess.assign(&other.hess);
        HyperDual {
            real: self + other.real,
            grad: new_grad,
            hess: new_hess,
        }
    }
}


impl Sub<HyperDual> for HyperDual {
    type Output = HyperDual;
    fn sub(self, other: HyperDual) -> HyperDual {
        HyperDual {
            real: self.real - other.real,
            grad: self.grad - other.grad,
            hess: self.hess - other.hess,
        }
    }
}

impl Sub<&HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn sub(self, other: &HyperDual) -> HyperDual {
        HyperDual {
            real: self.real - other.real,
            grad: &self.grad - &other.grad,
            hess: &self.hess - &other.hess,
        }
    }
}

impl Sub<HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn sub(self, other: HyperDual) -> HyperDual {
        HyperDual {
            real: self.real - other.real,
            grad: &self.grad - other.grad,
            hess: &self.hess - other.hess,
        }
    }
}

impl Sub<&HyperDual> for HyperDual {
    type Output = HyperDual;
    fn sub(self, other: &HyperDual) -> HyperDual {
        HyperDual {
            real: self.real - other.real,
            grad: self.grad - &other.grad,
            hess: self.hess - &other.hess,
        }
    }
}

impl Sub<f64> for HyperDual {
    type Output = HyperDual;
    fn sub(self, other: f64) -> HyperDual {
        let mut new_grad = Array::zeros(self.grad.raw_dim());
        new_grad.assign(&self.grad);
        let mut new_hess = Array::zeros(self.hess.raw_dim());
        new_hess.assign(&self.hess);
        HyperDual {
            real: self.real - other,
            grad: new_grad,
            hess: new_hess,
        }
    }
}

impl Sub<f64> for &HyperDual {
    type Output = HyperDual;
    fn sub(self, other: f64) -> HyperDual {
        let mut new_grad = Array::zeros(self.grad.raw_dim());
        new_grad.assign(&self.grad);
        let mut new_hess = Array::zeros(self.hess.raw_dim());
        new_hess.assign(&self.hess);
        HyperDual {
            real: self.real - other,
            grad: new_grad,
            hess: new_hess,
        }
    }
}

impl Sub<HyperDual> for f64 {
    type Output = HyperDual;
    fn sub(self, other: HyperDual) -> HyperDual {
        let mut new_grad = Array::zeros(other.grad.raw_dim());
        new_grad.assign(&other.grad);
        let mut new_hess = Array::zeros(other.hess.raw_dim());
        new_hess.assign(&other.hess);
        HyperDual {
            real: self - other.real,
            grad: -new_grad,
            hess: -new_hess,
        }
    }
}

impl Sub<&HyperDual> for f64 {
    type Output = HyperDual;
    fn sub(self, other: &HyperDual) -> HyperDual {
        let mut new_grad = Array::zeros(other.grad.raw_dim());
        new_grad.assign(&other.grad);
        let mut new_hess = Array::zeros(other.hess.raw_dim());
        new_hess.assign(&other.hess);
        HyperDual {
            real: self - other.real,
            grad: -new_grad,
            hess: -new_hess,
        }
    }
}


impl Neg for HyperDual {
    type Output = HyperDual;
    fn neg(self) -> HyperDual {
        HyperDual {
            real: -self.real,
            grad: -self.grad,
            hess: -self.hess,
        }
    }
}

impl Neg for &HyperDual {
    type Output = HyperDual;
    fn neg(self) -> HyperDual {
        HyperDual {
            real: -self.real,
            grad: -&self.grad,
            hess: -&self.hess,
        }
    }
}


impl Mul<HyperDual> for HyperDual {
    type Output = HyperDual;
    fn mul(self, other: HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &self.grad;
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        HyperDual {
            real: self.real * other.real,
            grad: self.real * other.grad + other.real * self.grad,
            hess: self.real * other.hess
                 + &hess_middle
                 + hess_middle.t()
                 + other.real * self.hess,
        }
    }
}

impl Mul<&HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn mul(self, other: &HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &self.grad;
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        HyperDual {
            real: self.real * other.real,
            grad: self.real * &other.grad + other.real * &self.grad,
            hess: self.real * &other.hess
                 + &hess_middle
                 + hess_middle.t()
                 + other.real * &self.hess,
        }
    }
}

impl Mul<HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn mul(self, other: HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &self.grad;
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        HyperDual {
            real: self.real * other.real,
            grad: self.real * &other.grad + other.real * &self.grad,
            hess: self.real * &other.hess
                 + &hess_middle
                 + hess_middle.t()
                 + other.real * &self.hess,
        }
    }
}

impl Mul<&HyperDual> for HyperDual {
    type Output = HyperDual;
    fn mul(self, other: &HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &self.grad;
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        HyperDual {
            real: self.real * other.real,
            grad: self.real * &other.grad + other.real * &self.grad,
            hess: self.real * &other.hess
                 + &hess_middle
                 + hess_middle.t()
                 + other.real * &self.hess,
        }
    }
}

impl Mul<f64> for HyperDual {
    type Output = HyperDual;
    fn mul(self, other: f64) -> HyperDual {
        HyperDual {
            real: self.real * other,
            grad: other * self.grad,
            hess: other * self.hess,
        }
    }
}

impl Mul<f64> for &HyperDual {
    type Output = HyperDual;
    fn mul(self, other: f64) -> HyperDual {
        HyperDual {
            real: self.real * other,
            grad: other * &self.grad,
            hess: other * &self.hess,
        }
    }
}

impl Mul<HyperDual> for f64 {
    type Output = HyperDual;
    fn mul(self, other: HyperDual) -> HyperDual {
        HyperDual {
            real: self * other.real,
            grad: self * other.grad,
            hess: self * other.hess,
        }
    }
}

impl Mul<&HyperDual> for f64 {
    type Output = HyperDual;
    fn mul(self, other: &HyperDual) -> HyperDual {
        HyperDual {
            real: self * other.real,
            grad: self * &other.grad,
            hess: self * &other.hess,
        }
    }
}


impl Div<HyperDual> for HyperDual {
    type Output = HyperDual;
    fn div(self, other: HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &(&self.grad/other.real.powi(2));
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        let mut other_g_hess: Array2<f64> = Array::zeros(self.hess.raw_dim());
        other_g_hess += &(-2.0*&other.grad);
        {
            let mut other_g_hess_t = other_g_hess.view_mut().reversed_axes();
            other_g_hess_t *= &other.grad;
        }
        HyperDual {
            real: self.real / other.real,
            grad: self.grad / other.real - self.real * other.grad / other.real.powi(2),
            hess: self.hess / other.real
                 - &hess_middle
                 - hess_middle.t()
                 - self.real / other.real.powi(3)
                    * ( other.real* other.hess + other_g_hess)
        }
    }
}

impl Div<&HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn div(self, other: &HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &(&self.grad/other.real.powi(2));
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        let mut other_g_hess: Array2<f64> = Array::zeros(self.hess.raw_dim());
        other_g_hess += &(-2.0*&other.grad);
        {
            let mut other_g_hess_t = other_g_hess.view_mut().reversed_axes();
            other_g_hess_t *= &other.grad;
        }
        HyperDual {
            real: self.real / other.real,
            grad: &self.grad / other.real - self.real * &other.grad / other.real.powi(2),
            hess: &self.hess / other.real
                 - &hess_middle
                 - hess_middle.t()
                 - self.real / other.real.powi(3)
                    * ( other.real * &other.hess + other_g_hess)
        }
    }
}

impl Div<HyperDual> for &HyperDual {
    type Output = HyperDual;
    fn div(self, other: HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &(&self.grad/other.real.powi(2));
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        let mut other_g_hess: Array2<f64> = Array::zeros(self.hess.raw_dim());
        other_g_hess += &(-2.0*&other.grad);
        {
            let mut other_g_hess_t = other_g_hess.view_mut().reversed_axes();
            other_g_hess_t *= &other.grad;
        }
        HyperDual {
            real: self.real / other.real,
            grad: &self.grad / other.real - self.real * &other.grad / other.real.powi(2),
            hess: &self.hess / other.real
                 - &hess_middle
                 - hess_middle.t()
                 - self.real / other.real.powi(3)
                    * ( other.real * &other.hess + other_g_hess)
        }
    }
}

impl Div<&HyperDual> for HyperDual {
    type Output = HyperDual;
    fn div(self, other: &HyperDual) -> HyperDual {
        let mut hess_middle: Array2<f64> = Array::zeros(self.hess.raw_dim());
        hess_middle += &(&self.grad/other.real.powi(2));
        {
            let mut hess_middle_t = hess_middle.view_mut().reversed_axes();
            hess_middle_t *= &other.grad;
        }
        let mut other_g_hess: Array2<f64> = Array::zeros(self.hess.raw_dim());
        other_g_hess += &(-2.0*&other.grad);
        {
            let mut other_g_hess_t = other_g_hess.view_mut().reversed_axes();
            other_g_hess_t *= &other.grad;
        }
        HyperDual {
            real: self.real / other.real,
            grad: &self.grad / other.real - self.real * &other.grad / other.real.powi(2),
            hess: &self.hess / other.real
                 - &hess_middle
                 - hess_middle.t()
                 - self.real / other.real.powi(3)
                    * ( other.real * &other.hess + other_g_hess)
        }
    }
}

impl Div<f64> for HyperDual {
    type Output = HyperDual;
    fn div(self, other: f64) -> HyperDual {
        HyperDual {
            real: self.real / other,
            grad: self.grad / other,
            hess: self.hess / other
        }
    }
}

impl Div<f64> for &HyperDual {
    type Output = HyperDual;
    fn div(self, other: f64) -> HyperDual {
        HyperDual {
            real: self.real / other,
            grad: &self.grad / other,
            hess: &self.hess / other
        }
    }
}

impl Div<HyperDual> for f64 {
    type Output = HyperDual;
    fn div(self, other: HyperDual) -> HyperDual {
        let mut other_g_hess: Array2<f64> = Array::zeros(other.hess.raw_dim());
        other_g_hess += &(-2.0*&other.grad);
        {
            let mut other_g_hess_t = other_g_hess.view_mut().reversed_axes();
            other_g_hess_t *= &other.grad;
        }
        HyperDual {
            real: self / other.real,
            grad: -self * &other.grad / other.real.powi(2),
            hess: -self / other.real.powi(3)
                    * ( other.real * &other.hess + other_g_hess)
        }
    }
}

impl Div<&HyperDual> for f64 {
    type Output = HyperDual;
    fn div(self, other: &HyperDual) -> HyperDual {
        let mut other_g_hess: Array2<f64> = Array::zeros(other.hess.raw_dim());
        other_g_hess += &(-2.0*&other.grad);
        {
            let mut other_g_hess_t = other_g_hess.view_mut().reversed_axes();
            other_g_hess_t *= &other.grad;
        }
        HyperDual {
            real: self / other.real,
            grad: -self * &other.grad / other.real.powi(2),
            hess: -self / other.real.powi(3)
                    * ( other.real * &other.hess + other_g_hess)
        }
    }
}
