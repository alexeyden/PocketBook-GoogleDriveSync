use std::collections::HashMap;

mod auth;
mod config;
mod error;
mod json;

pub use self::config::*;
pub use self::error::*;

use self::json::try_get_field;

#[derive(Clone, Debug)]
pub struct DriveFile {
    pub id: String,
    pub name: String,
    pub size: Option<u64>,
}

pub struct FileDownload {
    output: std::fs::File,
    input: Box<dyn std::io::Read + Send + Sync>,
    buf: Vec<u8>,
    n: u64,
}

impl FileDownload {
    pub fn next_chunk(&mut self) -> Result<bool, ApiError> {
        use std::io::prelude::*;

        let l = self.input.read(&mut self.buf)?;

        if l == 0 {
            return Ok(true);
        }

        self.n += l as u64;

        self.output.write(&self.buf[0..l])?;

        Ok(false)
    }

    pub fn ready(&self) -> u64 {
        self.n
    }
}

pub struct GoogleDriveApi {
    config: ApiConfig,
}

impl GoogleDriveApi {
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &ApiConfig {
        &self.config
    }

    pub fn auth(&mut self) -> Result<(), ApiError> {
        let req = self::auth::AuthRequest {
            config: &self.config,
            browser_command: "xdg-open".to_owned(),
            response_port: 8066,
            response_msg: "Auth tokens have been received, you may close the browser now"
                .to_owned(),
        };
        let token = self::auth::auth_desktop(req)?;

        self.config.access_token = token.access;
        self.config.refresh_token = token.refresh;

        Ok(())
    }

    pub fn auth_refresh(&mut self) -> Result<(), ApiError> {
        let token = self::auth::refresh_access_token(&self.config)?;
        self.config.access_token = token.access;

        Ok(())
    }

    pub fn list_files(&self, q: &str) -> Result<Vec<DriveFile>, ApiError> {
        const ERR_DOMAIN: &'static str = "files api";

        let r = ureq::get("https://www.googleapis.com/drive/v3/files")
            .set(
                "Authorization",
                &format!("Bearer {}", self.config.access_token),
            )
            .query("q", q)
            .query("fields", "files(id,name,size)")
            .call()?
            .into_string()?;

        let v: tinyjson::JsonValue = r.parse()?;

        let root = v
            .get::<HashMap<_, _>>()
            .ok_or_else(|| ApiError::custom("no root object", ERR_DOMAIN))?;

        let files = try_get_field::<Vec<_>>(root, "files", ERR_DOMAIN)?.clone();
        let mut out = vec![];
        for f in files {
            let f = f
                .get::<HashMap<_, _>>()
                .ok_or_else(|| ApiError::custom("invalid file object", ERR_DOMAIN))?;
            let name = try_get_field::<String>(f, "name", ERR_DOMAIN)?.clone();
            let id = try_get_field::<String>(f, "id", ERR_DOMAIN)?.clone();
            let size = f
                .get("size")
                .and_then(|f| f.get::<String>())
                .and_then(|s| s.parse::<u64>().ok());

            out.push(DriveFile { name, id, size });
        }

        Ok(out)
    }

    pub fn start_download(
        &self,
        id: &str,
        path: &std::path::Path,
    ) -> Result<FileDownload, ApiError> {
        let r = ureq::get(&format!("https://www.googleapis.com/drive/v3/files/{}", id))
            .set(
                "Authorization",
                &format!("Bearer {}", self.config.access_token),
            )
            .query("alt", "media")
            .call()?;

        let buf = vec![0; 4096];
        let f = std::fs::File::create(path)?;

        let dl = FileDownload {
            input: r.into_reader(),
            output: f,
            buf,
            n: 0,
        };

        Ok(dl)
    }
}
