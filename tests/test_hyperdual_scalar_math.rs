use optimization::number_system::HyperDualScalar as HDual;
use approx::assert_abs_diff_eq;


#[test]
fn test_sin() {
    let x = HDual{real: 2.0, grad: 3.0, hess: 2.0};
    let test_dual = x.sin();
    let real = 2.0_f64.sin();
    let grad = 3.0_f64 * 2.0_f64.cos();
    let hess = -2.0_f64.sin()*9.0 + 2.0_f64.cos()*2.0;

    assert_abs_diff_eq!(test_dual.real, real);
    assert_abs_diff_eq!(test_dual.grad, grad);
    assert_abs_diff_eq!(test_dual.hess, hess);
}

#[test]
fn test_cos() {
    let x = HDual{real: 2.0, grad: 3.0, hess: 2.0};
    let test_dual = x.cos();
    let real = 2.0_f64.cos();
    let grad = -3.0_f64 * 2.0_f64.sin();
    let hess = -2.0_f64.cos()*9.0 - 2.0_f64.sin()*2.0;

    assert_abs_diff_eq!(test_dual.real, real);
    assert_abs_diff_eq!(test_dual.grad, grad);
    assert_abs_diff_eq!(test_dual.hess, hess);
}

#[test]
fn test_powi() {
    let x = HDual{real: 2.0, grad: 3.0, hess: 2.0};
    let test_dual = x.powi(3);
    let real = 2.0_f64.powi(3);
    let grad = 3.0_f64 * 2.0_f64.powi(2) * 3.0_f64;
    let hess = 6.0_f64*2.0_f64*3.0_f64*3.0_f64 + 3.0_f64*2.0_f64.powi(2)*2.0_f64;

    assert_abs_diff_eq!(test_dual.real, real);
    assert_abs_diff_eq!(test_dual.grad, grad);
    assert_abs_diff_eq!(test_dual.hess, hess);
}
