mod a;

// 多级结构
use self::a::b::c::d;

fn main() {
    d::print_ddd();
}
