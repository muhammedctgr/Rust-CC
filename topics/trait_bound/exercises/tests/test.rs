use trait_bound::*;

#[test]
fn test_min() {
    assert_eq!(min(1u32, 2u32), 1u32);
    assert_eq!(min(0i32, -1i32), -1i32);
    assert_eq!(min(0.0, 3.0), 0.0);
    assert_eq!(min('a', 'b'), 'a');
}


