/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


use ndarray::Array1;


pub struct Solution {
    // solution of the optimization
    pub x: Array1<f64>,

    // whether the solver found a solution or not
    pub success: bool,

    // total number of iterations
    pub iter_num: u32,

    // number of function evaluations
    pub func_evals: u32,
}
