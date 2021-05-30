use optimization::number_system::Dual;
use approx::assert_abs_diff_eq;
use ndarray::arr1;


#[test]
fn new_values() {
    let dual1 = Dual::new(3);

    assert_abs_diff_eq!(dual1.re, 0.0);
    assert_abs_diff_eq!(dual1.du[0], 0.0);
    assert_abs_diff_eq!(dual1.du[1], 0.0);
    assert_abs_diff_eq!(dual1.du[2], 0.0);
}


#[test]
fn dual_add_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = dual1 + dual2;

    assert_abs_diff_eq!(dual3.re, 5.0);
    assert_abs_diff_eq!(dual3.du[0], 9.0);
    assert_abs_diff_eq!(dual3.du[1], 11.0);
    assert_abs_diff_eq!(dual3.du[2], 13.0);
}

#[test]
fn borrow_dual_add_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = &dual1 + &dual2;

    assert_abs_diff_eq!(dual3.re, 5.0);
    assert_abs_diff_eq!(dual3.du[0], 9.0);
    assert_abs_diff_eq!(dual3.du[1], 11.0);
    assert_abs_diff_eq!(dual3.du[2], 13.0);
}

#[test]
fn dual_add_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = dual1 + &dual2;

    assert_abs_diff_eq!(dual3.re, 5.0);
    assert_abs_diff_eq!(dual3.du[0], 9.0);
    assert_abs_diff_eq!(dual3.du[1], 11.0);
    assert_abs_diff_eq!(dual3.du[2], 13.0);
}

#[test]
fn borrow_dual_add_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = &dual1 + dual2;

    assert_abs_diff_eq!(dual3.re, 5.0);
    assert_abs_diff_eq!(dual3.du[0], 9.0);
    assert_abs_diff_eq!(dual3.du[1], 11.0);
    assert_abs_diff_eq!(dual3.du[2], 13.0);
}

#[test]
fn dual_add_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = dual1 + 3.0;

    assert_abs_diff_eq!(dual2.re, 5.0);
    assert_abs_diff_eq!(dual2.du[0], 3.0);
    assert_abs_diff_eq!(dual2.du[1], 4.0);
    assert_abs_diff_eq!(dual2.du[2], 5.0);
}

#[test]
fn borrow_dual_add_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = &dual1 + 3.0;

    assert_abs_diff_eq!(dual2.re, 5.0);
    assert_abs_diff_eq!(dual2.du[0], 3.0);
    assert_abs_diff_eq!(dual2.du[1], 4.0);
    assert_abs_diff_eq!(dual2.du[2], 5.0);
}

#[test]
fn f64_add_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = 3.0 + dual1;

    assert_abs_diff_eq!(dual2.re, 5.0);
    assert_abs_diff_eq!(dual2.du[0], 3.0);
    assert_abs_diff_eq!(dual2.du[1], 4.0);
    assert_abs_diff_eq!(dual2.du[2], 5.0);
}

#[test]
fn f64_add_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = 3.0 + &dual1;

    assert_abs_diff_eq!(dual2.re, 5.0);
    assert_abs_diff_eq!(dual2.du[0], 3.0);
    assert_abs_diff_eq!(dual2.du[1], 4.0);
    assert_abs_diff_eq!(dual2.du[2], 5.0);
}


#[test]
fn dual_sub_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = dual1 - dual2;

    assert_abs_diff_eq!(dual3.re, -1.0);
    assert_abs_diff_eq!(dual3.du[0], -3.0);
    assert_abs_diff_eq!(dual3.du[1], -3.0);
    assert_abs_diff_eq!(dual3.du[2], -3.0);
}

#[test]
fn borrow_dual_sub_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = &dual1 - &dual2;

    assert_abs_diff_eq!(dual3.re, -1.0);
    assert_abs_diff_eq!(dual3.du[0], -3.0);
    assert_abs_diff_eq!(dual3.du[1], -3.0);
    assert_abs_diff_eq!(dual3.du[2], -3.0);
}

#[test]
fn dual_sub_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = dual1 - &dual2;

    assert_abs_diff_eq!(dual3.re, -1.0);
    assert_abs_diff_eq!(dual3.du[0], -3.0);
    assert_abs_diff_eq!(dual3.du[1], -3.0);
    assert_abs_diff_eq!(dual3.du[2], -3.0);
}

