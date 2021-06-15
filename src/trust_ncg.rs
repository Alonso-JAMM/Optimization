/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use ndarray::{Array1, Array2, Array};
use crate::number_system::HyperDual;
use crate::Solution;
use std::cell::Cell;
use crate::problem::{Objective, Gradient, Hessian};


#[allow(non_snake_case)]
pub struct TrustNCG {
    // max number of iterations
    pub i_max: u32,

    // max trust region radius
    pub delta_max: f64,

    // gradient tolerance
    pub gtol: f64,

    // current step direction
    p_k: Array1<f64>,

    // current position
    x_k: Array1<f64>,

    // holder for previous position
    x_k_old: Array1<f64>,

    // current function evaluation
    f_k: HyperDual,

    // last function evaluation
    f_k_old: HyperDual,

    // evaluation of the subproblem
    m_k: f64,

    // holder for function calls
    f_calls: Cell<u32>,

    // holder for gradient calls
    f_grad_calls: Cell<u32>,

    // holder for hessian calls
    f_hess_calls: Cell<u32>,
}


impl TrustNCG {
    pub fn new() -> TrustNCG {
        TrustNCG {
            i_max: 1000,
            delta_max: 100.0,
            gtol: 1e-6,
            p_k: Array1::zeros(1),
            x_k: Array1::zeros(1),
            x_k_old: Array1::zeros(1),
            f_k: HyperDual::new(1),
            m_k: 0.0,
            f_k_old: HyperDual::new(1),
            f_calls: Cell::new(0),
            f_grad_calls: Cell::new(0),
            f_hess_calls: Cell::new(0),
        }
    }

    pub fn minimize<P>(&mut self, x0: &Array1<f64>, problem: &mut P) -> Solution
    where
        P: Objective + Gradient + Hessian,
    {
        self.set_up_parameters(x0, problem);
        let mut cg_steihaug = CGSteihaug::new(x0);
        let mut delta = 1.0;
        let eta = 0.15;  // eta can be between 0 to 1/4
        let mut rho: f64;

        // old function evaluation in case we need to discard a new step
        let mut old_fk = HyperDual::new(x0.len());

        let mut solution = Solution {
            x: Array::zeros(self.x_k.raw_dim()),
            success: false,
            iter_num: 0,
            f_evals: 0,
            f_grad_evals: 0,
            f_hess_evals: 0,
        };

        let mut k: u32 = 1;
        while k < self.i_max {

            if self.f_k.grad.dot(&self.f_k.grad).sqrt() < self.gtol {
                solution.success = true;
                break;
            }
            cg_steihaug.solve_step(&self.f_k.grad, &self.f_k.hess, delta, &mut self.p_k);
            old_fk.real = self.f_k_old.real;
            old_fk.grad.assign(&self.f_k_old.grad);
            old_fk.hess.assign(&self.f_k_old.hess);
            self.f_k_old.real = self.f_k.real;
            self.f_k_old.grad.assign(&self.f_k.grad);
            self.f_k_old.hess.assign(&self.f_k.hess);
            // temporarily move to next step
            self.x_k += & self.p_k;
            self.eval_func(problem);
            rho = self.calculate_rho();

            if rho < 0.25 {
                delta = 0.25*(self.p_k.dot(&self.p_k).sqrt());
            }
            else {
                if rho > 0.75 && (self.p_k.dot(&self.p_k)).sqrt() == delta {
                    delta = f64::min(2.0*delta, self.delta_max);
                }
            }
            if rho < eta {
                // roll back to previous step
                self.x_k -= &self.p_k;
                // reset f_k values
                self.f_k.real = self.f_k_old.real;
                self.f_k.grad.assign(&self.f_k_old.grad);
                self.f_k.hess.assign(&self.f_k_old.hess);
                self.f_k_old.real = old_fk.real;
                self.f_k_old.grad.assign(&old_fk.grad);
                self.f_k_old.hess.assign(&old_fk.hess);
            }

//             println!("k={}, p_k={}, x_k={}, rho={}, delta={}", k, self.p_k, self.x_k, rho, delta);
            k += 1;
        }

        solution.x.assign(&self.x_k);
        solution.iter_num = k;
        // Note that grad evaluations counts both actual grad evaluations
        // (multivariate) and diff evaluations (univariate) which may not be
        // what one expects since diff may evaluate the object function only once
        // while grad will evaluate it multiple times
        solution.f_evals = self.f_calls.get();
        solution.f_grad_evals = self.f_grad_calls.get();
        solution.f_hess_evals = self.f_hess_calls.get();
        solution
    }

    fn calculate_rho(&mut self) -> f64{
        // note that m(0) = f
        self.m_k = self.f_k_old.real + self.f_k_old.grad.dot(&self.p_k)
                   + 0.5*self.p_k.dot(&self.f_k_old.hess.dot(&self.p_k));
        (self.f_k_old.real - self.f_k.real)/(self.f_k_old.real - self.m_k)
    }

