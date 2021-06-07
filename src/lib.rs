/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod number_system;


pub mod problem;


mod line_search;
use line_search::LineSearch;
use line_search::StepValues;

mod solution;
use solution::Solution;

mod steepest_descent;
pub use steepest_descent::SteepestDescent;

mod ncg;
pub use ncg::NCG;

mod bfgs;
pub use bfgs::BFGS;
