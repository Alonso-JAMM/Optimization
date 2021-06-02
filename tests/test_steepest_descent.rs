use optimization::SteepestDescent;
use optimization::number_system::Dual;
use ndarray::{Array1, arr1};
use approx::assert_abs_diff_eq;


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


#[test]
fn test_steepest_descent() {
    let x0 = arr1::<f64>(&[1.0, 1.0, 1.0]);
    let mut min = SteepestDescent::new(obj_func);
    let sol = min.minimize(&x0);

    assert!(sol.success);
    // Make sure the solution are close to the "real solutions" until 6
    // decimal places
    assert_abs_diff_eq!(sol.x[0], 1.318127, epsilon = 0.000001);
    assert_abs_diff_eq!(sol.x[1], 0.201358, epsilon = 0.000001);
    assert_abs_diff_eq!(sol.x[2], 1.600000, epsilon = 0.000001);
}
