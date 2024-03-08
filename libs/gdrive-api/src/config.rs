use std::collections::HashMap;
use std::io::prelude::*;

use crate::json::try_get_field;
use crate::ApiError;

#[derive(Debug)]
pub struct ExportedConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl ExportedConfig {
    pub fn import(path: impl AsRef<std::path::Path>) -> Result<Self, ApiError> {
        const ERR_DOMAIN: &'static str = "config import";

        let mut f = std::fs::File::open(path.as_ref())?;
        let mut s = String::new();

        f.read_to_string(&mut s)?;

        let v: tinyjson::JsonValue = s.parse()?;

        let root = v
            .get::<HashMap<_, _>>()
            .ok_or_else(|| ApiError::custom("no root object", ERR_DOMAIN))?;

        let installed = try_get_field::<HashMap<_, _>>(root, "installed", ERR_DOMAIN)?;
        let client_id = try_get_field::<String>(installed, "client_id", ERR_DOMAIN)?;
        let client_secret = try_get_field::<String>(installed, "client_secret", ERR_DOMAIN)?;

        Ok(Self {
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
        })
    }
}

#[derive(Debug)]
pub struct ApiConfig {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl ApiConfig {
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ApiError> {
        const ERR_DOMAIN: &'static str = "config load";

        let mut f = std::fs::File::open(path.as_ref())?;
        let mut s = String::new();

        f.read_to_string(&mut s)?;

        let v: tinyjson::JsonValue = s.parse()?;

        let root = v
            .get::<HashMap<_, _>>()
            .ok_or_else(|| ApiError::custom("no root object", ERR_DOMAIN))?;

        let client_id = try_get_field::<String>(root, "client_id", ERR_DOMAIN)?.clone();
        let client_secret = try_get_field::<String>(root, "client_secret", ERR_DOMAIN)?.clone();
        let access_token = try_get_field::<String>(root, "access_token", ERR_DOMAIN)?.clone();
        let refresh_token = try_get_field::<String>(root, "refresh_token", ERR_DOMAIN)?.clone();

        Ok(Self {
            client_id,
            client_secret,
            access_token,
            refresh_token,
        })
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), ApiError> {
        let mut f = std::fs::File::create(path.as_ref())?;

        writeln!(f, "{{")?;
        writeln!(f, r#" "client_id": "{}","#, self.client_id)?;
        writeln!(f, r#" "client_secret": "{}","#, self.client_secret)?;
        writeln!(f, r#" "access_token": "{}","#, self.access_token)?;
        writeln!(f, r#" "refresh_token": "{}""#, self.refresh_token)?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl From<ExportedConfig> for ApiConfig {
    fn from(value: ExportedConfig) -> Self {
        Self {
            client_id: value.client_id,
            client_secret: value.client_secret,
            access_token: Default::default(),
            refresh_token: Default::default(),
        }
    }
}
