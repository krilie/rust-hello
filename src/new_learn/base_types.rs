#[test]
fn number_test() {
    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);
}

#[test]
fn test_slice() {
    let vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);
    println!("{:?}", &vec[1..]);
}