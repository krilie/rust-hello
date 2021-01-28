//! SHA3-256 示例
extern crate crypto;
extern crate rustc_hex;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use self::crypto::md5::Md5;

#[test]
fn test_rustc() {
    // create a SHA3-256 object
    let mut hasher = Md5::new();
    // write input message
    hasher.input_str("hello world");
    // read hash digest
    let hex = hasher.result_str();
    println!("{}", hex)
}

mod randomise {
    use rand::Rng;

    #[test]
    fn main() {
        let mut rng = rand::thread_rng();

        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();
        println!("Random u8: {}", n1);
        println!("Random u16: {}", n2);
        println!("Random u32: {}", rng.gen::<u32>());
        println!("Random i32: {}", rng.gen::<i32>());
        println!("Random float: {}", rng.gen::<f64>());
    }
}

mod t1 {
    use rand::Rng;

    #[test]
    fn main() {
        let mut rng = rand::thread_rng();
        println!("Integer: {}", rng.gen_range(0, 10));
        println!("Float: {}", rng.gen_range(0.0, 10.0));
    }
}

mod t2 {
    use rand::distributions::{Distribution, Uniform};

    #[test]
    fn main() {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..7);

        loop {
            let throw = die.sample(&mut rng);
            println!("Roll the die: {}", throw);
            if throw == 6 {
                break;
            }
        }
    }
}


mod t5 {
    use rand::Rng;
    use rand::distributions::{Distribution, Standard};

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Distribution<Point> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
            let (rand_x, rand_y) = rng.gen();
            Point {
                x: rand_x,
                y: rand_y,
            }
        }
    }

    #[test]
    fn main() {
        let mut rng = rand::thread_rng();
        let rand_tuple = rng.gen::<(i32, bool, f64)>();
        let rand_point: Point = rng.gen();
        println!("Random tuple: {:?}", rand_tuple);
        println!("Random Point: {:?}", rand_point);
    }
}

mod t11 {
    #[test]
    fn main() {
        let mut vec = vec![1, 5, 10, 2, 15];
        vec.sort();
        vec.reverse();
        assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    }
}

mod t33 {
    #[test]
    fn main() {
        let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    }
}

mod t55 {
    use rayon::prelude::*;

    #[test]
    fn main() {
        let mut arr = [0, 7, 9, 11];
        arr.par_iter_mut().for_each(|p| *p -= 1);
        println!("{:?}", arr);
    }
}


mod t6 {
    use rayon::prelude::*;

    #[test]
    fn main() {
        let mut vec = vec![2, 4, 6, 8];

        assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
        assert!(vec.par_iter().all(|n| (*n % 2) == 0));
        assert!(!vec.par_iter().any(|n| *n > 8));
        assert!(vec.par_iter().all(|n| *n <= 8));

        vec.push(9);

        assert!(vec.par_iter().any(|n| (*n % 2) != 0));
        assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
        assert!(vec.par_iter().any(|n| *n > 8));
        assert!(!vec.par_iter().all(|n| *n <= 8));
        let b = vec.par_iter().all(|n| *n <= 8);
        println!("{}", b)
    }
}

// mod t532 {
//     use futures::executor::block_on;
//     async fn process() {
//         println!("Hello world!");
//     }
//     #[test]
//     fn test() {
//         let future = process();
//         block_on(future);
//     }
// }

mod t332 {
    fn display<T>(age: u32, print_info: T) where T: Fn(u32) {
        print_info(age);
    }

    #[test]
    fn main() {
        let mut name = String::from("Ethan");

        let print_info_closure = |age| {
            println!("name is {}", name);
            println!("age is {}", age);
        };
        println!("name out is {}", name);
        let age = 18;
        display(age, print_info_closure);
    }

    #[test]
    fn test2() {
        let lifttime_closure = |a, b| {
            println!("{}", a);
            println!("{}", b);
            b
        };
        let a = String::from("abc");
        let c;
        {
            let b = String::from("xyz");
            c = lifttime_closure(&a, &b);
        }
        // println!("{}", c); // 报错
    }
}

mod t221 {
    fn closure_inside() -> Box<dyn FnMut() -> ()>
    {
        let mut age = 1;
        let mut name = String::from("Ethan");
        let age_closure = move || {
            name.push_str(" Yuan");
            age += 1;
            println!("name is {}", name);
            println!("age is {}", age);
        };
        Box::new(age_closure)
    }

    #[test]
    fn main() {
        let mut age_closure = closure_inside();
        age_closure();
        age_closure();
        age_closure();
        let mut age_closure = closure_inside();
        age_closure();
    }
}