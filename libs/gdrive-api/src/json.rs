use crate::ApiError;

pub fn try_get_field<'a, T: tinyjson::InnerAsRef>(
    root: &'a std::collections::HashMap<String, tinyjson::JsonValue>,
    name: &str,
    err_domain: &'static str,
) -> Result<&'a T, ApiError> {
    let f = root
        .get(name)
        .ok_or_else(|| ApiError::custom(format!("no '{}' field", name), err_domain))?
        .get::<T>()
        .ok_or_else(|| {
            ApiError::custom(format!("invalid format of `{}` field", name), err_domain)
        })?;

    Ok(f)
}
