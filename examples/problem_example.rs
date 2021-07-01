use optimization::number_system::DualScalar;
use optimization::problem::{Gradient, Objective};
use ndarray::{Array1, Array, arr1};


// Objective function. In this case it only contains a vector of DualScalars
// representing the variables. Note that it could also contain a vector of
// error functions in a least-squares problem.
pub struct TestProblem {
    x: Vec<DualScalar>,
    value: DualScalar,
}


// Implementation of the Objective trait. The method `eval` is the method that
// actually evaluates the objective function. The methods `eval_real`, `grad`,
// and `diff` will call `eval` in order to evaluate the function and then pick
// the real or dual value of the result.
impl Objective for TestProblem {
    fn eval(&mut self) {
        let x1 = &self.x[0];
        let x2 = &self.x[1];
        let x3 = &self.x[2];

        let u1 = x1.cos()*x2.sin() - 0.05;
        let u2 = x2.sin() - 0.2;
        let u3 = x3.powi(2) - 2.56;

        self.value = u1.powi(2) + u2.powi(2) + u3.powi(2)
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
impl Gradient for TestProblem {
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


// Here is an example implementation of the TestProblem.
fn main() {
    let x0 = arr1(&[1.0, 1.0, 1.0]);
    let mut output: Array1<f64> = Array::zeros(x0.raw_dim());

    let a = DualScalar{re:1.0, du: 0.0};
    let b = DualScalar{re:1.0, du: 0.0};
    let c = DualScalar{re:1.0, du: 0.0};

    let mut x = Vec::new();
    x.push(a);
    x.push(b);
    x.push(c);

    let mut problem = TestProblem{x, value: DualScalar::new()};
    let val = problem.eval_real();
    problem.grad(&mut output);
    println!("current position: {}", x0);
    println!("objective function value: {}", val);
    println!("gradient of objective function is: {}", output);
}
