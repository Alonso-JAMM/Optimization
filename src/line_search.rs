/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use crate::number_system::Dual;
use crate::number_system::DualScalar;
use ndarray::Array1;


// Contains the values for each iteration step
pub struct StepValues<'a> {
    pub x_k: &'a Array1<f64>,
    pub f_k: &'a Dual,
    pub p_k: &'a Array1<f64>,
    pub alpha_1: f64,
}


pub struct LineSearch {
    pub c1: f64,
    pub c2: f64,
    pub i_max: u32,
    pub alpha_max: f64,
    f: fn(&Array1<f64>) -> Dual,
    phi_0: DualScalar,
}


impl LineSearch {
    pub fn new(func: fn(&Array1<f64>) -> Dual) -> LineSearch {
        LineSearch {
            c1: 1e-4,
            c2: 0.9,
            i_max: 100,
            alpha_max: 1e3,
            f: func,
            phi_0: DualScalar::new(),
        }
    }

    pub fn find_alpha(&mut self,
                  step: StepValues) -> f64 {
        self.set_phi_0(&step);
        self.line_search(&step)

    }

    fn line_search(&self,
                   step: &StepValues) -> f64 {

        let mut alpha_im1 = 0.0;
        let mut phi_im1 = self.phi_0;

        let mut alpha_i = step.alpha_1;
        let mut phi_i = self.eval_phi(step, alpha_i);

        let mut alpha_star = 1.0;

        let mut i: u32 = 1;
        while i < self.i_max {
//             println!("      i={}, alpha_i={}", i, alpha_i);

            if phi_i.re > self.phi_0.re + self.c1*alpha_i*self.phi_0.du
                    || (phi_i.re >= phi_im1.re && i > 1) {
                alpha_star = self.zoom(step, alpha_im1, phi_im1, alpha_i, phi_i);
                break;
            }
            if phi_i.du.abs() <= -self.c2*self.phi_0.du {
                alpha_star = alpha_i;
                break;
            }
            if phi_i.du >= 0.0 {
                alpha_star = self.zoom(step, alpha_im1, phi_im1, alpha_i, phi_i);
                break;
            }

            alpha_im1 = alpha_i;
            alpha_i = 2.0*alpha_i;
            phi_im1 = phi_i;
            phi_i = self.eval_phi(step, alpha_i);
            i += 1;

            // Now check that alpha_i doesn't get too close to maximum allowed
            // value
            if ((alpha_i - self.alpha_max)/self.alpha_max).abs() < 1e-2 {
                break;
            }
            i += 1;
        }
        alpha_star
    }

    fn zoom(&self,
            step: &StepValues,
            mut alpha_lo: f64,
            mut phi_lo: DualScalar,
            mut alpha_hi: f64,
            mut phi_hi: DualScalar) -> f64 {

        let mut alpha_j = self.cubic_interpolation(alpha_lo, &phi_lo, alpha_hi, &phi_hi);
        let mut phi_j = self.eval_phi(step, alpha_j);

        let mut alpha_star = 0.0;

        let mut j: u32 = 1;
        while j < self.i_max {
//             println!("          j={}, alpha_j={}", j, alpha_j);
            if phi_j.re > self.phi_0.re + self.c1*alpha_j*self.phi_0.du
                || phi_j.re >= phi_lo.re {
                alpha_hi = alpha_j;
                phi_hi = phi_j;
                }
            else {
                if phi_j.du.abs() <= -self.c2*self.phi_0.du {
                    alpha_star = alpha_j;
                    break;
                }
                if phi_j.du*(alpha_hi - alpha_lo) >= 0.0{
                    alpha_hi = alpha_lo;
                    phi_hi = phi_lo;
                }
                alpha_lo = alpha_j;
                phi_lo = phi_j;
            }

            // sometimes alpha_lo and alpha_hi may be "flipped," that is
            // alpha_lo may be larger than alpha_hi which causes problem with
            // the interpolation algorithm so we change their values.
            if alpha_lo > alpha_hi {
                let tmp = alpha_lo;
                alpha_lo = alpha_hi;
                alpha_hi = tmp;
                let tmp_phi = phi_lo;
                phi_lo = phi_hi;
                phi_hi = tmp_phi;
            }
            alpha_j = self.cubic_interpolation(alpha_lo, &phi_lo, alpha_hi, &phi_hi);
            phi_j = self.eval_phi(step, alpha_j);

            j += 1;
        }
        alpha_star
    }

    // Cubic interpolation method obtained from "Numerical Optimization" (3.43)
    fn cubic_interpolation(&self,
                           alpha_im1: f64,
                           phi_im1: &DualScalar,
                           alpha_i: f64,
                           phi_i: &DualScalar) -> f64{
        let d1 = phi_im1.du + phi_i.du
                 - 3.0*(phi_im1.re - phi_i.re)
                      /(alpha_im1 - alpha_i);
        let d2 = (d1.powi(2) - phi_im1.du*phi_i.du).sqrt();
        let mut new_alpha = alpha_i - (alpha_i - alpha_im1)*(phi_i.du + d2 - d1)
                            /(phi_i.du - phi_im1.du + 2.0*d2);
        if new_alpha > self.alpha_max {
            new_alpha = self.alpha_max;
        }
        new_alpha
    }


