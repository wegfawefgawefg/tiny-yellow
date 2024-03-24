use common::Message;
use warp::Filter;

pub mod common;

#[tokio::main]
async fn main() {
    let people = warp::path("people").map(|| {
        // let person = Message::Person {
        //     name: "John Doe".into(),
        //     age: 30,
        // };
        let thing = Message::Dog {
            name: "Fido".into(),
            age: 5,
        };
        warp::reply::json(&thing)
    });

    warp::serve(people).run(([127, 0, 0, 1], 3030)).await;
}
