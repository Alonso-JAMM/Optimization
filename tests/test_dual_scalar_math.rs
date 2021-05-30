use optimization::number_system::DualScalar;
use approx::assert_abs_diff_eq;


#[test]
fn test_sin() {
    let a = DualScalar{re: 2.0, du:3.0};
    let test_dual = a.sin();
    let real = 2.0_f64.sin();
    let dual = 3.0_f64 * 2.0_f64.cos();

    assert_abs_diff_eq!(test_dual.re, real);
    assert_abs_diff_eq!(test_dual.du, dual);
}


#[test]
fn test_cos() {
    let a = DualScalar{re: 2.0, du:3.0};
    let test_dual = a.cos();
    let real = 2.0_f64.cos();
    let dual = -3.0_f64 * 2.0_f64.sin();

    assert_abs_diff_eq!(test_dual.re, real);
    assert_abs_diff_eq!(test_dual.du, dual);
}


#[test]
fn test_powi() {
    let a = DualScalar{re: 2.0, du:3.0};
    let test_dual = a.powi(3);
    let real = 2.0_f64.powi(3);
    let dual = 3.0_f64 * 3.0 * 2.0_f64.powi(2);

    assert_abs_diff_eq!(test_dual.re, real);
    assert_abs_diff_eq!(test_dual.du, dual);
}
