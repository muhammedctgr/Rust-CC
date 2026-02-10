use vector::*;

#[test]
fn test_init() {
    let v = init(1, 2, 3);
    assert_eq!(v.len(), 3);
    
