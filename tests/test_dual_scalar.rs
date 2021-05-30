use optimization::number_system::dual_scalar::DualScalar;
use approx::assert_abs_diff_eq;


#[test]
fn new_values() {
    let new_dual = DualScalar::new();

    assert_abs_diff_eq!(new_dual.re, 0.0);
    assert_abs_diff_eq!(new_dual.du, 0.0);
}


#[test]
fn dual_add_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = DualScalar{re: 3.0, du: 2.0};
    let dual3 = dual1 + dual2;

    assert_abs_diff_eq!(dual3.re, 4.0);
    assert_abs_diff_eq!(dual3.du, 3.0);
}

#[test]
fn borrow_dual_add_borrow_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 7.0, du: 5.0};
    let dual3 = &dual1 + &dual2;

    assert_abs_diff_eq!(dual3.re, 9.0);
    assert_abs_diff_eq!(dual3.du, 8.0);
}

#[test]
fn dual_add_borrow_dual() {
    let dual1 = DualScalar{re: 4.0, du: 5.0};
    let dual2 = DualScalar{re: 1.0, du: 3.0};
    let dual3 = dual1 + &dual2;

    assert_abs_diff_eq!(dual3.re, 5.0);
    assert_abs_diff_eq!(dual3.du, 8.0);
}

#[test]
fn borrow_dual_add_dual() {
    let dual1 = DualScalar{re: 5.0, du: 6.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = &dual1 + dual2;

    assert_abs_diff_eq!(dual3.re, 9.0);
    assert_abs_diff_eq!(dual3.du, 11.0);
}

#[test]
fn dual_add_f64() {
    let dual1 = DualScalar{re: 7.0, du: 6.0};
    let dual2 = dual1 + 2.0;

    assert_abs_diff_eq!(dual2.re, 9.0);
    assert_abs_diff_eq!(dual2.du, 6.0);
}

#[test]
fn f64_add_dual() {
    let dual1 = DualScalar{re: 9.0, du: 8.0};
    let dual2 = 3.0 + dual1;

    assert_abs_diff_eq!(dual2.re, 12.0);
    assert_abs_diff_eq!(dual2.du, 8.0);
}

#[test]
fn borrow_dual_add_f64() {
    let dual1 = DualScalar{re: 7.0, du: 6.0};
    let dual2 = &dual1 + 2.0;

    assert_abs_diff_eq!(dual2.re, 9.0);
    assert_abs_diff_eq!(dual2.du, 6.0);
}

#[test]
fn f64_borrow_add_dual() {
    let dual1 = DualScalar{re: 9.0, du: 8.0};
    let dual2 = 3.0 + &dual1;

    assert_abs_diff_eq!(dual2.re, 12.0);
    assert_abs_diff_eq!(dual2.du, 8.0);
}


#[test]
fn dual_sub_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = DualScalar{re: 1.0, du: 2.0};
    let dual3 = dual1 - dual2;

    assert_abs_diff_eq!(dual3.re, 0.0);
    assert_abs_diff_eq!(dual3.du, -1.0);
}

#[test]
fn borrow_dual_sub_borrow_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = DualScalar{re: 1.0, du: 2.0};
    let dual3 = &dual1 - &dual2;

    assert_abs_diff_eq!(dual3.re, 0.0);
    assert_abs_diff_eq!(dual3.du, -1.0);
}

#[test]
fn dual_sub_borrow_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = DualScalar{re: 1.0, du: 2.0};
    let dual3 = dual1 - &dual2;

    assert_abs_diff_eq!(dual3.re, 0.0);
    assert_abs_diff_eq!(dual3.du, -1.0);
}

#[test]
fn borrow_dual_sub_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = DualScalar{re: 1.0, du: 2.0};
    let dual3 = &dual1 - dual2;

    assert_abs_diff_eq!(dual3.re, 0.0);
    assert_abs_diff_eq!(dual3.du, -1.0);
}

#[test]
fn dual_sub_f64() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = dual1 - 2.0;

    assert_abs_diff_eq!(dual2.re, -1.0);
    assert_abs_diff_eq!(dual2.du, 1.0);
}

