use hyper::body::Buf;

async fn print_async(){
    println!("hello from print_async");
}

#[test]
fn test(){
    let future = print_async();
    println!("hello from main");
}
