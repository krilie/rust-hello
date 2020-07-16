pub use  serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Point {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    x: i32,
    #[serde(default)]
    y: i32,
    // #[serde(tag = "the_z")]
    z: Option<i32>,
    ok_for_test: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Message {
    Request { id: String, method: String, params: String },
    Response { id: String, result: String },
}

#[test]
fn main_test_json() {
    let point = Point { x: 1, y: 2, z: Option::from(32), ok_for_test: None };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let msg = Message::Request {
        id: "".to_string(),
        method: "".to_string(),
        params: "".to_string(),
    };
    let msg_str = serde_json::to_string(&msg).unwrap();
    println!("{}", msg_str)
}