#[test]
fn borrow_dual_sub_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = &dual1 - dual2;

    assert_abs_diff_eq!(dual3.re, -1.0);
    assert_abs_diff_eq!(dual3.du[0], -3.0);
    assert_abs_diff_eq!(dual3.du[1], -3.0);
    assert_abs_diff_eq!(dual3.du[2], -3.0);
}

#[test]
fn dual_sub_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = dual1 - 3.0;

    assert_abs_diff_eq!(dual2.re, -1.0);
    assert_abs_diff_eq!(dual2.du[0], 3.0);
    assert_abs_diff_eq!(dual2.du[1], 4.0);
    assert_abs_diff_eq!(dual2.du[2], 5.0);
}

#[test]
fn borrow_dual_sub_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = &dual1 - 3.0;

    assert_abs_diff_eq!(dual2.re, -1.0);
    assert_abs_diff_eq!(dual2.du[0], 3.0);
    assert_abs_diff_eq!(dual2.du[1], 4.0);
    assert_abs_diff_eq!(dual2.du[2], 5.0);
}

#[test]
fn f64_sub_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = 3.0 - dual1;

    assert_abs_diff_eq!(dual2.re, 1.0);
    assert_abs_diff_eq!(dual2.du[0], -3.0);
    assert_abs_diff_eq!(dual2.du[1], -4.0);
    assert_abs_diff_eq!(dual2.du[2], -5.0);
}

#[test]
fn f64_sub_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = 3.0 - &dual1;

    assert_abs_diff_eq!(dual2.re, 1.0);
    assert_abs_diff_eq!(dual2.du[0], -3.0);
    assert_abs_diff_eq!(dual2.du[1], -4.0);
    assert_abs_diff_eq!(dual2.du[2], -5.0);
}


#[test]
fn neg_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = -dual1;

    assert_abs_diff_eq!(dual2.re, -2.0);
    assert_abs_diff_eq!(dual2.du[0], -3.0);
    assert_abs_diff_eq!(dual2.du[1], -4.0);
    assert_abs_diff_eq!(dual2.du[2], -5.0);
}

#[test]
fn neg_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = -&dual1;

    assert_abs_diff_eq!(dual2.re, -2.0);
    assert_abs_diff_eq!(dual2.du[0], -3.0);
    assert_abs_diff_eq!(dual2.du[1], -4.0);
    assert_abs_diff_eq!(dual2.du[2], -5.0);
}


#[test]
fn dual_mul_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = dual1 * dual2;

    assert_abs_diff_eq!(dual3.re, 6.0);
    assert_abs_diff_eq!(dual3.du[0], 21.0);
    assert_abs_diff_eq!(dual3.du[1], 26.0);
    assert_abs_diff_eq!(dual3.du[2], 31.0);
}

#[test]
fn borrow_dual_mul_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = &dual1 * &dual2;

    assert_abs_diff_eq!(dual3.re, 6.0);
    assert_abs_diff_eq!(dual3.du[0], 21.0);
    assert_abs_diff_eq!(dual3.du[1], 26.0);
    assert_abs_diff_eq!(dual3.du[2], 31.0);
}

#[test]
fn dual_mul_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = dual1 * &dual2;

    assert_abs_diff_eq!(dual3.re, 6.0);
    assert_abs_diff_eq!(dual3.du[0], 21.0);
    assert_abs_diff_eq!(dual3.du[1], 26.0);
    assert_abs_diff_eq!(dual3.du[2], 31.0);
}

#[test]
fn borrow_dual_mul_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = Dual{re: 3.0, du: grad2};
    let dual3 = &dual1 * dual2;

    assert_abs_diff_eq!(dual3.re, 6.0);
    assert_abs_diff_eq!(dual3.du[0], 21.0);
    assert_abs_diff_eq!(dual3.du[1], 26.0);
    assert_abs_diff_eq!(dual3.du[2], 31.0);
}

#[test]
fn dual_mul_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = dual1 * 3.0;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du[0], 9.0);
    assert_abs_diff_eq!(dual2.du[1], 12.0);
    assert_abs_diff_eq!(dual2.du[2], 15.0);
}

