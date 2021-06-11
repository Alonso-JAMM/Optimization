use optimization::number_system::HyperDualScalar as HDual;
use approx::assert_abs_diff_eq;


#[test]
fn new_values() {
    let hdual = HDual::new();

    assert_abs_diff_eq!(hdual.real, 0.0);
    assert_abs_diff_eq!(hdual.grad, 0.0);
    assert_abs_diff_eq!(hdual.hess, 0.0);
}


#[test]
fn hdual_add_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = hdual1 + hdual2;

    assert_abs_diff_eq!(hdual3.real, 4.0);
    assert_abs_diff_eq!(hdual3.grad, 6.0);
    assert_abs_diff_eq!(hdual3.hess, 8.0);
}

#[test]
fn borrow_hdual_add_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = &hdual1 + &hdual2;

    assert_abs_diff_eq!(hdual3.real, 4.0);
    assert_abs_diff_eq!(hdual3.grad, 6.0);
    assert_abs_diff_eq!(hdual3.hess, 8.0);
}

#[test]
fn hdual_add_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = hdual1 + &hdual2;

    assert_abs_diff_eq!(hdual3.real, 4.0);
    assert_abs_diff_eq!(hdual3.grad, 6.0);
    assert_abs_diff_eq!(hdual3.hess, 8.0);
}

#[test]
fn borrow_hdual_add_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = &hdual1 + hdual2;

    assert_abs_diff_eq!(hdual3.real, 4.0);
    assert_abs_diff_eq!(hdual3.grad, 6.0);
    assert_abs_diff_eq!(hdual3.hess, 8.0);
}

#[test]
fn hdual_add_f64() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = hdual1 + 2.0;

    assert_abs_diff_eq!(hdual2.real, 3.0);
    assert_abs_diff_eq!(hdual2.grad, 2.0);
    assert_abs_diff_eq!(hdual2.hess, 3.0);
}

#[test]
fn borrow_hdual_add_f64() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = hdual1 + 2.0;

    assert_abs_diff_eq!(hdual2.real, 3.0);
    assert_abs_diff_eq!(hdual2.grad, 2.0);
    assert_abs_diff_eq!(hdual2.hess, 3.0);
}

#[test]
fn f64_add_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = 2.0 + hdual1;

    assert_abs_diff_eq!(hdual2.real, 3.0);
    assert_abs_diff_eq!(hdual2.grad, 2.0);
    assert_abs_diff_eq!(hdual2.hess, 3.0);
}

#[test]
fn f64_add_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = 2.0 + &hdual1;

    assert_abs_diff_eq!(hdual2.real, 3.0);
    assert_abs_diff_eq!(hdual2.grad, 2.0);
    assert_abs_diff_eq!(hdual2.hess, 3.0);
}



#[test]
fn hdual_sub_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = hdual1 - hdual2;

    assert_abs_diff_eq!(hdual3.real, -2.0);
    assert_abs_diff_eq!(hdual3.grad, -2.0);
    assert_abs_diff_eq!(hdual3.hess, -2.0);
}

#[test]
fn borrow_hdual_sub_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = &hdual1 - &hdual2;

    assert_abs_diff_eq!(hdual3.real, -2.0);
    assert_abs_diff_eq!(hdual3.grad, -2.0);
    assert_abs_diff_eq!(hdual3.hess, -2.0);
}

#[test]
fn hdual_sub_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = hdual1 - &hdual2;

    assert_abs_diff_eq!(hdual3.real, -2.0);
    assert_abs_diff_eq!(hdual3.grad, -2.0);
    assert_abs_diff_eq!(hdual3.hess, -2.0);
}

#[test]
fn borrow_hdual_sub_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = &hdual1 - hdual2;

    assert_abs_diff_eq!(hdual3.real, -2.0);
    assert_abs_diff_eq!(hdual3.grad, -2.0);
    assert_abs_diff_eq!(hdual3.hess, -2.0);
}

#[test]
fn hdual_sub_f64() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = hdual1 - 2.0;

    assert_abs_diff_eq!(hdual2.real, -1.0);
    assert_abs_diff_eq!(hdual2.grad, 2.0);
    assert_abs_diff_eq!(hdual2.hess, 3.0);
}

#[test]
fn borrow_hdual_sub_f64() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = &hdual1 - 2.0;

    assert_abs_diff_eq!(hdual2.real, -1.0);
    assert_abs_diff_eq!(hdual2.grad, 2.0);
    assert_abs_diff_eq!(hdual2.hess, 3.0);
}

#[test]
fn f64_sub_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = 2.0 - hdual1;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad, -2.0);
    assert_abs_diff_eq!(hdual2.hess, -3.0);
}

#[test]
fn f64_sub_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = 2.0 - &hdual1;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad, -2.0);
    assert_abs_diff_eq!(hdual2.hess, -3.0);
}


