extern crate tri_loader;


#[test]
fn test_load() {
    let result = tri_loader::load("../assets/unity.tri");

    assert!(result.is_ok());
}

#[test]
fn test_load_elements() {
    let model = tri_loader::load("../assets/unity.tri").unwrap();
    let expected = 12583;
    let result = model.len();

    assert_eq!(result, expected);
}

#[test]
fn test_validate_elements() {
    let model = tri_loader::load("../assets/unity.tri").unwrap();
    assert!(model.iter().all(|triangle| {
        triangle.vertex0.x.is_finite() && triangle.vertex0.y.is_finite() && triangle.vertex0.z.is_finite() &&
        triangle.vertex1.x.is_finite() && triangle.vertex1.y.is_finite() && triangle.vertex1.z.is_finite() &&
        triangle.vertex2.x.is_finite() && triangle.vertex2.y.is_finite() && triangle.vertex2.z.is_finite()
    }));
}