#[test]
fn borrow_dual_mul_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = &dual1 * 3.0;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du[0], 9.0);
    assert_abs_diff_eq!(dual2.du[1], 12.0);
    assert_abs_diff_eq!(dual2.du[2], 15.0);
}

#[test]
fn f64_mul_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = 3.0 * dual1;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du[0], 9.0);
    assert_abs_diff_eq!(dual2.du[1], 12.0);
    assert_abs_diff_eq!(dual2.du[2], 15.0);
}

#[test]
fn f64_mul_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let dual1 = Dual{re: 2.0, du: grad1};
    let dual2 = 3.0 * &dual1;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du[0], 9.0);
    assert_abs_diff_eq!(dual2.du[1], 12.0);
    assert_abs_diff_eq!(dual2.du[2], 15.0);
}


#[test]
fn dual_div_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = Dual{re: 2.0, du: grad2};
    let dual3 = dual1 / dual2;

    assert_abs_diff_eq!(dual3.re, 2.0);
    assert_abs_diff_eq!(dual3.du[0], -4.5);
    assert_abs_diff_eq!(dual3.du[1], -5.0);
    assert_abs_diff_eq!(dual3.du[2], -5.5);
}

#[test]
fn borrow_dual_div_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = Dual{re: 2.0, du: grad2};
    let dual3 = &dual1 / &dual2;

    assert_abs_diff_eq!(dual3.re, 2.0);
    assert_abs_diff_eq!(dual3.du[0], -4.5);
    assert_abs_diff_eq!(dual3.du[1], -5.0);
    assert_abs_diff_eq!(dual3.du[2], -5.5);
}

#[test]
fn dual_div_borrow_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = Dual{re: 2.0, du: grad2};
    let dual3 = dual1 / &dual2;

    assert_abs_diff_eq!(dual3.re, 2.0);
    assert_abs_diff_eq!(dual3.du[0], -4.5);
    assert_abs_diff_eq!(dual3.du[1], -5.0);
    assert_abs_diff_eq!(dual3.du[2], -5.5);
}

#[test]
fn borrow_dual_div_dual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = Dual{re: 2.0, du: grad2};
    let dual3 = &dual1 / dual2;

    assert_abs_diff_eq!(dual3.re, 2.0);
    assert_abs_diff_eq!(dual3.du[0], -4.5);
    assert_abs_diff_eq!(dual3.du[1], -5.0);
    assert_abs_diff_eq!(dual3.du[2], -5.5);
}

#[test]
fn dual_div_f64() {
    let grad1 = arr1::<f64>(&[4.0, 5.0, 6.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = dual1 / 2.0;

    assert_abs_diff_eq!(dual2.re, 2.0);
    assert_abs_diff_eq!(dual2.du[0], 2.0);
    assert_abs_diff_eq!(dual2.du[1], 2.5);
    assert_abs_diff_eq!(dual2.du[2], 3.0);
}

#[test]
fn borrow_dual_div_f64() {
    let grad1 = arr1::<f64>(&[4.0, 5.0, 6.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = &dual1 / 2.0;

    assert_abs_diff_eq!(dual2.re, 2.0);
    assert_abs_diff_eq!(dual2.du[0], 2.0);
    assert_abs_diff_eq!(dual2.du[1], 2.5);
    assert_abs_diff_eq!(dual2.du[2], 3.0);
}

#[test]
fn f64_div_dual() {
    let grad1 = arr1::<f64>(&[4.0, 5.0, 6.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = 2.0 / dual1;

    assert_abs_diff_eq!(dual2.re, 0.5);
    assert_abs_diff_eq!(dual2.du[0], -0.5);
    assert_abs_diff_eq!(dual2.du[1], -0.625);
    assert_abs_diff_eq!(dual2.du[2], -0.75);
}

#[test]
fn f64_div_borrow_dual() {
    let grad1 = arr1::<f64>(&[4.0, 5.0, 6.0]);
    let dual1 = Dual{re: 4.0, du: grad1};
    let dual2 = 2.0 / &dual1;

    assert_abs_diff_eq!(dual2.re, 0.5);
    assert_abs_diff_eq!(dual2.du[0], -0.5);
    assert_abs_diff_eq!(dual2.du[1], -0.625);
    assert_abs_diff_eq!(dual2.du[2], -0.75);
}
