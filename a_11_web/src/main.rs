extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
}

mod one {

    extern crate iron;
    extern crate rustc_serialize;

    use iron::prelude::*;
    use iron::status;
    use rustc_serialize::json;
    use std::borrow::Borrow;

    #[derive(RustcEncodable)]
    struct Greeting {
        msg: String
    }
    #[test]
    fn main() {
        fn hello_world(req: &mut Request) -> IronResult<Response> {
            let greeting = Greeting { msg: "Hello, World".to_string() };
            let payload = json::encode(&greeting).unwrap();
            Ok(Response::with((status::Ok, payload)))
        }

        Iron::new(hello_world).http("localhost:3000").unwrap();
        println!("On 3000");
    }
}
