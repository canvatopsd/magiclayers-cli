use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub fn parse(value: &str) -> Result<uuid::Uuid, String> {
    let id = if value.len() == 22 {
        let bytes = URL_SAFE_NO_PAD.decode(value).map_err(|e| e.to_string())?;
        let id = uuid::Uuid::from_slice(&bytes).map_err(|e| e.to_string())?;
        if compact(id) != value {
            return Err("Task ID is not canonical".into());
        }
        id
    } else {
        uuid::Uuid::parse_str(value).map_err(|e| e.to_string())?
    };
    if id.get_version_num() != 7 {
        return Err("Task ID must encode UUIDv7".into());
    }
    Ok(id)
}
pub fn compact(id: uuid::Uuid) -> String {
    URL_SAFE_NO_PAD.encode(id.as_bytes())
}
