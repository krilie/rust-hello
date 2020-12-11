mod a;

// 多级结构
use self::a::b::c::d;

fn main() {
    d::print_ddd();
    self::a::b::c::d::print_ddd();
    self::a::d::print_ddd();
    self::a::print_ddd();
}

#[test]
fn test(){
    // Intent: `a` exports `I`, `bar`, and `foo`, but nothing else.
    pub mod a {
        pub const I: i32 = 3;
        use self::b::c::semisecret;
        pub fn bar(z: i32) -> i32 { semisecret(I) * z }
        pub fn foo(y: i32) -> i32 { semisecret(I) + y }

        mod b {
            pub(in a) use self::c::semisecret;
            mod c {
                const J: i32 = 4; // J is meant to be hidden from the outside world.

                // `pub(in a)` means "usable within hierarchy of `mod a`, but not
                // elsewhere."
                pub fn semisecret(x: i32) -> i32  { x + J }
            }
        }
    }
    println!("{}", a::foo(2));
}
