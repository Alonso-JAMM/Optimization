use optimization::number_system::HyperDual;
use approx::assert_abs_diff_eq;
use ndarray::{Array2, Array, arr1};


#[test]
fn test_sin() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess: hess1};
    let hdual2 = hdual1.sin();
    let real = 2.0_f64.sin();
    let grad1 = 3.0_f64 * 2.0_f64.cos();
    let grad2 = 4.0_f64 * 2.0_f64.cos();
    let grad3 = 5.0_f64 * 2.0_f64.cos();
    let hess00 = -9.0_f64 * 2.0_f64.sin();
    let hess01 = -12.0_f64 * 2.0_f64.sin();
    let hess02 = -15.0_f64 * 2.0_f64.sin();
    let hess10 = -12.0_f64 * 2.0_f64.sin();
    let hess11 = -16.0_f64 * 2.0_f64.sin();
    let hess12 = -20.0_f64 * 2.0_f64.sin();
    let hess20 = -15.0_f64 * 2.0_f64.sin();
    let hess21 = -20.0_f64 * 2.0_f64.sin();
    let hess22 = -25.0_f64 * 2.0_f64.sin();

    assert_abs_diff_eq!(hdual2.real, real);
    assert_abs_diff_eq!(hdual2.grad[0], grad1);
    assert_abs_diff_eq!(hdual2.grad[1], grad2);
    assert_abs_diff_eq!(hdual2.grad[2], grad3);
    assert_abs_diff_eq!(hdual2.hess[[0,0]], hess00);
    assert_abs_diff_eq!(hdual2.hess[[0,1]], hess01);
    assert_abs_diff_eq!(hdual2.hess[[0,2]], hess02);
    assert_abs_diff_eq!(hdual2.hess[[1,0]], hess10);
    assert_abs_diff_eq!(hdual2.hess[[1,1]], hess11);
    assert_abs_diff_eq!(hdual2.hess[[1,2]], hess12);
    assert_abs_diff_eq!(hdual2.hess[[2,0]], hess20);
    assert_abs_diff_eq!(hdual2.hess[[2,1]], hess21);
    assert_abs_diff_eq!(hdual2.hess[[2,2]], hess22);
}

#[test]
fn test_cos() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess: hess1};
    let hdual2 = hdual1.cos();
    let real = 2.0_f64.cos();
    let grad1 = -3.0_f64 * 2.0_f64.sin();
    let grad2 = -4.0_f64 * 2.0_f64.sin();
    let grad3 = -5.0_f64 * 2.0_f64.sin();
    let hess00 = -9.0_f64 * 2.0_f64.cos();
    let hess01 = -12.0_f64 * 2.0_f64.cos();
    let hess02 = -15.0_f64 * 2.0_f64.cos();
    let hess10 = -12.0_f64 * 2.0_f64.cos();
    let hess11 = -16.0_f64 * 2.0_f64.cos();
    let hess12 = -20.0_f64 * 2.0_f64.cos();
    let hess20 = -15.0_f64 * 2.0_f64.cos();
    let hess21 = -20.0_f64 * 2.0_f64.cos();
    let hess22 = -25.0_f64 * 2.0_f64.cos();

    assert_abs_diff_eq!(hdual2.real, real);
    assert_abs_diff_eq!(hdual2.grad[0], grad1);
    assert_abs_diff_eq!(hdual2.grad[1], grad2);
    assert_abs_diff_eq!(hdual2.grad[2], grad3);
    assert_abs_diff_eq!(hdual2.hess[[0,0]], hess00);
    assert_abs_diff_eq!(hdual2.hess[[0,1]], hess01);
    assert_abs_diff_eq!(hdual2.hess[[0,2]], hess02);
    assert_abs_diff_eq!(hdual2.hess[[1,0]], hess10);
    assert_abs_diff_eq!(hdual2.hess[[1,1]], hess11);
    assert_abs_diff_eq!(hdual2.hess[[1,2]], hess12);
    assert_abs_diff_eq!(hdual2.hess[[2,0]], hess20);
    assert_abs_diff_eq!(hdual2.hess[[2,1]], hess21);
    assert_abs_diff_eq!(hdual2.hess[[2,2]], hess22);
}

#[test]
fn test_powi() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess: hess1};
    let hdual2 = hdual1.powi(3);
    let real = 2.0_f64.powi(3);
    let grad1 = 3.0_f64 * 3.0 * 2.0_f64.powi(2);
    let grad2 = 4.0_f64 * 3.0 * 2.0_f64.powi(2);
    let grad3 = 5.0_f64 * 3.0 * 2.0_f64.powi(2);
    let hess00 = 6.0_f64 * 2.0_f64 * 9.0;
    let hess01 = 6.0_f64 * 2.0_f64 * 12.0;
    let hess02 = 6.0_f64 * 2.0_f64 * 15.0;
    let hess10 = 6.0_f64 * 2.0_f64 * 12.0;
    let hess11 = 6.0_f64 * 2.0_f64 * 16.0;
    let hess12 = 6.0_f64 * 2.0_f64 * 20.0;
    let hess20 = 6.0_f64 * 2.0_f64 * 15.0;
    let hess21 = 6.0_f64 * 2.0_f64 * 20.0;
    let hess22 = 6.0_f64 * 2.0_f64 * 25.0;

    assert_abs_diff_eq!(hdual2.real, real);
    assert_abs_diff_eq!(hdual2.grad[0], grad1);
    assert_abs_diff_eq!(hdual2.grad[1], grad2);
    assert_abs_diff_eq!(hdual2.grad[2], grad3);
    assert_abs_diff_eq!(hdual2.hess[[0,0]], hess00);
    assert_abs_diff_eq!(hdual2.hess[[0,1]], hess01);
    assert_abs_diff_eq!(hdual2.hess[[0,2]], hess02);
    assert_abs_diff_eq!(hdual2.hess[[1,0]], hess10);
    assert_abs_diff_eq!(hdual2.hess[[1,1]], hess11);
    assert_abs_diff_eq!(hdual2.hess[[1,2]], hess12);
    assert_abs_diff_eq!(hdual2.hess[[2,0]], hess20);
    assert_abs_diff_eq!(hdual2.hess[[2,1]], hess21);
    assert_abs_diff_eq!(hdual2.hess[[2,2]], hess22);
}
