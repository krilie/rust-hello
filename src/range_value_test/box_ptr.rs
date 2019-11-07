pub fn test() {
    #[derive(PartialEq,Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    let box_point = Box::new(Point { x: 0.0, y: 0.0 });
    let unbox_point: Point = *box_point;
    assert_eq!(unbox_point, Point { x: 0.0, y: 0.0 });
    println!("{:?}",unbox_point);
}
