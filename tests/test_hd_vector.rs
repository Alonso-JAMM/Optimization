use optimization::number_system::HyperDualScalar as HDual;
use optimization::geometry::{HDQuaternion, HDVector};
use approx::assert_abs_diff_eq;


#[test]
fn new_values() {
    let v = HDVector::new();

    // Only check real part assuming hyperdual scalars work as intended
    assert_abs_diff_eq!(v.x.re, 0.0);
    assert_abs_diff_eq!(v.y.re, 0.0);
    assert_abs_diff_eq!(v.z.re, 0.0);
}

#[test]
fn from_quaternion() {
    let pi = std::f64::consts::PI;
    let phi = HDual{re: pi/2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let theta = HDual{re: pi/2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let psi = HDual::new();
    let q = HDQuaternion::from_angles(phi, theta, psi);
    let v = HDVector::from_quaternion(&q);
    // Only check real part assuming hyperdual scalars work as intended
    assert_abs_diff_eq!(v.x.re, 0.5);
    assert_abs_diff_eq!(v.y.re, 0.5);
    assert_abs_diff_eq!(v.z.re, -0.5);
}


#[test]
fn dot_product() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: x1, y: y2, z: z2};

    let v = a.dot(&b);

    assert_abs_diff_eq!(v.re, 1.0);
}


#[test]
fn cross_product() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: HDual::new()};

    let v = a.cross(&b);

    assert_abs_diff_eq!(v.x.re, 0.0);
    assert_abs_diff_eq!(v.y.re, 0.0);
    assert_abs_diff_eq!(v.z.re, 1.0);
}


#[test]
fn hdvector_add_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: z2};

    let v = a + b;

    assert_abs_diff_eq!(v.x.re, 1.0);
    assert_abs_diff_eq!(v.y.re, 2.0);
    assert_abs_diff_eq!(v.z.re, 3.0);
}

#[test]
fn borrow_hdvector_add_borrow_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: z2};

    let v = &a + &b;

    assert_abs_diff_eq!(v.x.re, 1.0);
    assert_abs_diff_eq!(v.y.re, 2.0);
    assert_abs_diff_eq!(v.z.re, 3.0);
}

#[test]
fn hdvector_add_borrow_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: z2};

    let v = a + &b;

    assert_abs_diff_eq!(v.x.re, 1.0);
    assert_abs_diff_eq!(v.y.re, 2.0);
    assert_abs_diff_eq!(v.z.re, 3.0);
}

#[test]
fn borrow_hdvector_add_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: z2};

    let v = &a + b;

    assert_abs_diff_eq!(v.x.re, 1.0);
    assert_abs_diff_eq!(v.y.re, 2.0);
    assert_abs_diff_eq!(v.z.re, 3.0);
}


#[test]
fn hdvector_sub_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: z2};

    let v = a - b;

    assert_abs_diff_eq!(v.x.re, 1.0);
    assert_abs_diff_eq!(v.y.re, -2.0);
    assert_abs_diff_eq!(v.z.re, -3.0);
}

#[test]
fn borrow_hdvector_sub_borrow_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: z2};

    let v = &a - &b;

    assert_abs_diff_eq!(v.x.re, 1.0);
    assert_abs_diff_eq!(v.y.re, -2.0);
    assert_abs_diff_eq!(v.z.re, -3.0);
}

#[test]
fn hdvector_sub_borrow_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y2 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z2 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: HDual::new(), z: HDual::new()};
    let b = HDVector{x: HDual::new(), y: y2, z: z2};

    let v = a - &b;

    assert_abs_diff_eq!(v.x.re, 1.0);
    assert_abs_diff_eq!(v.y.re, -2.0);
    assert_abs_diff_eq!(v.z.re, -3.0);
}


#[test]
fn neg_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};

    let v = -a;

    assert_abs_diff_eq!(v.x.re, -1.0);
    assert_abs_diff_eq!(v.y.re, -2.0);
    assert_abs_diff_eq!(v.z.re, -3.0);
}

#[test]
fn neg_borrow_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};

    let v = -&a;

    assert_abs_diff_eq!(v.x.re, -1.0);
    assert_abs_diff_eq!(v.y.re, -2.0);
    assert_abs_diff_eq!(v.z.re, -3.0);
}


#[test]
fn hdvector_mul_f64() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = a*2.0;

    assert_abs_diff_eq!(v.x.re, 2.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn borrow_hdvector_mul_f64() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = &a*2.0;

    assert_abs_diff_eq!(v.x.re, 2.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn f64_mul_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = 2.0*a;

    assert_abs_diff_eq!(v.x.re, 2.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn f64_mul_borrow_hdvector() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = 2.0*&a;

    assert_abs_diff_eq!(v.x.re, 2.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn hdvector_mul_hdual() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = a * x1;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn borrow_hdvector_mul_borrow_hdual() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = &a * &x1;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn hdvector_mul_borrow_hdual() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = a * &x1;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn borrow_hdvector_mul_hdual() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = &a * x1;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}


#[test]
fn hdual_mul_hdvector() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = x1 * a;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn borrow_hdual_mul_borrow_hdvector() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = &x1 * &a;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}

#[test]
fn hdual_mul_borrow_hdvector() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = x1 * &a;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}


#[test]
fn borrow_hdual_mul_hdvector() {
    let x1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 3.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = &x1 * a;

    assert_abs_diff_eq!(v.x.re, 4.0);
    assert_abs_diff_eq!(v.y.re, 4.0);
    assert_abs_diff_eq!(v.z.re, 6.0);
}



#[test]
fn hdvector_div_f64() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 4.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = a/2.0;

    assert_abs_diff_eq!(v.x.re, 0.5);
    assert_abs_diff_eq!(v.y.re, 1.0);
    assert_abs_diff_eq!(v.z.re, 2.0);
}

#[test]
fn borrow_hdvector_div_f64() {
    let x1 = HDual{re: 1.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let y1 = HDual{re: 2.0, e1: 0.0, e2: 0.0, e1e2: 0.0};
    let z1 = HDual{re: 4.0, e1: 0.0, e2: 0.0, e1e2: 0.0};

    let a = HDVector{x: x1, y: y1, z: z1};
    let v = &a/2.0;

    assert_abs_diff_eq!(v.x.re, 0.5);
    assert_abs_diff_eq!(v.y.re, 1.0);
    assert_abs_diff_eq!(v.z.re, 2.0);
}
