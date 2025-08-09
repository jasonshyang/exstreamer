use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RequestKind {
    Subscribe,
    Unsubscribe,
}

pub mod to_upper {
    use super::*;
    use serde::Serializer;

    pub fn serialize<S>(value: &RequestKind, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:?}", value).to_uppercase();
        s.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<RequestKind, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_uppercase().as_str() {
            "SUBSCRIBE" => Ok(RequestKind::Subscribe),
            "UNSUBSCRIBE" => Ok(RequestKind::Unsubscribe),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown request kind: {}",
                s
            ))),
        }
    }
}

pub mod to_lower {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        value: &crate::models::RequestKind,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:?}", value).to_lowercase();
        s.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<crate::models::RequestKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "subscribe" => Ok(crate::models::RequestKind::Subscribe),
            "unsubscribe" => Ok(crate::models::RequestKind::Unsubscribe),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown request kind: {}",
                s
            ))),
        }
    }
}