    fn set_up_parameters<P>(&mut self, x0: &Array1<f64>, problem: &mut P)
    where
        P: Objective + Gradient + Hessian,
    {
        self.x_k = Array::zeros(x0.raw_dim());
        self.x_k.assign(x0);
        self.x_k_old = Array::zeros(x0.raw_dim());
        self.x_k_old.assign(x0);
        self.f_k = HyperDual::new(x0.len());
        self.eval_func(problem);
        self.p_k = -&self.f_k.grad;
        self.f_k_old = HyperDual::new(x0.len());
        self.f_k_old.real = 1.0;
    }

    fn eval_func<P>(&mut self, problem: &mut P)
    where
        P: Objective + Gradient + Hessian,
    {
        self.f_calls.set(self.f_calls.get() + 1);
        self.f_grad_calls.set(self.f_grad_calls.get() + 1);
        self.f_hess_calls.set(self.f_hess_calls.get() + 1);
        problem.update_x(&self.x_k);
        self.f_k.real = problem.eval_real();
        problem.grad(&mut self.f_k.grad);
        problem.hess(&mut self.f_k.hess)
    }
}




struct CGSteihaug {
    pub eps: f64,
    p: Array1<f64>,
    p_old: Array1<f64>,
    r: Array1<f64>,
    d: Array1<f64>,
    pub j_max: u32,
}

impl CGSteihaug {
    pub fn new(x: &Array1<f64>) -> CGSteihaug {
        CGSteihaug {
            eps: 1e-7,
            p: Array::zeros(x.raw_dim()),
            p_old: Array::zeros(x.raw_dim()),
            r: Array::zeros(x.raw_dim()),
            d: Array::zeros(x.raw_dim()),
            j_max: 50,
        }
    }

    #[allow(non_snake_case)]
    pub fn solve_step(&mut self, r_0: &Array1<f64>, B: &Array2<f64>, delta: f64, output: &mut Array1<f64>) {
        // using epsilon from line search algorithm (I don't know if it also works here)
        let r0_magnitude = (r_0.dot(r_0)).sqrt();
        self.eps = f64::min(0.5, r0_magnitude.sqrt())*r0_magnitude;
        self.r.assign(r_0);
        // set d_0 = - r_0
        self.d.assign(r_0);
        self.d *= -1.0;
        // make sure p_0 = 0
        self.p *= 0.0;
        self.p_old *= 0.0;

        let mut alpha: f64;
        let mut beta: f64;

        // placeholder for (dj^T)B(dj)
        let mut djBdj: f64;
        // placeholder for (rj^T)(rj)
        let mut rjrj: f64;
        // placeholder for (rj_new^T)(rj_new)
        let mut rjrj_new = self.r.dot(&self.r);


        if r_0.dot(r_0).sqrt() < self.eps {
            // return self.p
            output.assign(&self.p);
            return
        }

        let mut j: u32 = 0;
        while j < self.j_max {
            djBdj = self.d.dot(&B.dot(&self.d));


            if djBdj <= 0.0 {
                // Find tau
                // p = p + tau*d
                let a = (r_0.dot(r_0)).powi(3) / (delta*r_0.dot(&B.dot(r_0)));
                let tau = f64::min(a, 1.0);
                let tmp = &self.p + tau*&self.d;
                output.assign(&tmp);
                return
            }

            rjrj = rjrj_new;
            alpha = rjrj/djBdj;
            // p_new = p + alpha*dj
            self.p_old.assign(&self.p);
            self.p += &(alpha*&self.d);

            if self.p.dot(&self.p).sqrt() >= delta {
                // find tau
                // p = p_old + tau*d
                let a = self.d.dot(&self.d);
                let b = 2.0*self.p_old.dot(&self.d);
                let c = self.p_old.dot(&self.p_old) - delta.powi(2);
                let f = (b.powi(2) - 4.0*a*c).sqrt();
                let tau = (-b + f)/(2.0*a);
                if tau > 0.0 {
//                     p = p_old + tau*d
                        let tmp = &self.p_old + tau*&self.d;
                        output.assign(&tmp);
                }
                else {
//                     p = p_old - tau*d
                    let tmp = &self.p_old - tau*&self.d;
                    output.assign(&tmp);
                }
                return
            }

            self.r += &(alpha*B.dot(&self.d));

            if self.r.dot(&self.r).sqrt() < self.eps {
                // return p
                output.assign(&self.p);
                return
            }

            rjrj_new =  self.r.dot(&self.r);
            beta = rjrj_new/rjrj;
            // The newer version of the book shows that r needs to be negative
            self.d = -&self.r + beta*&self.d;

//             println!("  j={}, alpha={}, dBd={}, rjrj={}", j, alpha, djBdj, rjrj);

            j += 1;
        }
    }

}

impl CGSteihaug {


}

