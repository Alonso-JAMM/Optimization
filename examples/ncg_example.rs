use ndarray::{arr1, Array1};
use optimization::NCG;
use optimization::number_system::Dual;


fn obj_func(x: &Array1<f64>) -> Dual {
    let mut d1 = Dual::new(3);
    let mut d2 = Dual::new(3);
    let mut d3 = Dual::new(3);

    d1.du[0] = 1.0;
    d2.du[1] = 1.0;
    d3.du[2] = 1.0;

    d1.re = x[0];
    d2.re = x[1];
    d3.re = x[2];

    d1 = d1.cos()*d2.sin() - 0.05;
    d2 = d2.sin() - 0.2;
    d3 = d3.powi(2) - 2.56;

    let d4 = (d1).powi(2) + d2.powi(2) + d3.powi(2);

    d4
}

fn main() {
    let x0 = arr1::<f64>(&[1.0, 1.0, 1.0]);
    let mut min = NCG::new(obj_func);
    min.gtol = 1e-6;
    let sol = min.minimize(&x0);

    println!("Solution succeeded?: {}, iterations: {}, function evaluations: {}",
    sol.success, sol.iter_num, sol.func_evals);
    println!("solution x: {}", sol.x);
}
