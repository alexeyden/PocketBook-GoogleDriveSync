use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;

use crate::json::try_get_field;
use crate::{ApiConfig, ApiError};

const ERR_DOMAIN: &'static str = "auth";

pub struct AuthRequest<'a> {
    pub config: &'a ApiConfig,
    pub browser_command: String,
    pub response_port: u16,
    pub response_msg: String,
}

pub struct AuthToken {
    pub access: String,
    pub refresh: String,
}

pub fn auth_desktop(request: AuthRequest<'_>) -> Result<AuthToken, ApiError> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", request.response_port))?;

    let r = ureq::get("https://accounts.google.com/o/oauth2/v2/auth")
        .query("client_id", &request.config.client_id)
        .query(
            "redirect_uri",
            &format!("http://127.0.0.1:{}", request.response_port),
        )
        .query("response_type", "code")
        .query("scope", "https://www.googleapis.com/auth/drive");

    let url = r.url();

    let mut child = std::process::Command::new(request.browser_command)
        .arg(url)
        .spawn()?;

    let (socket, _) = listener.accept()?;

    let mut reader = std::io::BufReader::new(socket);
    let mut s = String::new();
    reader.read_line(&mut s)?;

    let url = s
        .split(' ')
        .nth(1)
        .ok_or_else(|| ApiError::custom("invalid response format", ERR_DOMAIN))?;

    let url = url::Url::parse(&format!("http://127.0.0.1{}", url))
        .map_err(|e| ApiError::custom(format!("malformed url: {}", e), ERR_DOMAIN))?;

    let code = url
        .query_pairs()
        .find(|(name, _)| name == "code")
        .ok_or_else(|| ApiError::custom("no code in auth response", ERR_DOMAIN))?
        .1
        .into_owned();

    let mut sock = reader.into_inner();
    write!(
        sock,
        "HTTP/1.1 200 OK\n\
        Content-Length: {}\n\
        Content-Type: text/plain; charset=utf-8\n\
        \n\
        {}",
        request.response_msg.len(),
        request.response_msg
    )?;

    child.wait()?;

    let r = ureq::post("https://oauth2.googleapis.com/token")
        .send_form(&[
            ("client_id", &request.config.client_id),
            ("client_secret", &request.config.client_secret),
            ("code", &code),
            ("grant_type", "authorization_code"),
            (
                "redirect_uri",
                &format!("http://127.0.0.1:{}", request.response_port),
            ),
        ])?
        .into_string()?;

    let v: tinyjson::JsonValue = r.parse()?;

    let root = v
        .get::<HashMap<_, _>>()
        .ok_or_else(|| ApiError::custom("no root object", ERR_DOMAIN))?;

    let access = try_get_field::<String>(root, "access_token", ERR_DOMAIN)?.clone();
    let refresh = try_get_field::<String>(root, "refresh_token", ERR_DOMAIN)?.clone();

    Ok(AuthToken { access, refresh })
}

pub fn refresh_access_token(config: &ApiConfig) -> Result<AuthToken, ApiError> {
    let r = ureq::post("https://oauth2.googleapis.com/token")
        .send_form(&[
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
            ("grant_type", "refresh_token"),
            ("refresh_token", &config.refresh_token),
        ])?
        .into_string()?;

    let v: tinyjson::JsonValue = r.parse()?;

    let root = v
        .get::<HashMap<_, _>>()
        .ok_or_else(|| ApiError::custom("no root object", ERR_DOMAIN))?;

    let access = try_get_field::<String>(root, "access_token", ERR_DOMAIN)?.clone();

    let token = AuthToken {
        access,
        refresh: config.refresh_token.clone(),
    };

    Ok(token)
}
