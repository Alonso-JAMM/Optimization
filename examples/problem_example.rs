use optimization::number_system::DualScalar;
use optimization::problem::{Gradient, Objective};
use ndarray::{Array1, Array, arr1};


// Objective function. In this case it only contains a vector of DualScalars
// representing the variables. Note that it could also contain a vector of
// error functions in a least-squares problem.
pub struct TestProblem {
    x: Vec<DualScalar>,
}


// Implementation of the Objectie trait. The method `eval` is the method that
// actually evaluates the objective function. The methods `eval_real`, `grad`,
// and `diff` will call `eval` in order to evaluate the function and then pick
// the real or dual value of the result.
impl Objective for TestProblem {
    type Output = DualScalar;
    fn eval(&self) -> DualScalar {
        let x1 = &self.x[0];
        let x2 = &self.x[1];
        let x3 = &self.x[2];

        let u1 = x1.cos()*x2.cos() - 0.05;
        let u2 = x2.sin() - 0.2;
        let u3 = x3.powi(2) - 2.56;

        u1.powi(2) + u2.powi(2) + u3.powi(2)
    }

    fn eval_real(&self) -> f64 {
        self.eval().re
    }

    fn update_x(&mut self, x: &Array1<f64>) {
        for i in 0..self.x.len() {
            self.x[i].re = x[i];
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
            output[i] = self.eval().du;
            self.x[i].du = 0.0;
        }
    }

    fn diff(&self) -> f64 {
        self.eval().du
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

    let mut problem = TestProblem{x};
    let val = problem.eval();
    problem.grad(&mut output);
    println!("current position: {}", x0);
    println!("objective function value: {}", val.re);
    println!("gradient of objective function is: {}", output);
}
