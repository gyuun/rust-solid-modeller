use rust_solid_modeller::math::{point::Point3, vector::Vector3};

const EPSILON: f64 = 1e-12;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "expected {expected}, got {actual}"
    );
}

fn assert_point_close(actual: Point3, expected: Point3) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.z, expected.z);
}

fn assert_vector_close(actual: Vector3, expected: Vector3) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.z, expected.z);
}

#[test]
fn point_plus_vector_returns_translated_point() {
    let point = Point3::new(1.0, 2.0, 3.0);
    let offset = Vector3::new(4.0, -2.0, 0.5);

    let result = point + offset;

    assert_point_close(result, Point3::new(5.0, 0.0, 3.5));
}

#[test]
fn point_minus_point_returns_displacement_vector() {
    let end = Point3::new(5.0, 1.0, 4.0);
    let start = Point3::new(2.0, 3.0, 1.0);

    let result = end - start;

    assert_vector_close(result, Vector3::new(3.0, -2.0, 3.0));
}

#[test]
fn vector_plus_vector_adds_each_component() {
    let lhs = Vector3::new(1.0, 2.0, 3.0);
    let rhs = Vector3::new(-2.0, 0.5, 4.0);

    let result = lhs + rhs;

    assert_vector_close(result, Vector3::new(-1.0, 2.5, 7.0));
}

#[test]
fn dot_product_returns_scalar_product() {
    let lhs = Vector3::new(1.0, 2.0, 3.0);
    let rhs = Vector3::new(4.0, -5.0, 6.0);

    assert_close(lhs.dot(rhs), 12.0);
}

#[test]
fn cross_product_follows_right_hand_rule() {
    let x_axis = Vector3::new(1.0, 0.0, 0.0);
    let y_axis = Vector3::new(0.0, 1.0, 0.0);

    let result = x_axis.cross(y_axis);

    assert_vector_close(result, Vector3::new(0.0, 0.0, 1.0));
}

#[test]
fn normalize_returns_unit_vector() {
    let vector = Vector3::new(3.0, 4.0, 0.0);

    let result = vector.normalize().expect("non-zero vector should normalize");

    assert_vector_close(result, Vector3::new(0.6, 0.8, 0.0));
    assert_close(result.dot(result), 1.0);
}

#[test]
fn normalize_rejects_near_zero_vector() {
    let vector = Vector3::new(1e-13, 0.0, 0.0);

    assert!(vector.normalize().is_none());
}
