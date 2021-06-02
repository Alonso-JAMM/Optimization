/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use ndarray::{Array1, Array};
use crate::number_system::Dual;
use crate::Solution;
use crate::LineSearch;
use crate::StepValues;
use std::cell::Cell;


pub struct SteepestDescent {
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


impl SteepestDescent {
    pub fn new(f: fn(&Array1<f64>) -> Dual) -> SteepestDescent {
        SteepestDescent {
            i_max: 1000,
            func: f,
            gtol: 1e-6,
            p_k: Array1::zeros(1),
            x_k: Array1::zeros(1),
            f_k: Dual::new(1),
            f_k_old: Dual::new(1),
            f_calls: Cell::new(0),
        }
    }

    pub fn minimize(&mut self, x0: &Array1<f64>) -> Solution {
        // start the procedure by setting up the system parameters
        self.set_up_parameters(x0);
        let mut line_search = LineSearch::new(self.func);
        line_search.c2 = 0.1;

        let mut alpha_1: f64;
        let mut step: StepValues;
        let mut alpha_k: f64;
        let mut x_k_new: Array1<f64>;

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

            alpha_1 = self.guess_alpha();
            step = StepValues{x_k: &self.x_k, f_k: &self.f_k, p_k: &self.p_k, alpha_1};
            alpha_k = line_search.find_alpha(step);
            x_k_new = &self.x_k + alpha_k*&self.p_k;

            self.x_k.assign(&x_k_new);
            self.f_k_old.re = self.f_k.re;
            self.f_k_old.du.assign(&self.f_k.du);
            self.f_k = self.eval_func(&x_k_new);
            self.p_k = -&self.f_k.du;

//             println!("k={}, p_k={}, x_k={}, alpha_k={}", k, self.p_k, self.x_k, alpha_1);

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

    fn guess_alpha(&self) -> f64 {
        let phi_0_grad = self.f_k.du.dot(&self.p_k);
        2.0*(self.f_k.re - self.f_k_old.re)/phi_0_grad
    }

    fn eval_func(&self, x: &Array1<f64>) -> Dual {
        self.f_calls.set(self.f_calls.get() + 1);
        (self.func)(x)
    }
}
