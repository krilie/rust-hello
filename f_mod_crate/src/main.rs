mod aaa;

mod ccc {
    pub fn print_ccc() {
        println!("{}", 25);
    }
}

fn main() {
    {
        use ccc::print_ccc;
        print_ccc();
    }
    {
        // 或者
        ccc::print_ccc();
    }
    {
        aaa::print_aaa();
    }
    {
        use self::aaa::print_aaa;
        print_aaa();
    }
}
