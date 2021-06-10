/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use ndarray::{Array1, Array};
use crate::number_system::Dual;
use crate::Solution;
use crate::LineSearch;
use crate::StepValues;
use std::cell::Cell;
use crate::problem::{Objective, Gradient};


pub struct SteepestDescent {
    // max number of iterations
    pub i_max: u32,

    // gradient tolerance
    pub gtol: f64,

    // current step direction
    p_k: Array1<f64>,

    // current position
    x_k: Array1<f64>,

    // current function evaluation
    f_k: Dual,

    // last function evaluation (the gradient is not needed)
    f_k_old_re: f64,

    // holder for function calls
    f_calls: Cell<u32>,

    // holder for gradient calls
    f_grad_calls: Cell<u32>,
}


impl SteepestDescent {
    pub fn new() -> SteepestDescent {
        SteepestDescent {
            i_max: 1000,
            gtol: 1e-6,
            p_k: Array1::zeros(1),
            x_k: Array1::zeros(1),
            f_k: Dual::new(1),
            f_k_old_re: 0.0,
            f_calls: Cell::new(0),
            f_grad_calls: Cell::new(0),
        }
    }

    pub fn minimize<P>(&mut self, x0: &Array1<f64>, problem: &mut P) -> Solution
    where
        P: Objective + Gradient,
    {
        // start the procedure by setting up the system parameters
        self.set_up_parameters(x0, problem);
        let mut line_search = LineSearch::new();
        line_search.c2 = 0.1;

        let mut alpha_1: f64;
        let mut step: StepValues;
        let mut alpha_k: f64;

        let mut solution = Solution {
            x: Array::zeros(self.x_k.raw_dim()),
            success: false,
            iter_num: 0,
            f_evals: 0,
            f_grad_evals: 0,
        };

        let mut k: u32 = 1;
        while k < self.i_max {
            if self.f_k.du.dot(&self.f_k.du).sqrt() < self.gtol {
                solution.success = true;
                break;
            }
            alpha_1 = self.guess_alpha();

            step = StepValues{x_k: &self.x_k, f_k: &self.f_k, p_k: &self.p_k, alpha_1};
            alpha_k = line_search.find_alpha(step, problem);

            // calculate alpha*p_k
            self.p_k *= alpha_k;
            // then do the operation x_new = x_k + alpha*p_k
            self.x_k += &self.p_k;
            self.f_k_old_re = self.f_k.re;
            self.eval_func(problem);
            // update p_k to -f_k.du
            self.p_k.assign(&self.f_k.du);
            self.p_k *= -1.0;

//             println!("k={}, p_k={}, x_k={}, alpha_k={}", k, self.p_k, self.x_k, alpha_k);

            k += 1;
        }

        solution.x.assign(&self.x_k);
        solution.iter_num = k;
        // Note that grad evaluations counts both actual grad evaluations
        // (multivariate) and diff evaluations (univariate) which may not be
        // what one expects since diff may evaluate the object function only once
        // while grad will evaluate it multiple times
        solution.f_evals = self.f_calls.get() + line_search.f_calls.get();
        solution.f_grad_evals = self.f_grad_calls.get() + line_search.f_grad_calls.get();
        solution
    }

    fn set_up_parameters<P>(&mut self, x0: &Array1<f64>, problem: &mut P)
    where
        P: Objective + Gradient,
    {
        self.x_k = Array::zeros(x0.raw_dim());
        self.x_k.assign(x0);
        self.f_k = Dual::new(x0.len());
        self.p_k = Array::zeros(x0.raw_dim());
        self.eval_func(problem);
        self.f_k_old_re = 1.0;
        self.p_k = -&self.f_k.du;
    }

    fn guess_alpha(&self) -> f64 {
        let phi_0_grad = self.f_k.du.dot(&self.p_k);
        2.0*(self.f_k.re - self.f_k_old_re)/phi_0_grad
    }

    fn eval_func<P>(&mut self, problem: &mut P)
    where
        P: Objective + Gradient,
    {
        self.f_calls.set(self.f_calls.get() + 1);
        self.f_grad_calls.set(self.f_grad_calls.get() + 1);
        problem.update_x(&self.x_k);
        self.f_k.re = problem.eval_real();
        problem.grad(&mut self.f_k.du);
    }
}
