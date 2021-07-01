use ndarray::{arr1, Array1, Array2};
use optimization::number_system::HyperDualScalar as HDual;
use optimization::problem::{Objective, Gradient, Hessian};
use optimization::TrustNCG;


// Objective function. In this case it only contains a vector of HyperDualScalars
// representing the variables. Note that it could also contain a vector of
// error functions in a least-squares problem.
pub struct ProblemObjective {
    x: Vec<HDual>,
    value: HDual,
}


// Implementation of the Objective trait. The method `eval` is the method that
// actually evaluates the objective function. The methods `eval_re`, `grad`,
// and `diff` will call `eval` in order to evaluate the function and then pick
// the re or dual value of the result.
impl Objective for ProblemObjective {
    fn eval(&mut self) {
        let x1 = &self.x[0];
        let x2 = &self.x[1];
        let x3 = &self.x[2];

        let u1 = x1.cos()*x2.sin() - 0.05;
        let u2 = x2.sin() - 0.2;
        let u3 = x3.powi(2) - 2.56;

        self.value = u1.powi(2) + u2.powi(2) + u3.powi(2);
    }

    fn eval_real(&mut self) -> f64 {
        self.eval();
        self.value.re
    }

    fn update_x(&mut self, x: &Array1<f64>) {
        for i in 0..self.x.len() {
            self.x[i].re = x[i];
            self.x[i].e1 = 0.0; // make sure we are removing any dual part
        }
    }

    fn move_step(&mut self, x: &Array1<f64>, p: &Array1<f64>, alpha: f64) {
        let a = HDual{re: alpha, e1: 1.0, e2: 1.0, e1e2: 0.0};
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
            self.x[i].e1 = 1.0;
            self.eval();
            output[i] = self.value.e1;
            self.x[i].e1 = 0.0;
        }
    }

    fn diff(&mut self) -> f64 {
        self.eval();
        self.value.e1
    }
}


// Implementation of the hessian trait. This implementation tries to avoid
// performing redundant operations since the hessian is a symmetric matrix
impl Hessian for ProblemObjective {
    fn hess(&mut self, output: &mut Array2<f64>) {
        let mut eval: f64;
        for i in 0..self.x.len() {
            for j in i..self.x.len() {
                self.x[i].e1 = 1.0;
                self.x[j].e2 = 1.0;

                self.eval();
                eval = self.value.e1e2;
                output[[i,j]] = eval;
                output[[j,i]] = eval;

                self.x[i].e1 = 0.0;
                self.x[j].e2 = 0.0;
            }
        }
    }
}



fn main() {
    let x0 = arr1(&[1.0, 1.0, 1.0]);

    let a = HDual{re:1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let b = HDual{re:1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let c = HDual{re:1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let mut x = Vec::new();
    x.push(a);
    x.push(b);
    x.push(c);

    let mut problem = ProblemObjective{x, value: HDual::new()};
    let mut min = TrustNCG::new();

    let sol = min.minimize(&x0, &mut problem);

    println!("Solution succeeded?: {}, iterations: {}, function evaluations: {}, \
    gradient evaluations: {}", sol.success, sol.iter_num, sol.f_evals, sol.f_grad_evals);
    println!("solution x: {}", sol.x);
}
