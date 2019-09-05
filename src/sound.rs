
    fn guitar() {}

    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                super::show();
                super::super::guitar();
            }
        }
        pub fn show() {
            println!("hello show");
        }
    }

    mod voice {}

    pub struct  Vegetable{
        pub name :String,
        id: i32,
    }
    impl  Vegetable {
        pub fn new(name : &str) -> Vegetable {
            Vegetable{
                name:String::from(name),
                id:1,
            }
        }
    }
