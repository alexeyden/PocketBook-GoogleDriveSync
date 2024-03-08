use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ApiError {
    source: Box<dyn Error>,
    domain: &'static str,
}

#[derive(Debug)]
struct StringError(String);

impl Error for StringError {}

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ApiError {
    pub fn wrap(source: impl Error + 'static, domain: &'static str) -> Self {
        Self { source: Box::new(source), domain }
    }

    pub fn custom(msg: impl Into<String>, domain: &'static str) -> Self {
        Self { source: Box::new(StringError(msg.into())), domain }
    }

    pub fn as_http_status(&self) -> Option<u16> {
        match self.source.downcast_ref::<ureq::Error>()? {
            ureq::Error::Status(code, _) => Some(*code),
            _ => None
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.domain, self.source)
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(source: std::io::Error) -> Self {
        Self::wrap(source, "i/o")
    }
}

impl From<ureq::Error> for ApiError {
    fn from(source: ureq::Error) -> Self {
        Self::wrap(source, "http")
    }
}

impl From<tinyjson::JsonParseError> for ApiError {
    fn from(source: tinyjson::JsonParseError) -> Self {
        Self::wrap(source, "json")
    }
}
