use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Person { name: String, age: u8 },
    Dog { name: String, age: u8 },
    Text { text: String },
    FriendRequest { from: String, to: String },
}
