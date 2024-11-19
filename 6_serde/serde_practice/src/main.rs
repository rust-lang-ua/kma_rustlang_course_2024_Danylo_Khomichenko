use serde::{Serializer, Deserializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

fn serialize_date<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("Date super custom: {}", date))
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.replace("Date super custom: ", ""))
}

fn main() {
    let event = Event {
        name: "Event1".to_string(),
        date: "2024-10-10".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Serialization error");
    println!("Serialized JSON with custom data: {}", json);

    let deserialized_event: Event = serde_json::from_str(&json).expect("Deserialization error");
    println!("Serialized JSON with custom data: {:?}", deserialized_event);
}