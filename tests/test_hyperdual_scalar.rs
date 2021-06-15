use optimization::number_system::HyperDualScalar as HDual;
use approx::assert_abs_diff_eq;


#[test]
fn new_values() {
    let hdual = HDual::new();

    assert_abs_diff_eq!(hdual.re, 0.0);
    assert_abs_diff_eq!(hdual.e1, 0.0);
    assert_abs_diff_eq!(hdual.e2, 0.0);
    assert_abs_diff_eq!(hdual.e1e2, 0.0);
}


#[test]
fn hdual_add_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = hdual1 + hdual2;

    assert_abs_diff_eq!(hdual3.re, 4.0);
    assert_abs_diff_eq!(hdual3.e1, 6.0);
    assert_abs_diff_eq!(hdual3.e2, 6.0);
    assert_abs_diff_eq!(hdual3.e1e2, 8.0);
}

#[test]
fn borrow_hdual_add_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = &hdual1 + &hdual2;

    assert_abs_diff_eq!(hdual3.re, 4.0);
    assert_abs_diff_eq!(hdual3.e1, 6.0);
    assert_abs_diff_eq!(hdual3.e2, 6.0);
    assert_abs_diff_eq!(hdual3.e1e2, 8.0);
}

#[test]
fn hdual_add_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = hdual1 + &hdual2;

    assert_abs_diff_eq!(hdual3.re, 4.0);
    assert_abs_diff_eq!(hdual3.e1, 6.0);
    assert_abs_diff_eq!(hdual3.e2, 6.0);
    assert_abs_diff_eq!(hdual3.e1e2, 8.0);
}

#[test]
fn borrow_hdual_add_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = &hdual1 + hdual2;

    assert_abs_diff_eq!(hdual3.re, 4.0);
    assert_abs_diff_eq!(hdual3.e1, 6.0);
    assert_abs_diff_eq!(hdual3.e2, 6.0);
    assert_abs_diff_eq!(hdual3.e1e2, 8.0);
}

#[test]
fn hdual_add_f64() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = hdual1 + 2.0;

    assert_abs_diff_eq!(hdual2.re, 3.0);
    assert_abs_diff_eq!(hdual2.e1, 2.0);
    assert_abs_diff_eq!(hdual2.e2, 2.0);
    assert_abs_diff_eq!(hdual2.e1e2, 3.0);
}

#[test]
fn borrow_hdual_add_f64() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = hdual1 + 2.0;

    assert_abs_diff_eq!(hdual2.re, 3.0);
    assert_abs_diff_eq!(hdual2.e1, 2.0);
    assert_abs_diff_eq!(hdual2.e2, 2.0);
    assert_abs_diff_eq!(hdual2.e1e2, 3.0);
}

#[test]
fn f64_add_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = 2.0 + hdual1;

    assert_abs_diff_eq!(hdual2.re, 3.0);
    assert_abs_diff_eq!(hdual2.e1, 2.0);
    assert_abs_diff_eq!(hdual2.e2, 2.0);
    assert_abs_diff_eq!(hdual2.e1e2, 3.0);
}

#[test]
fn f64_add_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = 2.0 + &hdual1;

    assert_abs_diff_eq!(hdual2.re, 3.0);
    assert_abs_diff_eq!(hdual2.e1, 2.0);
    assert_abs_diff_eq!(hdual2.e2, 2.0);
    assert_abs_diff_eq!(hdual2.e1e2, 3.0);
}



#[test]
fn hdual_sub_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = hdual1 - hdual2;

    assert_abs_diff_eq!(hdual3.re, -2.0);
    assert_abs_diff_eq!(hdual3.e1, -2.0);
    assert_abs_diff_eq!(hdual3.e2, -2.0);
    assert_abs_diff_eq!(hdual3.e1e2, -2.0);
}

#[test]
fn borrow_hdual_sub_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = &hdual1 - &hdual2;

    assert_abs_diff_eq!(hdual3.re, -2.0);
    assert_abs_diff_eq!(hdual3.e1, -2.0);
    assert_abs_diff_eq!(hdual3.e2, -2.0);
    assert_abs_diff_eq!(hdual3.e1e2, -2.0);
}

#[test]
fn hdual_sub_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = hdual1 - &hdual2;

    assert_abs_diff_eq!(hdual3.re, -2.0);
    assert_abs_diff_eq!(hdual3.e1, -2.0);
    assert_abs_diff_eq!(hdual3.e2, -2.0);
    assert_abs_diff_eq!(hdual3.e1e2, -2.0);
}

