use criterion::{Criterion, criterion_group, criterion_main};
use optimization::BFGS;
use optimization::number_system::DualScalar;
use ndarray::{Array1, arr1};
use optimization::problem::{Objective, Gradient};


// Objective function. In this case it only contains a vector of DualScalars
// representing the variables. Note that it could also contain a vector of
// error functions in a least-squares problem.
pub struct ProblemObjective {
    x: Vec<DualScalar>,
}


// Implementation of the Objective trait. The method `eval` is the method that
// actually evaluates the objective function. The methods `eval_real`, `grad`,
// and `diff` will call `eval` in order to evaluate the function and then pick
// the real or dual value of the result.
impl Objective for ProblemObjective {
    type Output = DualScalar;
    fn eval(&self) -> DualScalar {
        let x1 = &self.x[0];
        let x2 = &self.x[1];
        let x3 = &self.x[2];

        let u1 = x1.cos()*x2.sin() - 0.05;
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
impl Gradient for ProblemObjective {
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


pub fn bfgs_benchmark(c: &mut Criterion) {
    c.bench_function("BFGS", |b| b.iter(|| {
        let a = DualScalar{re:1.0, du: 0.0};
        let b = DualScalar{re:1.0, du: 0.0};
        let d = DualScalar{re:1.0, du: 0.0};

        let mut x = Vec::new();
        x.push(a);
        x.push(b);
        x.push(d);

        let mut problem = ProblemObjective{x};
        let x0 = arr1::<f64>(&[1.0, 1.0, 1.0]);
        let mut solver = BFGS::new();
        let _sol = solver.minimize(&x0, &mut problem);
    }));

}

criterion_group!(benches, bfgs_benchmark);
criterion_main!(benches);

