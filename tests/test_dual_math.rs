use optimization::number_system::Dual;
use approx::assert_abs_diff_eq;
use ndarray::arr1;


#[test]
fn test_sin() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = dual1.sin();
    let real = 2.0_f64.sin();
    let grad1 = 3.0_f64 * 2.0_f64.cos();
    let grad2 = 4.0_f64 * 2.0_f64.cos();
    let grad3 = 5.0_f64 * 2.0_f64.cos();

    assert_abs_diff_eq!(dual2.re, real);
    assert_abs_diff_eq!(dual2.du[0], grad1);
    assert_abs_diff_eq!(dual2.du[1], grad2);
    assert_abs_diff_eq!(dual2.du[2], grad3);
}


#[test]
fn test_cos() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = dual1.cos();
    let real = 2.0_f64.cos();
    let grad1 = -3.0_f64 * 2.0_f64.sin();
    let grad2 = -4.0_f64 * 2.0_f64.sin();
    let grad3 = -5.0_f64 * 2.0_f64.sin();

    assert_abs_diff_eq!(dual2.re, real);
    assert_abs_diff_eq!(dual2.du[0], grad1);
    assert_abs_diff_eq!(dual2.du[1], grad2);
    assert_abs_diff_eq!(dual2.du[2], grad3);
}

#[test]
fn test_powi() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = dual1.powi(3);
    let real = 2.0_f64.powi(3);
    let grad1 = 3.0_f64 * 3.0 * 2.0_f64.powi(2);
    let grad2 = 4.0_f64 * 3.0 * 2.0_f64.powi(2);
    let grad3 = 5.0_f64 * 3.0 * 2.0_f64.powi(2);

    assert_abs_diff_eq!(dual2.re, real);
    assert_abs_diff_eq!(dual2.du[0], grad1);
    assert_abs_diff_eq!(dual2.du[1], grad2);
    assert_abs_diff_eq!(dual2.du[2], grad3);
}
