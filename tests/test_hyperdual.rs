use optimization::number_system::HyperDual;
use approx::assert_abs_diff_eq;
use ndarray::{arr1, Array, Array2};


#[test]
fn new_values() {
    let var = HyperDual::new(3);

    assert_abs_diff_eq!(var.real, 0.0);
    assert_abs_diff_eq!(var.grad[0], 0.0);
    assert_abs_diff_eq!(var.grad[1], 0.0);
    assert_abs_diff_eq!(var.grad[2], 0.0);
    // all the elements should be zero.
    let hess_sum = var.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn hdual_add_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 + hdual2;

    assert_abs_diff_eq!(hdual3.real, 5.0);
    assert_abs_diff_eq!(hdual3.grad[0], 9.0);
    assert_abs_diff_eq!(hdual3.grad[1], 11.0);
    assert_abs_diff_eq!(hdual3.grad[2], 13.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_add_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 + &hdual2;

    assert_abs_diff_eq!(hdual3.real, 5.0);
    assert_abs_diff_eq!(hdual3.grad[0], 9.0);
    assert_abs_diff_eq!(hdual3.grad[1], 11.0);
    assert_abs_diff_eq!(hdual3.grad[2], 13.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn hdual_add_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 + &hdual2;

    assert_abs_diff_eq!(hdual3.real, 5.0);
    assert_abs_diff_eq!(hdual3.grad[0], 9.0);
    assert_abs_diff_eq!(hdual3.grad[1], 11.0);
    assert_abs_diff_eq!(hdual3.grad[2], 13.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_add_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 + hdual2;

    assert_abs_diff_eq!(hdual3.real, 5.0);
    assert_abs_diff_eq!(hdual3.grad[0], 9.0);
    assert_abs_diff_eq!(hdual3.grad[1], 11.0);
    assert_abs_diff_eq!(hdual3.grad[2], 13.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn hdual_add_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = hdual1 + 3.0;

    assert_abs_diff_eq!(hdual2.real, 5.0);
    assert_abs_diff_eq!(hdual2.grad[0], 3.0);
    assert_abs_diff_eq!(hdual2.grad[1], 4.0);
    assert_abs_diff_eq!(hdual2.grad[2], 5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_add_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = &hdual1 + 3.0;

    assert_abs_diff_eq!(hdual2.real, 5.0);
    assert_abs_diff_eq!(hdual2.grad[0], 3.0);
    assert_abs_diff_eq!(hdual2.grad[1], 4.0);
    assert_abs_diff_eq!(hdual2.grad[2], 5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn f64_add_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 3.0 + hdual1;

    assert_abs_diff_eq!(hdual2.real, 5.0);
    assert_abs_diff_eq!(hdual2.grad[0], 3.0);
    assert_abs_diff_eq!(hdual2.grad[1], 4.0);
    assert_abs_diff_eq!(hdual2.grad[2], 5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn f64_add_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 3.0 + &hdual1;

    assert_abs_diff_eq!(hdual2.real, 5.0);
    assert_abs_diff_eq!(hdual2.grad[0], 3.0);
    assert_abs_diff_eq!(hdual2.grad[1], 4.0);
    assert_abs_diff_eq!(hdual2.grad[2], 5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}


#[test]
fn hdual_sub_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 - hdual2;

    assert_abs_diff_eq!(hdual3.real, -1.0);
    assert_abs_diff_eq!(hdual3.grad[0], -3.0);
    assert_abs_diff_eq!(hdual3.grad[1], -3.0);
    assert_abs_diff_eq!(hdual3.grad[2], -3.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_sub_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 - &hdual2;

    assert_abs_diff_eq!(hdual3.real, -1.0);
    assert_abs_diff_eq!(hdual3.grad[0], -3.0);
    assert_abs_diff_eq!(hdual3.grad[1], -3.0);
    assert_abs_diff_eq!(hdual3.grad[2], -3.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn hdual_sub_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 - &hdual2;

    assert_abs_diff_eq!(hdual3.real, -1.0);
    assert_abs_diff_eq!(hdual3.grad[0], -3.0);
    assert_abs_diff_eq!(hdual3.grad[1], -3.0);
    assert_abs_diff_eq!(hdual3.grad[2], -3.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_sub_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 - hdual2;

    assert_abs_diff_eq!(hdual3.real, -1.0);
    assert_abs_diff_eq!(hdual3.grad[0], -3.0);
    assert_abs_diff_eq!(hdual3.grad[1], -3.0);
    assert_abs_diff_eq!(hdual3.grad[2], -3.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn hdual_sub_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = hdual1 - 3.0;

    assert_abs_diff_eq!(hdual2.real, -1.0);
    assert_abs_diff_eq!(hdual2.grad[0], 3.0);
    assert_abs_diff_eq!(hdual2.grad[1], 4.0);
    assert_abs_diff_eq!(hdual2.grad[2], 5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_sub_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = &hdual1 - 3.0;

    assert_abs_diff_eq!(hdual2.real, -1.0);
    assert_abs_diff_eq!(hdual2.grad[0], 3.0);
    assert_abs_diff_eq!(hdual2.grad[1], 4.0);
    assert_abs_diff_eq!(hdual2.grad[2], 5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn f64_hdual_sub_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 3.0 - hdual1;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad[0], -3.0);
    assert_abs_diff_eq!(hdual2.grad[1], -4.0);
    assert_abs_diff_eq!(hdual2.grad[2], -5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn f64_hdual_sub_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 3.0 - &hdual1;

    assert_abs_diff_eq!(hdual2.real, 1.0);
    assert_abs_diff_eq!(hdual2.grad[0], -3.0);
    assert_abs_diff_eq!(hdual2.grad[1], -4.0);
    assert_abs_diff_eq!(hdual2.grad[2], -5.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}


#[test]
fn neg_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual3 = -hdual1;

    assert_abs_diff_eq!(hdual3.real, -2.0);
    assert_abs_diff_eq!(hdual3.grad[0], -3.0);
    assert_abs_diff_eq!(hdual3.grad[1], -4.0);
    assert_abs_diff_eq!(hdual3.grad[2], -5.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn neg_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual3 = -&hdual1;

    assert_abs_diff_eq!(hdual3.real, -2.0);
    assert_abs_diff_eq!(hdual3.grad[0], -3.0);
    assert_abs_diff_eq!(hdual3.grad[1], -4.0);
    assert_abs_diff_eq!(hdual3.grad[2], -5.0);
    // all the elements should be zero.
    let hess_sum = hdual3.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}


#[test]
fn hdual_mul_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 * hdual2;

    assert_abs_diff_eq!(hdual3.real, 6.0);
    assert_abs_diff_eq!(hdual3.grad[0], 21.0);
    assert_abs_diff_eq!(hdual3.grad[1], 26.0);
    assert_abs_diff_eq!(hdual3.grad[2], 31.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 36.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 56.0);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 80.0);
}

#[test]
fn borrow_hdual_mul_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 * &hdual2;

    assert_abs_diff_eq!(hdual3.real, 6.0);
    assert_abs_diff_eq!(hdual3.grad[0], 21.0);
    assert_abs_diff_eq!(hdual3.grad[1], 26.0);
    assert_abs_diff_eq!(hdual3.grad[2], 31.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 36.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 56.0);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 80.0);
}

#[test]
fn hdual_mul_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 * &hdual2;

    assert_abs_diff_eq!(hdual3.real, 6.0);
    assert_abs_diff_eq!(hdual3.grad[0], 21.0);
    assert_abs_diff_eq!(hdual3.grad[1], 26.0);
    assert_abs_diff_eq!(hdual3.grad[2], 31.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 36.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 56.0);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 80.0);
}

#[test]
fn borrow_hdual_mul_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let grad2 = arr1::<f64>(&[6.0, 7.0, 8.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 3.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 * hdual2;

    assert_abs_diff_eq!(hdual3.real, 6.0);
    assert_abs_diff_eq!(hdual3.grad[0], 21.0);
    assert_abs_diff_eq!(hdual3.grad[1], 26.0);
    assert_abs_diff_eq!(hdual3.grad[2], 31.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 36.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], 45.0);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 56.0);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 54.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 67.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 80.0);
}

#[test]
fn hdual_mul_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = hdual1 * 3.0;

    assert_abs_diff_eq!(hdual2.real, 6.0);
    assert_abs_diff_eq!(hdual2.grad[0], 9.0);
    assert_abs_diff_eq!(hdual2.grad[1], 12.0);
    assert_abs_diff_eq!(hdual2.grad[2], 15.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_mul_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = &hdual1 * 3.0;

    assert_abs_diff_eq!(hdual2.real, 6.0);
    assert_abs_diff_eq!(hdual2.grad[0], 9.0);
    assert_abs_diff_eq!(hdual2.grad[1], 12.0);
    assert_abs_diff_eq!(hdual2.grad[2], 15.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn f64_mul_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 3.0 * hdual1;

    assert_abs_diff_eq!(hdual2.real, 6.0);
    assert_abs_diff_eq!(hdual2.grad[0], 9.0);
    assert_abs_diff_eq!(hdual2.grad[1], 12.0);
    assert_abs_diff_eq!(hdual2.grad[2], 15.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn f64_mul_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 3.0 * &hdual1;

    assert_abs_diff_eq!(hdual2.real, 6.0);
    assert_abs_diff_eq!(hdual2.grad[0], 9.0);
    assert_abs_diff_eq!(hdual2.grad[1], 12.0);
    assert_abs_diff_eq!(hdual2.grad[2], 15.0);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}



#[test]
fn hdual_div_hdual() {
    let grad1 = arr1::<f64>(&[1.0, 0.0, 0.0]);
    let grad2 = arr1::<f64>(&[0.0, 1.0, 0.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 4.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 / hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad[0], 0.25);
    assert_abs_diff_eq!(hdual3.grad[1], -0.125);
    assert_abs_diff_eq!(hdual3.grad[2], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 0.0);
}

#[test]
fn borrow_hdual_div_borrow_hdual() {
    let grad1 = arr1::<f64>(&[1.0, 0.0, 0.0]);
    let grad2 = arr1::<f64>(&[0.0, 1.0, 0.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 4.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 / &hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad[0], 0.25);
    assert_abs_diff_eq!(hdual3.grad[1], -0.125);
    assert_abs_diff_eq!(hdual3.grad[2], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 0.0);
}

#[test]
fn hdual_div_borrow_hdual() {
    let grad1 = arr1::<f64>(&[1.0, 0.0, 0.0]);
    let grad2 = arr1::<f64>(&[0.0, 1.0, 0.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 4.0, grad: grad2, hess:hess2};

    let hdual3 = hdual1 / &hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad[0], 0.25);
    assert_abs_diff_eq!(hdual3.grad[1], -0.125);
    assert_abs_diff_eq!(hdual3.grad[2], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 0.0);
}

#[test]
fn borrow_hdual_div_hdual() {
    let grad1 = arr1::<f64>(&[1.0, 0.0, 0.0]);
    let grad2 = arr1::<f64>(&[0.0, 1.0, 0.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));
    let hess2: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};
    let hdual2 = HyperDual{real: 4.0, grad: grad2, hess:hess2};

    let hdual3 = &hdual1 / hdual2;

    assert_abs_diff_eq!(hdual3.real, 0.5);
    assert_abs_diff_eq!(hdual3.grad[0], 0.25);
    assert_abs_diff_eq!(hdual3.grad[1], -0.125);
    assert_abs_diff_eq!(hdual3.grad[2], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[0,1]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[0,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[1,0]], -0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,1]], 0.0625);
    assert_abs_diff_eq!(hdual3.hess[[1,2]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,0]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,1]], 0.0);
    assert_abs_diff_eq!(hdual3.hess[[2,2]], 0.0);
}

#[test]
fn hdual_div_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = hdual1 / 4.0;

    assert_abs_diff_eq!(hdual2.real, 0.5);
    assert_abs_diff_eq!(hdual2.grad[0], 0.75);
    assert_abs_diff_eq!(hdual2.grad[1], 1.0);
    assert_abs_diff_eq!(hdual2.grad[2], 1.25);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn borrow_hdual_div_f64() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = &hdual1 / 4.0;

    assert_abs_diff_eq!(hdual2.real, 0.5);
    assert_abs_diff_eq!(hdual2.grad[0], 0.75);
    assert_abs_diff_eq!(hdual2.grad[1], 1.0);
    assert_abs_diff_eq!(hdual2.grad[2], 1.25);
    // all the elements should be zero.
    let hess_sum = hdual2.hess.sum();
    assert_abs_diff_eq!(hess_sum, 0.0);
}

#[test]
fn f64_div_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 4.0 / hdual1;

    assert_abs_diff_eq!(hdual2.real, 2.0);
    assert_abs_diff_eq!(hdual2.grad[0], -3.0);
    assert_abs_diff_eq!(hdual2.grad[1], -4.0);
    assert_abs_diff_eq!(hdual2.grad[2], -5.0);
    assert_abs_diff_eq!(hdual2.hess[[0,0]], 9.0);
    assert_abs_diff_eq!(hdual2.hess[[0,1]], 12.0);
    assert_abs_diff_eq!(hdual2.hess[[0,2]], 15.0);
    assert_abs_diff_eq!(hdual2.hess[[1,0]], 12.0);
    assert_abs_diff_eq!(hdual2.hess[[1,1]], 16.0);
    assert_abs_diff_eq!(hdual2.hess[[1,2]], 20.0);
    assert_abs_diff_eq!(hdual2.hess[[2,0]], 15.0);
    assert_abs_diff_eq!(hdual2.hess[[2,1]], 20.0);
    assert_abs_diff_eq!(hdual2.hess[[2,2]], 25.0);
}

#[test]
fn f64_div_borrow_hdual() {
    let grad1 = arr1::<f64>(&[3.0, 4.0, 5.0]);
    let hess1: Array2<f64> = Array::zeros((3, 3));

    let hdual1 = HyperDual{real: 2.0, grad: grad1, hess:hess1};

    let hdual2 = 4.0 / &hdual1;

    assert_abs_diff_eq!(hdual2.real, 2.0);
    assert_abs_diff_eq!(hdual2.grad[0], -3.0);
    assert_abs_diff_eq!(hdual2.grad[1], -4.0);
    assert_abs_diff_eq!(hdual2.grad[2], -5.0);
    assert_abs_diff_eq!(hdual2.hess[[0,0]], 9.0);
    assert_abs_diff_eq!(hdual2.hess[[0,1]], 12.0);
    assert_abs_diff_eq!(hdual2.hess[[0,2]], 15.0);
    assert_abs_diff_eq!(hdual2.hess[[1,0]], 12.0);
    assert_abs_diff_eq!(hdual2.hess[[1,1]], 16.0);
    assert_abs_diff_eq!(hdual2.hess[[1,2]], 20.0);
    assert_abs_diff_eq!(hdual2.hess[[2,0]], 15.0);
    assert_abs_diff_eq!(hdual2.hess[[2,1]], 20.0);
    assert_abs_diff_eq!(hdual2.hess[[2,2]], 25.0);
}
