macro_rules! numin {
      () =>{
          {
            let mut input = String::new();
              std::io::stdin()
                  .read_line(&mut input)
                .expect("Failed to read line");
              input.trim().parse().unwrap()
        }
    };
}

fn main() {
    let num: i32 = numin!();
    println!("您输入的数字是：{}", num);
}
