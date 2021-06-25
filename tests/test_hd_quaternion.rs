use optimization::number_system::HyperDualScalar as HDual;
use optimization::geometry::HDQuaternion;
use approx::assert_abs_diff_eq;


#[test]
fn new_values() {
    let q = HDQuaternion::new();

    // Only check real part assuming hyperdual scalars work as intended
    assert_abs_diff_eq!(q.q0.re, 0.0);
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 0.0);
}


#[test]
fn from_x_value() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q = HDQuaternion::from_x_angle(phi/2.0);

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 1.0/2.0_f64.sqrt());
}

#[test]
fn from_y_value() {
    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q = HDQuaternion::from_y_angle(theta/2.0);

    assert_abs_diff_eq!(q.q0.re, 0.0);
    assert_abs_diff_eq!(q.q1.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 1.0/2.0_f64.sqrt());
}

#[test]
fn from_z_value() {
    let psi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q = HDQuaternion::from_z_angle(psi/2.0);

    assert_abs_diff_eq!(q.q0.re, 0.0);
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q3.re, 1.0/2.0_f64.sqrt());
}

#[test]
fn invue() {
    let psi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q = HDQuaternion::from_z_angle(psi/2.0).inv();

    assert_abs_diff_eq!(q.q0.re, 0.0);
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, -1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q3.re, 1.0/2.0_f64.sqrt());
}



#[test]
fn q_add_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = q_a + q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}

#[test]
fn borrow_q_add_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = &q_a + &q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}

#[test]
fn q_add_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = q_a + &q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}

#[test]
fn borrow_q_add_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = &q_a + q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}

#[test]
fn q_sub_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = q_a - q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, -1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 0.0);
}

#[test]
fn borrow_q_sub_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = &q_a - &q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, -1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 0.0);
}

#[test]
fn q_sub_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = q_a - &q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, -1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 0.0);
}

#[test]
fn borrow_q_sub_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = &q_a - q_b;

    assert_abs_diff_eq!(q.q0.re, 1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, -1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 0.0);
}


#[test]
fn neg_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let q = -q_a;

    assert_abs_diff_eq!(q.q0.re, -1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, -1.0/2.0_f64.sqrt());
}

#[test]
fn neg_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let q = -&q_a;

    assert_abs_diff_eq!(q.q0.re, -1.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, -1.0/2.0_f64.sqrt());
}


#[test]
fn q_mul_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = q_b * q_a;

    assert_abs_diff_eq!(q.q0.re, 0.5);
    assert_abs_diff_eq!(q.q1.re, 0.5);
    assert_abs_diff_eq!(q.q2.re, -0.5);
    assert_abs_diff_eq!(q.q3.re, 0.5);
}

#[test]
fn borrow_q_mul_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = &q_b * &q_a;

    assert_abs_diff_eq!(q.q0.re, 0.5);
    assert_abs_diff_eq!(q.q1.re, 0.5);
    assert_abs_diff_eq!(q.q2.re, -0.5);
    assert_abs_diff_eq!(q.q3.re, 0.5);
}

#[test]
fn q_mul_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = q_b * &q_a;

    assert_abs_diff_eq!(q.q0.re, 0.5);
    assert_abs_diff_eq!(q.q1.re, 0.5);
    assert_abs_diff_eq!(q.q2.re, -0.5);
    assert_abs_diff_eq!(q.q3.re, 0.5);
}

#[test]
fn borrow_q_mul_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let theta = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_b = HDQuaternion::from_y_angle(theta/2.0);

    let q = &q_b * &q_a;

    assert_abs_diff_eq!(q.q0.re, 0.5);
    assert_abs_diff_eq!(q.q1.re, 0.5);
    assert_abs_diff_eq!(q.q2.re, -0.5);
    assert_abs_diff_eq!(q.q3.re, 0.5);
}


#[test]
fn q_mul_f64() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let q = q_a * 2.0;

    assert_abs_diff_eq!(q.q0.re, 2.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}

#[test]
fn borrow_q_mul_f64() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let q = &q_a * 2.0;

    assert_abs_diff_eq!(q.q0.re, 2.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}

#[test]
fn f64_mul_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let q = 2.0 * q_a;

    assert_abs_diff_eq!(q.q0.re, 2.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}

#[test]
fn f64_mul_borrow_q() {
    let phi = HDual{re: std::f64::consts::PI, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let q_a = HDQuaternion::from_x_angle(phi/2.0);

    let q = 2.0 * &q_a;

    assert_abs_diff_eq!(q.q0.re, 2.0/2.0_f64.sqrt());
    assert_abs_diff_eq!(q.q1.re, 0.0);
    assert_abs_diff_eq!(q.q2.re, 0.0);
    assert_abs_diff_eq!(q.q3.re, 2.0/2.0_f64.sqrt());
}
