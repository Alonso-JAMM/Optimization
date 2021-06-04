/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use ndarray::{Array2, Array1, Array};
use crate::number_system::Dual;
use crate::Solution;
use crate::LineSearch;
use crate::StepValues;
use std::cell::Cell;


pub struct BFGS {
    // max number of iterations
    pub i_max: u32,

    // Objective function
    pub func: fn(&Array1<f64>) -> Dual,

    // gradient tolerance
    pub gtol: f64,

    // current step direction
    p_k: Array1<f64>,

    // current position
    x_k: Array1<f64>,

    // current function evaluation
    f_k: Dual,

    // last function evaluation
    f_k_old: Dual,

    // holder for function calls
    f_calls: Cell<u32>,
}


impl BFGS {
    pub fn new(f: fn(&Array1<f64>) -> Dual) -> BFGS {
        BFGS {
            i_max: 20,
            func: f,
            gtol: 1e-6,
            p_k: Array1::zeros(1),
            x_k: Array1::zeros(1),
            f_k: Dual::new(1),
            f_k_old: Dual::new(1),
            f_calls: Cell::new(0),
        }
    }

    #[allow(non_snake_case)]
    pub fn minimize(&mut self, x0: &Array1<f64>) -> Solution {
        self.set_up_parameters(x0);
        let mut line_search = LineSearch::new(self.func);

        let mut alpha_1: f64;
        let mut step: StepValues;
        let mut alpha_k: f64;
        let mut x_k_new: Array1<f64>;

        let I: Array2<f64> = Array::eye(x0.len());
        let mut rho: f64;
        let mut s_k: Array1<f64>;
        let mut y_k: Array1<f64>;
        let mut sk_yk: Array2<f64> = Array::zeros(I.dim());
        let mut yk_sk: Array2<f64> = Array::zeros(I.dim());
        let mut sk_sk: Array2<f64> = Array::zeros(I.dim());
        let mut H_k: Array2<f64> = Array::eye(x0.dim());

        let mut solution = Solution {
            x: Array::zeros(self.x_k.raw_dim()),
            success: false,
            iter_num: 0,
            func_evals: 0,
        };

        let mut k: u32 = 1;
        while k < self.i_max {
            if self.f_k.du.dot(&self.f_k.du).sqrt() < self.gtol {
                solution.success = true;
                break;
            }

            sk_sk.fill(0.0);
            sk_yk.fill(0.0);

            alpha_1 = 1.0;
            step = StepValues{x_k: &self.x_k, f_k: &self.f_k, p_k: &self.p_k, alpha_1};
            alpha_k = line_search.find_alpha(step);
            x_k_new = &self.x_k + alpha_k*&self.p_k;
            self.f_k_old.re = self.f_k.re;
            self.f_k_old.du.assign(&self.f_k.du);
            self.f_k = self.eval_func(&x_k_new);
            s_k = alpha_k * &self.p_k;
            y_k = &self.f_k.du - &self.f_k_old.du;

            rho = 1.0/(y_k.dot(&s_k));
            sk_yk += &y_k;
            {
                let mut sk_yk_t = sk_yk.view_mut().reversed_axes();
                sk_yk_t *= &s_k;
            }
            yk_sk.assign(&sk_yk.t());

            sk_sk += &s_k;
            {
                let mut sk_sk_t = sk_sk.view_mut().reversed_axes();
                sk_sk_t *= &s_k;
            }

            H_k = (&I - rho*&sk_yk).dot(&H_k).dot(&(&I - rho*&yk_sk)) + rho*&sk_sk;

            self.x_k.assign(&x_k_new);
            self.p_k = -H_k.dot(&self.f_k.du);
//             println!("k={}, p_k={}, x_k={}, alpha_k={}", k, self.p_k, self.x_k, alpha_k);

            k += 1;
        }

        solution.x.assign(&self.x_k);
        solution.iter_num = k;
        solution.func_evals = self.f_calls.get() + line_search.func_calls.get();
        solution
    }

    fn set_up_parameters(&mut self, x0: &Array1<f64>) {
        let mut new_x_k = Array::zeros(x0.raw_dim());
        new_x_k.assign(x0);
        self.x_k = new_x_k;
        self.f_k = self.eval_func(&self.x_k);
        self.f_k_old = Dual::new(x0.len());
        self.f_k_old.re = 1.0;
        self.p_k = -&self.f_k.du;
    }

    fn eval_func(&self, x: &Array1<f64>) -> Dual {
        self.f_calls.set(self.f_calls.get() + 1);
        (self.func)(x)
    }
}