#[test]
fn borrow_hdual_sub_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = &hdual1 - hdual2;

    assert_abs_diff_eq!(hdual3.re, -2.0);
    assert_abs_diff_eq!(hdual3.e1, -2.0);
    assert_abs_diff_eq!(hdual3.e2, -2.0);
    assert_abs_diff_eq!(hdual3.e1e2, -2.0);
}

#[test]
fn hdual_sub_f64() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = hdual1 - 2.0;

    assert_abs_diff_eq!(hdual2.re, -1.0);
    assert_abs_diff_eq!(hdual2.e1, 2.0);
    assert_abs_diff_eq!(hdual2.e2, 2.0);
    assert_abs_diff_eq!(hdual2.e1e2, 3.0);
}

#[test]
fn borrow_hdual_sub_f64() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = &hdual1 - 2.0;

    assert_abs_diff_eq!(hdual2.re, -1.0);
    assert_abs_diff_eq!(hdual2.e1, 2.0);
    assert_abs_diff_eq!(hdual2.e2, 2.0);
    assert_abs_diff_eq!(hdual2.e1e2, 3.0);
}

#[test]
fn f64_sub_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = 2.0 - hdual1;

    assert_abs_diff_eq!(hdual2.re, 1.0);
    assert_abs_diff_eq!(hdual2.e1, -2.0);
    assert_abs_diff_eq!(hdual2.e2, -2.0);
    assert_abs_diff_eq!(hdual2.e1e2, -3.0);
}

#[test]
fn f64_sub_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = 2.0 - &hdual1;

    assert_abs_diff_eq!(hdual2.re, 1.0);
    assert_abs_diff_eq!(hdual2.e1, -2.0);
    assert_abs_diff_eq!(hdual2.e2, -2.0);
    assert_abs_diff_eq!(hdual2.e1e2, -3.0);
}


#[test]
fn neg_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = -hdual1;

    assert_abs_diff_eq!(hdual2.re, -1.0);
    assert_abs_diff_eq!(hdual2.e1, -2.0);
    assert_abs_diff_eq!(hdual2.e2, -2.0);
    assert_abs_diff_eq!(hdual2.e1e2, -3.0);
}

#[test]
fn neg_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = -&hdual1;

    assert_abs_diff_eq!(hdual2.re, -1.0);
    assert_abs_diff_eq!(hdual2.e1, -2.0);
    assert_abs_diff_eq!(hdual2.e2, -2.0);
    assert_abs_diff_eq!(hdual2.e1e2, -3.0);
}



#[test]
fn hdual_mul_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = hdual1 * hdual2;

    assert_abs_diff_eq!(hdual3.re, 3.0);
    assert_abs_diff_eq!(hdual3.e1, 10.0);
    assert_abs_diff_eq!(hdual3.e2, 10.0);
    assert_abs_diff_eq!(hdual3.e1e2, 30.0);
}

#[test]
fn borrow_hdual_mul_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = &hdual1 * &hdual2;

    assert_abs_diff_eq!(hdual3.re, 3.0);
    assert_abs_diff_eq!(hdual3.e1, 10.0);
    assert_abs_diff_eq!(hdual3.e2, 10.0);
    assert_abs_diff_eq!(hdual3.e1e2, 30.0);
}

#[test]
fn hdual_mul_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = hdual1 * &hdual2;

    assert_abs_diff_eq!(hdual3.re, 3.0);
    assert_abs_diff_eq!(hdual3.e1, 10.0);
    assert_abs_diff_eq!(hdual3.e2, 10.0);
    assert_abs_diff_eq!(hdual3.e1e2, 30.0);
}

#[test]
fn borrow_hdual_mul_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = HDual{re: 3.0, e1: 4.0, e2: 4.0, e1e2: 5.0};
    let hdual3 = &hdual1 * hdual2;

    assert_abs_diff_eq!(hdual3.re, 3.0);
    assert_abs_diff_eq!(hdual3.e1, 10.0);
    assert_abs_diff_eq!(hdual3.e2, 10.0);
    assert_abs_diff_eq!(hdual3.e1e2, 30.0);
}

#[test]
fn hdual_mul_f64() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = hdual1 * 2.0;

    assert_abs_diff_eq!(hdual2.re, 2.0);
    assert_abs_diff_eq!(hdual2.e1, 4.0);
    assert_abs_diff_eq!(hdual2.e2, 4.0);
    assert_abs_diff_eq!(hdual2.e1e2, 6.0);
}

#[test]
fn borrow_hdual_mul_f64() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = &hdual1 * 2.0;

    assert_abs_diff_eq!(hdual2.re, 2.0);
    assert_abs_diff_eq!(hdual2.e1, 4.0);
    assert_abs_diff_eq!(hdual2.e2, 4.0);
    assert_abs_diff_eq!(hdual2.e1e2, 6.0);
}