    fn eval_phi(&self, step: &StepValues, alpha: f64) -> DualScalar {

        // We don't need to evaluate the objective function when alpha is zero
        // since we already know this value (it is phi_0)
        if alpha == 0.0 {
            self.phi_0
        }
        else {
            let mut new_phi = DualScalar::new();
            let x_k = step.x_k;
            let p_k = step.p_k;

            let xk_apk = x_k + alpha*p_k;
            let eval = (self.f)(&xk_apk);

            new_phi.re = eval.re;
            new_phi.du = eval.du.dot(p_k);

            new_phi
        }
    }

    fn set_phi_0(&mut self, step: &StepValues) {
        let f_k = step.f_k;
        let p_k = step.p_k;

        self.phi_0.re = f_k.re;
        self.phi_0.du = f_k.du.dot(p_k);
    }
}


#[cfg(test)]
mod tests {
    use super::LineSearch;
    use super::Dual;
    use super::DualScalar;
    use ndarray::{Array1, arr1};
    use super::StepValues;
    use approx::assert_abs_diff_eq;


    fn obj_func(x: &Array1<f64>) -> Dual {
        let mut d1 = Dual::new(3);
        let mut d2 = Dual::new(3);
        let mut d3 = Dual::new(3);

        d1.du[0] = 1.0;
        d2.du[1] = 1.0;
        d3.du[2] = 1.0;

        d1.re = x[0];
        d2.re = x[1];
        d3.re = x[2];

        d1 = d1 - 10.0;
        d2 = d2 - 2.0;
        d3 = d3 + 5.0;

        let d4 = (d1).powi(2) + d2.powi(2) + d3.powi(2);

        d4
    }

    // Tests the overall behavior of the line search algorithm. The line search
    // algorithm is a 1D minimizer algorithm. The minimum point is at
    // [10, 2, -5]. Starting from [0, 0, 0] and with a step direction of
    // [10, 2, -5] then the factor of the step direction that gives a minimum
    // value for the function is "1". Thus, we check that the line search
    // algorithm finds a step direction factor of "1" and that it produces
    // a "0" for the objective function.
    #[test]
    fn test_line_search_algorithm() {
        let xk = arr1::<f64>(&[0.0, 0.0, 0.0]);
        let fk = obj_func(&xk);
        let pk = arr1::<f64>(&[10.0, 2.0, -5.0]);
        let mut line_search = LineSearch::new(obj_func);

        let step = StepValues{x_k: &xk, f_k: &fk, p_k: &pk, alpha_1: 1.0};

        let alpha_star = line_search.find_alpha(step);

        let new_xk = &xk + alpha_star*&pk;
        let new_fk = obj_func(&new_xk);

        assert_abs_diff_eq!(alpha_star, 1.0);
        assert_abs_diff_eq!(new_fk.re, 0.0);
    }

    // Under the specified initial position, step, and objective function
    // the value of the objective function is 129 and its gradient
    // value is [-20, -4, 10] so the dot product of the gradient and step
    // direction is -200 - 8 - 50 = -258
    #[test]
    fn test_set_phi0() {
        let xk = arr1::<f64>(&[0.0, 0.0, 0.0]);
        let fk = obj_func(&xk);
        let pk = arr1::<f64>(&[10.0, 2.0, -5.0]);
        let mut line_search = LineSearch::new(obj_func);

        let step = StepValues{x_k: &xk, f_k: &fk, p_k: &pk, alpha_1: 1.0};

        line_search.set_phi_0(&step);

        let phi_0 = line_search.phi_0;

        assert_abs_diff_eq!(phi_0.re, 129.0);
        assert_abs_diff_eq!(phi_0.du, -258.0);
    }


    #[test]
    fn test_eval_phi() {
        let xk = arr1::<f64>(&[0.0, 0.0, 0.0]);
        let fk = obj_func(&xk);
        let pk = arr1::<f64>(&[10.0, 2.0, -5.0]);
        let mut line_search = LineSearch::new(obj_func);

        let step = StepValues{x_k: &xk, f_k: &fk, p_k: &pk, alpha_1: 1.0};
        line_search.set_phi_0(&step);

        // let's see if this function really gives phi_0 when alpha is 0.0
        let phi_0 = line_search.eval_phi(&step, 0.0);
        assert_abs_diff_eq!(phi_0.re, 129.0);
        assert_abs_diff_eq!(phi_0.du, -258.0);

        // when alpha is 1.0, we reach the minimum where the function value is
        // 0.0 and the gradient is 0.0
        let phi_1 = line_search.eval_phi(&step, 1.0);
        assert_abs_diff_eq!(phi_1.re, 0.0);
        assert_abs_diff_eq!(phi_1.du, 0.0);
    }


    #[test]
    fn test_cubic_interpolation() {
        let xk = arr1::<f64>(&[0.0, 0.0, 0.0]);
        let line_search = LineSearch::new(obj_func);

        let alpha_im1 = 0.0;
        let phi_im1 = DualScalar{re: 1.0, du: -1.0};
        let alpha_i = 1.0;
        let phi_i = DualScalar{re: -1.0, du: 1.0};

        let new_alpha = line_search.cubic_interpolation(alpha_im1,
                                                        &phi_im1,
                                                        alpha_i,
                                                        &phi_i);
        // from testing the formula manually, the resulting alpha is
        // 0.9235635441915183
        assert_abs_diff_eq!(new_alpha, 0.9235635441915183);
    }

    // I don't include tests for line_search and zoom since they seem to give
    // good results from test test_line_search_algorithm.

}
