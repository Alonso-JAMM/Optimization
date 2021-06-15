use optimization::number_system::HyperDualScalar as HDual;
use approx::assert_abs_diff_eq;


#[test]
fn test_sin() {
    let x = HDual{re: 2.0, e1: 3.0, e2: 3.0, e1e2: 2.0};
    let test_dual = x.sin();
    let re = 2.0_f64.sin();
    let e1 = 3.0_f64 * 2.0_f64.cos();
    let e1e2 = -2.0_f64.sin()*9.0 + 2.0_f64.cos()*2.0;

    assert_abs_diff_eq!(test_dual.re, re);
    assert_abs_diff_eq!(test_dual.e1, e1);
    assert_abs_diff_eq!(test_dual.e2, e1);
    assert_abs_diff_eq!(test_dual.e1e2, e1e2);
}

#[test]
fn test_cos() {
    let x = HDual{re: 2.0, e1: 3.0, e2: 3.0, e1e2: 2.0};
    let test_dual = x.cos();
    let re = 2.0_f64.cos();
    let e1 = -3.0_f64 * 2.0_f64.sin();
    let e1e2 = -2.0_f64.cos()*9.0 - 2.0_f64.sin()*2.0;

    assert_abs_diff_eq!(test_dual.re, re);
    assert_abs_diff_eq!(test_dual.e1, e1);
    assert_abs_diff_eq!(test_dual.e2, e1);
    assert_abs_diff_eq!(test_dual.e1e2, e1e2);
}

#[test]
fn test_powi() {
    let x = HDual{re: 2.0, e1: 3.0, e2: 3.0, e1e2: 2.0};
    let test_dual = x.powi(3);
    let re = 2.0_f64.powi(3);
    let e1 = 3.0_f64 * 2.0_f64.powi(2) * 3.0_f64;
    let e1e2 = 6.0_f64*2.0_f64*3.0_f64*3.0_f64 + 3.0_f64*2.0_f64.powi(2)*2.0_f64;

    assert_abs_diff_eq!(test_dual.re, re);
    assert_abs_diff_eq!(test_dual.e1, e1);
    assert_abs_diff_eq!(test_dual.e2, e1);
    assert_abs_diff_eq!(test_dual.e1e2, e1e2);
}