#[test]
fn neg_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = -hdual1;

    assert_abs_diff_eq!(hdual2.real, -1.0);
    assert_abs_diff_eq!(hdual2.grad, -2.0);
    assert_abs_diff_eq!(hdual2.hess, -3.0);
}

#[test]
fn neg_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = -&hdual1;

    assert_abs_diff_eq!(hdual2.real, -1.0);
    assert_abs_diff_eq!(hdual2.grad, -2.0);
    assert_abs_diff_eq!(hdual2.hess, -3.0);
}



#[test]
fn hdual_mul_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = hdual1 * hdual2;

    assert_abs_diff_eq!(hdual3.real, 3.0);
    assert_abs_diff_eq!(hdual3.grad, 10.0);
    assert_abs_diff_eq!(hdual3.hess, 30.0);
}

#[test]
fn borrow_hdual_mul_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = &hdual1 * &hdual2;

    assert_abs_diff_eq!(hdual3.real, 3.0);
    assert_abs_diff_eq!(hdual3.grad, 10.0);
    assert_abs_diff_eq!(hdual3.hess, 30.0);
}

#[test]
fn hdual_mul_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = hdual1 * &hdual2;

    assert_abs_diff_eq!(hdual3.real, 3.0);
    assert_abs_diff_eq!(hdual3.grad, 10.0);
    assert_abs_diff_eq!(hdual3.hess, 30.0);
}

#[test]
fn borrow_hdual_mul_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = HDual{real: 3.0, grad: 4.0, hess: 5.0};
    let hdual3 = &hdual1 * hdual2;

    assert_abs_diff_eq!(hdual3.real, 3.0);
    assert_abs_diff_eq!(hdual3.grad, 10.0);
    assert_abs_diff_eq!(hdual3.hess, 30.0);
}

#[test]
fn hdual_mul_f64() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = hdual1 * 2.0;

    assert_abs_diff_eq!(hdual2.real, 2.0);
    assert_abs_diff_eq!(hdual2.grad, 4.0);
    assert_abs_diff_eq!(hdual2.hess, 6.0);
}

#[test]
fn borrow_hdual_mul_f64() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = &hdual1 * 2.0;

    assert_abs_diff_eq!(hdual2.real, 2.0);
    assert_abs_diff_eq!(hdual2.grad, 4.0);
    assert_abs_diff_eq!(hdual2.hess, 6.0);
}

#[test]
fn f64_mul_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = 2.0 * hdual1;

    assert_abs_diff_eq!(hdual2.real, 2.0);
    assert_abs_diff_eq!(hdual2.grad, 4.0);
    assert_abs_diff_eq!(hdual2.hess, 6.0);
}

#[test]
fn f64_mul_borrow_hdual() {
    let hdual1 = HDual{real: 1.0, grad: 2.0, hess: 3.0};
    let hdual2 = 2.0 * &hdual1;

    assert_abs_diff_eq!(hdual2.real, 2.0);
    assert_abs_diff_eq!(hdual2.grad, 4.0);
    assert_abs_diff_eq!(hdual2.hess, 6.0);
}


#[test]
fn hdual_div_hdual() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = HDual{real: 4.0, grad: 3.0, hess: 5.0};
    let hdual3 = hdual1 / hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad, 0.125);
    assert_abs_diff_eq!(hdual3.hess, 0.1875);
}

#[test]
fn borrow_hdual_div_borrow_hdual() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = HDual{real: 4.0, grad: 3.0, hess: 5.0};
    let hdual3 = &hdual1 / &hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad, 0.125);
    assert_abs_diff_eq!(hdual3.hess, 0.1875);
}

#[test]
fn hdual_div_borrow_hdual() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = HDual{real: 4.0, grad: 3.0, hess: 5.0};
    let hdual3 = hdual1 / &hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad, 0.125);
    assert_abs_diff_eq!(hdual3.hess, 0.1875);
}

#[test]
fn borrow_hdual_div_hdual() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = HDual{real: 4.0, grad: 3.0, hess: 5.0};
    let hdual3 = &hdual1 / hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad, 0.125);
    assert_abs_diff_eq!(hdual3.hess, 0.1875);
}

#[test]
fn hdual_div_f64() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = hdual1 / 2.0;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad, 1.0);
    assert_abs_diff_eq!(hdual2.hess, 2.0);
}

#[test]
fn borrow_hdual_div_f64() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = &hdual1 / 2.0;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad, 1.0);
    assert_abs_diff_eq!(hdual2.hess, 2.0);
}

#[test]
fn f64_div_hdual() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = 2.0 / hdual1;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad, -1.0);
    assert_abs_diff_eq!(hdual2.hess, 0.0);
}

#[test]
fn f64_div_borrow_hdual() {
    let hdual1 = HDual{real: 2.0, grad: 2.0, hess: 4.0};
    let hdual2 = 2.0 / &hdual1;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad, -1.0);
    assert_abs_diff_eq!(hdual2.hess, 0.0);
}
