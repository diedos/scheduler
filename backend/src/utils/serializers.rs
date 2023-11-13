use serde::{Serialize, Serializer};
use sqlx::types::chrono::NaiveDateTime;

pub fn serialize_dt<S>(dt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = dt {
        dt.format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}
