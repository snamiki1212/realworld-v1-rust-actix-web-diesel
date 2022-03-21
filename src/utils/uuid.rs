use crate::error::AppError;
use uuid::Uuid;

pub fn parse(maybe_uuid: &str) -> Result<Uuid, AppError> {
    let uuid = Uuid::parse_str(maybe_uuid)?;
    Ok(uuid)
}
