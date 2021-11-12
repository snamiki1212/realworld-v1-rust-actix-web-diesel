use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Iso8601(pub chrono::NaiveDateTime);

impl Serialize for Iso8601 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0.format("%Y-%m-%dT%H:%M:%S.%3fZ");
        serializer.serialize_str(&s.to_string())
    }
}
