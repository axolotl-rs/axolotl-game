pub mod world;

macro_rules! get_type {
    ($map:expr) => {
        if let Some((key, value)) = $map.next_entry::<String, OwnedNameSpaceKey>()? {
            if key.eq("type") {
                value
            } else {
                return Err(serde::de::Error::custom(format!(
                    "Expected `type` key, got `{}`",
                    key
                )));
            }
        } else {
            return Err(serde::de::Error::custom("Expected `type` key, got nothing"));
        }
    };
}
pub(crate) use get_type;
