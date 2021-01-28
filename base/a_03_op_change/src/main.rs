use std::ops::Add;

#[derive(Debug)]
struct Complex {
    a: f64,
    b: f64,
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {a: self.a+other.a, b: self.b+other.b}
    }
}

fn main() {
    let cp1 = Complex{a: 1f64, b: 2.0};
    let cp2 = Complex{a: 5.0, b:8.1};
    let cp3 = cp1 + cp2;
    print!("{:?}", cp3);
}

mod one{
    use std::ops::Add;

    #[derive(Debug)]
    struct Complex {
        a: f64,
        b: f64,
    }

    impl Add for Complex {
        type Output = Complex;
        fn add(self, other: Complex) -> Complex {
            Complex {a: self.a+other.a, b: self.b+other.b}
        }
    }

    impl Add<i32> for Complex {
        type Output = f64;
        fn add(self, other: i32) -> f64 {
            self.a + self.b + (other as f64)
        }
    }

    #[test]
    fn main() {
        let cp1 = Complex{a: 1f64, b: 2.0};
        let cp2 = Complex{a: 5.0, b:8.1};
        let cp3 = Complex{a: 9.0, b:20.0};
        let complex_add_result = cp1 + cp2;
        print!("{:?}\n", complex_add_result);
        print!("{:?}", cp3 + 10i32);
    }
}

mod two{
    use std::ops::Mul;

    trait HasArea<T> {
        fn area(&self) -> T;
    }

    struct Square<T> {
        x: T,
        y: T,
        side: T,
    }

    impl<T> HasArea<T> for Square<T>
        where T: Mul<Output=T> + Copy {
        fn area(&self) -> T {
            self.side * self.side
        }
    }

    #[test]
    fn main() {
        let s = Square {
            x: 0.0f64,
            y: 0.0f64,
            side: 12.0f64,
        };

        println!("Area of s: {}", s.area());
    }
}
