/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


//! Traits to be used by the objective function.
//!
//! This collection of traits help to interface an arbitrary objective function
//! with the optimization algorithms used in this crate.
//!
//! # Example
//! ``` rust
//!use optimization::number_system::DualScalar;
//!use optimization::problem::{Gradient, Objective};
//!use ndarray::{Array1, Array, arr1};
//!
//!
//!// Objective function. In this case it only contains a vector of DualScalars
//!// representing the variables. Note that it could also contain a vector of
//!// error functions in a least-squares problem.
//!pub struct TestProblem {
//!    x: Vec<DualScalar>,
//!    value: DualScalar,
//!}
//!
//!
//!// Implementation of the Objective trait. The method `eval` is the method that
//!// actually evaluates the objective function. The methods `eval_real`, `grad`,
//!// and `diff` will call `eval` in order to evaluate the function and then pick
//!// the real or dual value of the result.
//!impl Objective for TestProblem {
//!    fn eval(&mut self) {
//!        let x1 = &self.x[0];
//!        let x2 = &self.x[1];
//!        let x3 = &self.x[2];
//!
//!        let u1 = x1.cos()*x2.cos() - 0.05;
//!        let u2 = x2.sin() - 0.2;
//!        let u3 = x3.powi(2) - 2.56;
//!
//!        self.value = u1.powi(2) + u2.powi(2) + u3.powi(2)
//!    }
//!
//!    fn eval_real(&mut self) -> f64 {
//!        self.eval();
//!        self.value.re
//!    }
//!
//!    fn update_x(&mut self, x: &Array1<f64>) {
//!        for i in 0..self.x.len() {
//!            self.x[i].re = x[i];
//!            self.x[i].du = 0.0; // make sure we are removing any dual part
//!        }
//!    }
//!
//!    fn move_step(&mut self, x: &Array1<f64>, p: &Array1<f64>, alpha: f64) {
//!        let a = DualScalar{re: alpha, du: 1.0};
//!        for i in 0..self.x.len() {
//!            self.x[i] = x[i] + a*p[i];
//!        }
//!    }
//!}
//!
//!// Implementation of the gradient trait. Note that both `grad` and `diff` call
//!// `eval` in order to obtain the corresponding derivatives.
//!impl Gradient for TestProblem {
//!    fn grad(&mut self, output: &mut Array1<f64>) {
//!        for i in 0..self.x.len() {
//!            self.x[i].du = 1.0;
//!            self.eval();
//!            output[i] = self.value.du;
//!            self.x[i].du = 0.0;
//!        }
//!    }
//!
//!    fn diff(&mut self) -> f64 {
//!        self.eval();
//!        self.value.du
//!    }
//!}
//!
//!
//!
//!// Here is an example implementation of the TestProblem.
//!fn main() {
//!    let x0 = arr1(&[1.0, 1.0, 1.0]);
//!    let mut output: Array1<f64> = Array::zeros(x0.raw_dim());
//!
//!    let a = DualScalar{re:1.0, du: 0.0};
//!    let b = DualScalar{re:1.0, du: 0.0};
//!    let c = DualScalar{re:1.0, du: 0.0};
//!
//!    let mut x = Vec::new();
//!    x.push(a);
//!    x.push(b);
//!    x.push(c);
//!
//!    let mut problem = TestProblem{x, value: DualScalar::new()};
//!    let val = problem.eval_real();
//!    problem.grad(&mut output);
//!    println!("current position: {}", x0);
//!    println!("objective function value: {}", val);
//!    println!("gradient of objective function is: {}", output);
//!}
//! ```


use ndarray::{Array1, Array2};

/// Objective function evaluation and update of variable values.
///
/// This trait is used as an interface of the objective function with the
/// minimization algorithms. Thus the algorithms can work for different
/// objective functions without much trouble.
pub trait Objective {
    /// The method used to evaluate the objective function. The intended use of
    /// this method is that the problem stores internally the objective function
    /// value, gradient, hessian, etc so that they can later be retrieved.
    /// assumes that the vector containing the values of the variables was
    /// already updated before being called.
    fn eval(&mut self);

    /// This method will retrieve the objective function value after being evaluated.
    /// The minimization algorithms will use this method internally since the
    /// internal type could be anything.
    fn eval_real(&mut self) -> f64;

    /// Method that updates the internal vector containing the variables of the
    /// objective function.
    fn update_x(&mut self, x: &Array1<f64>);

    /// Method that updates the internal vector containing the variables of the
    /// objective function by moving along some line. This method calculates
    /// `x + alpha*p` and puts the output of this calculation on the internal
    /// vector containing the variables.
    fn move_step(&mut self, x: &Array1<f64>, p: &Array1<f64>, alpha: f64);
}


/// Gradient of the objective function.
///
/// Any objective function that contains this trait should be able to calculate
/// its gradient. Many minimization algorithms need to calculate the gradient
/// of the objective function and this trait is the interface to do that.
pub trait Gradient {
    /// This method calculates the gradient of the objective function and writes
    /// the gradient value into `output` array.
    fn grad(&mut self, output: &mut Array1<f64>);

    /// This method calculates the 1D derivative of the objective function.
    /// This method is useful to get the derivative with respect of `alpha` when
    /// `x + alpha*p` is evaluated
    fn diff(&mut self) -> f64;
}


/// Hessian of the objective function.
///
/// Any objective function that contains this trait should be able to calculate
/// its hessian. The minimization algorithms that require a hessian will use this
/// trait in order to obtain the hessian of the objective function.
pub trait Hessian {
    /// Method that actually calculates the hessian matrix of the objective
    /// function.
    fn hess(&mut self, output: &mut Array2<f64>);
}