#[test]
fn f64_mul_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = 2.0 * hdual1;

    assert_abs_diff_eq!(hdual2.re, 2.0);
    assert_abs_diff_eq!(hdual2.e1, 4.0);
    assert_abs_diff_eq!(hdual2.e2, 4.0);
    assert_abs_diff_eq!(hdual2.e1e2, 6.0);
}

#[test]
fn f64_mul_borrow_hdual() {
    let hdual1 = HDual{re: 1.0, e1: 2.0, e2: 2.0, e1e2: 3.0};
    let hdual2 = 2.0 * &hdual1;

    assert_abs_diff_eq!(hdual2.re, 2.0);
    assert_abs_diff_eq!(hdual2.e1, 4.0);
    assert_abs_diff_eq!(hdual2.e2, 4.0);
    assert_abs_diff_eq!(hdual2.e1e2, 6.0);
}


#[test]
fn hdual_div_hdual() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = HDual{re: 4.0, e1: 3.0, e2: 3.0, e1e2: 5.0};
    let hdual3 = hdual1 / hdual2;

    assert_abs_diff_eq!(hdual3.re, 0.5);
    assert_abs_diff_eq!(hdual3.e1, 0.125);
    assert_abs_diff_eq!(hdual3.e2, 0.125);
    assert_abs_diff_eq!(hdual3.e1e2, 0.1875);
}

#[test]
fn borrow_hdual_div_borrow_hdual() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = HDual{re: 4.0, e1: 3.0, e2: 3.0, e1e2: 5.0};
    let hdual3 = &hdual1 / &hdual2;

    assert_abs_diff_eq!(hdual3.re, 0.5);
    assert_abs_diff_eq!(hdual3.e1, 0.125);
    assert_abs_diff_eq!(hdual3.e2, 0.125);
    assert_abs_diff_eq!(hdual3.e1e2, 0.1875);
}

#[test]
fn hdual_div_borrow_hdual() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = HDual{re: 4.0, e1: 3.0, e2: 3.0, e1e2: 5.0};
    let hdual3 = hdual1 / &hdual2;

    assert_abs_diff_eq!(hdual3.re, 0.5);
    assert_abs_diff_eq!(hdual3.e1, 0.125);
    assert_abs_diff_eq!(hdual3.e2, 0.125);
    assert_abs_diff_eq!(hdual3.e1e2, 0.1875);
}

#[test]
fn borrow_hdual_div_hdual() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = HDual{re: 4.0, e1: 3.0, e2: 3.0, e1e2: 5.0};
    let hdual3 = &hdual1 / hdual2;

    assert_abs_diff_eq!(hdual3.re, 0.5);
    assert_abs_diff_eq!(hdual3.e1, 0.125);
    assert_abs_diff_eq!(hdual3.e2, 0.125);
    assert_abs_diff_eq!(hdual3.e1e2, 0.1875);
}

#[test]
fn hdual_div_f64() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = hdual1 / 2.0;

    assert_abs_diff_eq!(hdual2.re, 1.0);
    assert_abs_diff_eq!(hdual2.e1, 1.0);
    assert_abs_diff_eq!(hdual2.e2, 1.0);
    assert_abs_diff_eq!(hdual2.e1e2, 2.0);
}

#[test]
fn borrow_hdual_div_f64() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = &hdual1 / 2.0;

    assert_abs_diff_eq!(hdual2.re, 1.0);
    assert_abs_diff_eq!(hdual2.e1, 1.0);
    assert_abs_diff_eq!(hdual2.e2, 1.0);
    assert_abs_diff_eq!(hdual2.e1e2, 2.0);
}

#[test]
fn f64_div_hdual() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = 2.0 / hdual1;

    assert_abs_diff_eq!(hdual2.re, 1.0);
    assert_abs_diff_eq!(hdual2.e1, -1.0);
    assert_abs_diff_eq!(hdual2.e2, -1.0);
    assert_abs_diff_eq!(hdual2.e1e2, 0.0);
}

#[test]
fn f64_div_borrow_hdual() {
    let hdual1 = HDual{re: 2.0, e1: 2.0, e2: 2.0, e1e2: 4.0};
    let hdual2 = 2.0 / &hdual1;

    assert_abs_diff_eq!(hdual2.re, 1.0);
    assert_abs_diff_eq!(hdual2.e1, -1.0);
    assert_abs_diff_eq!(hdual2.e2, -1.0);
    assert_abs_diff_eq!(hdual2.e1e2, 0.0);
}