#[test]
fn borrow_dual_sub_f64() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = &dual1 - 2.0;

    assert_abs_diff_eq!(dual2.re, -1.0);
    assert_abs_diff_eq!(dual2.du, 1.0);
}

#[test]
fn f64_sub_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = 2.0 - dual1;

    assert_abs_diff_eq!(dual2.re, 1.0);
    assert_abs_diff_eq!(dual2.du, -1.0);
}

#[test]
fn f64_sub_borrow_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = 2.0 - &dual1;

    assert_abs_diff_eq!(dual2.re, 1.0);
    assert_abs_diff_eq!(dual2.du, -1.0);
}


#[test]
fn neg_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = -dual1;

    assert_abs_diff_eq!(dual2.re, -1.0);
    assert_abs_diff_eq!(dual2.du, -1.0);
}

#[test]
fn neg_borrow_dual() {
    let dual1 = DualScalar{re: 1.0, du: 1.0};
    let dual2 = -&dual1;

    assert_abs_diff_eq!(dual2.re, -1.0);
    assert_abs_diff_eq!(dual2.du, -1.0);
}

#[test]
fn dual_mul_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = dual1 * dual2;

    assert_abs_diff_eq!(dual3.re, 8.0);
    assert_abs_diff_eq!(dual3.du, 22.0);
}

#[test]
fn borrow_dual_mul_borrow_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = &dual1 * &dual2;

    assert_abs_diff_eq!(dual3.re, 8.0);
    assert_abs_diff_eq!(dual3.du, 22.0);
}

#[test]
fn dual_mul_borrow_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = dual1 * &dual2;

    assert_abs_diff_eq!(dual3.re, 8.0);
    assert_abs_diff_eq!(dual3.du, 22.0);
}

#[test]
fn borrow_dual_mul_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = &dual1 * dual2;

    assert_abs_diff_eq!(dual3.re, 8.0);
    assert_abs_diff_eq!(dual3.du, 22.0);
}

#[test]
fn dual_mul_f64() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = dual1 * 3.0;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du, 9.0);
}

#[test]
fn borrow_dual_mul_f64() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = &dual1 * 3.0;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du, 9.0);
}

#[test]
fn f64_mul_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = 3.0 * dual1;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du, 9.0);
}

#[test]
fn f64_mul_borrow_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = 3.0 * &dual1;

    assert_abs_diff_eq!(dual2.re, 6.0);
    assert_abs_diff_eq!(dual2.du, 9.0);
}


#[test]
fn dual_div_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = dual1 / dual2;

    assert_abs_diff_eq!(dual3.re, 0.5);
    assert_abs_diff_eq!(dual3.du, 0.125);
}

#[test]
fn borrow_dual_div_borrow_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = &dual1 / &dual2;

    assert_abs_diff_eq!(dual3.re, 0.5);
    assert_abs_diff_eq!(dual3.du, 0.125);
}

#[test]
fn dual_div_borrow_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = dual1 / &dual2;

    assert_abs_diff_eq!(dual3.re, 0.5);
    assert_abs_diff_eq!(dual3.du, 0.125);
}

#[test]
fn borrow_dual_div_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = DualScalar{re: 4.0, du: 5.0};
    let dual3 = &dual1 / dual2;

    assert_abs_diff_eq!(dual3.re, 0.5);
    assert_abs_diff_eq!(dual3.du, 0.125);
}

#[test]
fn dual_div_f64() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = dual1 / 4.0;

    assert_abs_diff_eq!(dual2.re, 0.5);
    assert_abs_diff_eq!(dual2.du, 0.75);
}

#[test]
fn borrow_dual_div_f64() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = &dual1 / 4.0;

    assert_abs_diff_eq!(dual2.re, 0.5);
    assert_abs_diff_eq!(dual2.du, 0.75);
}

#[test]
fn f64_div_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = 6.0 / dual1;

    assert_abs_diff_eq!(dual2.re, 3.0);
    assert_abs_diff_eq!(dual2.du, 4.5);
}

#[test]
fn f64_div_borrow_dual() {
    let dual1 = DualScalar{re: 2.0, du: 3.0};
    let dual2 = 6.0 / &dual1;

    assert_abs_diff_eq!(dual2.re, 3.0);
    assert_abs_diff_eq!(dual2.du, 4.5);
}
