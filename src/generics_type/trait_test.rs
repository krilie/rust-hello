struct Duck;
struct Pig;
trait Fly {
    fn fly(&self) -> bool;
}
impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}
impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}
fn fly_dyn(s: &impl Fly) -> bool {
    s.fly()
}

pub fn test() {
    let pig = Pig;
    println!("{:?}", fly_static(pig));
    let duck = Duck;
    println!("{:?}", fly_dyn(&duck));
}

pub mod debug_trait {
    use std::fmt::*;
    //#[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Debug for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Point ---- == == x:{} y:{}", self.x, self.y)
        }
    }
    pub fn test() {
        let origin = Point { x: 0, y: 0 };
        println!("{:?}", origin);
    }
}
