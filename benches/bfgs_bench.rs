use criterion::{Criterion, criterion_group, criterion_main};
use optimization::BFGS;
use optimization::number_system::Dual;
use ndarray::{Array1, arr1};


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


pub fn bfgs_benchmark(c: &mut Criterion) {
    c.bench_function("BFGS", |b| b.iter(|| {
        let x0 = arr1::<f64>(&[1.0, 1.0, 1.0]);
        let mut solver = BFGS::new(obj_func);
        let _sol = solver.minimize(&x0);
    }));

}

criterion_group!(benches, bfgs_benchmark);
criterion_main!(benches);

