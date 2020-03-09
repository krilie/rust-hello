struct Pick<F> {
    data: (u32, u32),
    func: F,
}

impl<F> Pick<F>
    where
        F: Fn(&(u32, u32)) -> &u32,
{
    fn call(&self) -> &u32 {
        (self.func)(&self.data)
    }
}

fn max(data: &(u32, u32)) -> &u32 {
    if data.0 > data.1 {
        &data.0
    } else {
        &data.1
    }
}

pub fn test() {
    let elm = Pick {
        data: (3, 1),
        func: max,
    };
    println!("test112 {:?}", elm.call());
}
