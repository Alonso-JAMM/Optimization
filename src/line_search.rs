/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use crate::number_system::Dual;
use crate::number_system::DualScalar;
use crate::problem::{Objective, Gradient};
use ndarray::Array1;
use std::cell::Cell;


// Contains the values for each iteration step
pub struct StepValues<'a> {
    pub x_k: &'a Array1<f64>,
    pub f_k: &'a Dual,
    pub p_k: &'a Array1<f64>,
    pub alpha_1: f64,
}


pub struct LineSearch{
    pub c1: f64,
    pub c2: f64,
    pub i_max: u32,
    pub alpha_max: f64,
    pub f_calls: Cell<u32>,
    pub f_grad_calls: Cell<u32>,
    phi_0: DualScalar,
}


impl LineSearch {
    pub fn new() -> LineSearch {
        LineSearch {
            c1: 1e-4,
            c2: 0.9,
            i_max: 100,
            alpha_max: 1e3,
            f_calls: Cell::new(0),
            f_grad_calls: Cell::new(0),
            phi_0: DualScalar::new()
        }
    }

    pub fn find_alpha<P>(&mut self, step: StepValues, problem: &mut P) -> f64
    where
        P: Objective + Gradient,
    {
        self.set_phi_0(&step);
        self.line_search(&step, problem)
    }

