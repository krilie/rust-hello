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

mod two{
    extern crate iron;
    extern crate router;
    extern crate rustc_serialize;

    use iron::prelude::*;
    use iron::status;
    use router::Router;
    use rustc_serialize::json;
    use std::io::Read;

    #[derive(RustcEncodable, RustcDecodable)]
    struct Greeting {
        msg: String
    }
    #[test]
    fn main() {
        let mut routers = Router::new();
        routers.get("/", hello_world,"1");
        routers.get("/set", set_greeting,"2");

        fn hello_world(_: &mut Request) -> IronResult<Response> {
            let greeting = Greeting { msg: "Hello, World".to_string() };
            let payload = json::encode(&greeting).unwrap();
            Ok(Response::with((status::Ok, payload)))
        }

        fn set_greeting(request: &mut Request) -> IronResult<Response> {
            let mut payload = String::new();
            request.body.read_to_string(&mut payload);
            let request: Greeting = json::decode(&payload).unwrap();
            let greeting = Greeting { msg: request.msg };
            let payload = json::encode(&greeting).unwrap();
            Ok(Response::with((status::Ok, payload)))
        }
        Iron::new(routers).http("localhost:3000").unwrap();
    }
}
