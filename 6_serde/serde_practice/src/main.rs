use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

fn main() {
    let user = User {
        name: "Danylo".to_string(),
        email: "danil200319771@gmail.com".to_string(),
        birthdate: "2003-10-10".to_string(),
    };

    let json = serde_json::to_string(&user).expect("Serialize JSON failed");
    println!("Serialized JSON: {}", json);

    let deserialized_user: User = serde_json::from_str(&json).expect("Deserialize JSON failed");
    println!("Deserialized JSON: {:?}", deserialized_user);
}