    fn line_search<P>(&self, step: &StepValues, problem: &mut P) -> f64
    where
        P: Objective + Gradient
    {
        let mut alpha_im1 = 0.0;
        let mut phi_im1 = self.phi_0;

        let mut alpha_i = step.alpha_1;
        let mut phi_i = self.eval_phi(step, alpha_i, problem);

        let mut alpha_star = 1.0;

        let mut i: u32 = 1;
        while i < self.i_max {
//             println!("      i={}, alpha_i={}", i, alpha_i);

            if phi_i.re > self.phi_0.re + self.c1*alpha_i*self.phi_0.du
                    || (phi_i.re >= phi_im1.re && i > 1) {
                alpha_star = self.zoom(step, alpha_im1, phi_im1, alpha_i, phi_i, problem);
                break;
            }
            if phi_i.du.abs() <= -self.c2*self.phi_0.du {
                alpha_star = alpha_i;
                break;
            }
            if phi_i.du >= 0.0 {
                alpha_star = self.zoom(step, alpha_im1, phi_im1, alpha_i, phi_i, problem);
                break;
            }

            alpha_im1 = alpha_i;
            alpha_i = 2.0*alpha_i;
            phi_im1 = phi_i;
            phi_i = self.eval_phi(step, alpha_i, problem);
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

    fn zoom<P>(&self,
               step: &StepValues,
               mut alpha_lo: f64,
               mut phi_lo: DualScalar,
               mut alpha_hi: f64,
               mut phi_hi: DualScalar,
               problem: &mut P) -> f64
    where
        P: Objective + Gradient,
    {

        let mut alpha_j = self.cubic_interpolation(alpha_lo, &phi_lo, alpha_hi, &phi_hi);
        let mut phi_j = self.eval_phi(step, alpha_j, problem);

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
            // alpha_lo may be larger than alpha_hi which causes problemlem with
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
            phi_j = self.eval_phi(step, alpha_j, problem);

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


    fn eval_phi<P>(&self,
                   step: &StepValues,
                   alpha: f64,
                   problem: &mut P) -> DualScalar
    where
        P: Objective + Gradient,
    {
        // We don't need to evaluate the objective function when alpha is zero
        // since we already know this value (it is phi_0)
        if alpha == 0.0 {
            self.phi_0
        }
        else {
            let mut new_phi = DualScalar::new();
            let x_k = step.x_k;
            let p_k = step.p_k;

            self.f_calls.set(self.f_calls.get() + 1);
            self.f_grad_calls.set(self.f_grad_calls.get() + 1);
            problem.move_step(x_k, p_k, alpha);

            new_phi.re = problem.eval_real();
            new_phi.du = problem.diff();

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
    use crate::problem::{Objective, Gradient};


    // Objective function. In this case it only contains a vector of DualScalars
    // representing the variables. Note that it could also contain a vector of
    // error functions in a least-squares problem.
    pub struct ProblemObjective {
        x: Vec<DualScalar>,
        value: DualScalar,
    }


    // Implementation of the Objective trait. The method `eval` is the method that
    // actually evaluates the objective function. The methods `eval_real`, `grad`,
    // and `diff` will call `eval` in order to evaluate the function and then pick
    // the real or dual value of the result.
    impl Objective for ProblemObjective {
        fn eval(&mut self) {
            let x1 = &self.x[0];
            let x2 = &self.x[1];
            let x3 = &self.x[2];

            let u1 = x1 - 10.0;
            let u2 = x2 - 2.0;
            let u3 = x3 + 5.0;

            self.value = u1.powi(2) + u2.powi(2) + u3.powi(2);
        }

        fn eval_real(&mut self) -> f64 {
            self.eval();
            self.value.re
        }

        fn update_x(&mut self, x: &Array1<f64>) {
            for i in 0..self.x.len() {
                self.x[i].re = x[i];
                self.x[i].du = 0.0; // make sure we are removing any dual part
            }
        }

        fn move_step(&mut self, x: &Array1<f64>, p: &Array1<f64>, alpha: f64) {
            let a = DualScalar{re: alpha, du: 1.0};
            for i in 0..self.x.len() {
                self.x[i] = x[i] + a*p[i];
            }
        }
    }

    // Implementation of the gradient trait. Note that both `grad` and `diff` call
    // `eval` in order to obtain the corresponding derivatives.
    impl Gradient for ProblemObjective {
        fn grad(&mut self, output: &mut Array1<f64>) {
            for i in 0..self.x.len() {
                self.x[i].du = 1.0;
                self.eval();
                output[i] = self.value.du;
                self.x[i].du = 0.0;
            }
        }

        fn diff(&mut self) -> f64 {
            self.eval();
            self.value.du
        }
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
        let pk = arr1::<f64>(&[10.0, 2.0, -5.0]);
        let mut fk = Dual::new(3);
        let a = DualScalar{re:0.0, du: 0.0};
        let b = DualScalar{re:0.0, du: 0.0};
        let c = DualScalar{re:0.0, du: 0.0};
        let mut x = Vec::new();
        x.push(a);
        x.push(b);
        x.push(c);

        let mut problem = ProblemObjective{x, value: DualScalar::new()};


        fk.re = problem.eval_real();
        problem.grad(&mut fk.du);

        let mut line_search = LineSearch::new();
        let step = StepValues{x_k: &xk, f_k: &fk, p_k: &pk, alpha_1: 1.0};

        let alpha_star = line_search.find_alpha(step, &mut problem);

        let new_xk = &xk + alpha_star*&pk;
        let mut new_fk = Dual::new(3);
        problem.update_x(&new_xk);
        new_fk.re = problem.eval_real();
        problem.grad(&mut new_fk.du);

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
        let mut fk = Dual::new(3);
        let pk = arr1::<f64>(&[10.0, 2.0, -5.0]);

        let a = DualScalar{re:0.0, du: 0.0};
        let b = DualScalar{re:0.0, du: 0.0};
        let c = DualScalar{re:0.0, du: 0.0};
        let mut x = Vec::new();
        x.push(a);
        x.push(b);
        x.push(c);

        let mut problem = ProblemObjective{x, value: DualScalar::new()};

        fk.re = problem.eval_real();
        problem.grad(&mut fk.du);


        let mut line_search = LineSearch::new();

        let step = StepValues{x_k: &xk, f_k: &fk, p_k: &pk, alpha_1: 1.0};

        line_search.set_phi_0(&step);

        let phi_0 = line_search.phi_0;

        assert_abs_diff_eq!(phi_0.re, 129.0);
        assert_abs_diff_eq!(phi_0.du, -258.0);
    }


    #[test]
    fn test_eval_phi() {
        let xk = arr1::<f64>(&[0.0, 0.0, 0.0]);
        let mut fk = Dual::new(3);
        let pk = arr1::<f64>(&[10.0, 2.0, -5.0]);

        let a = DualScalar{re:0.0, du: 0.0};
        let b = DualScalar{re:0.0, du: 0.0};
        let c = DualScalar{re:0.0, du: 0.0};
        let mut x = Vec::new();
        x.push(a);
        x.push(b);
        x.push(c);

        let mut problem = ProblemObjective{x, value: DualScalar::new()};

        fk.re = problem.eval_real();
        problem.grad(&mut fk.du);

        let mut line_search = LineSearch::new();

        let step = StepValues{x_k: &xk, f_k: &fk, p_k: &pk, alpha_1: 1.0};
        line_search.set_phi_0(&step);

        // let's see if this function really gives phi_0 when alpha is 0.0
        let phi_0 = line_search.eval_phi(&step, 0.0, &mut problem);
        assert_abs_diff_eq!(phi_0.re, 129.0);
        assert_abs_diff_eq!(phi_0.du, -258.0);

        // when alpha is 1.0, we reach the minimum where the function value is
        // 0.0 and the gradient is 0.0
        let phi_1 = line_search.eval_phi(&step, 1.0, &mut problem);
        assert_abs_diff_eq!(phi_1.re, 0.0);
        assert_abs_diff_eq!(phi_1.du, 0.0);
    }


    #[test]
    fn test_cubic_interpolation() {
        let line_search = LineSearch::new();

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
