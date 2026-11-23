use anyhow::{ensure, Context, Result};
use reqwest::blocking::{Client, RequestBuilder};
use serde_json::Value;
use std::time::Duration;
use url::Url;

#[derive(Debug)]
pub struct ApiFailure {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for ApiFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "HTTP {} {}: {}",
            self.status, self.code, self.message
        )
    }
}
impl std::error::Error for ApiFailure {}

pub struct ProductClient {
    client: Client,
    base: Url,
}

impl ProductClient {
    pub fn new(base: &str) -> Result<Self> {
        let base = Url::parse(base).context("Invalid base URL")?;
        ensure!(
            base.username().is_empty()
                && base.password().is_none()
                && base.query().is_none()
                && base.fragment().is_none()
                && base.path() == "/",
            "Base URL must be an origin without credentials, path, query or fragment"
        );
        ensure!(
            base.scheme() == "https"
                || (base.scheme() == "http"
                    && matches!(base.host_str(), Some("127.0.0.1" | "localhost" | "[::1]"))),
            "Use HTTPS or local loopback HTTP"
        );
        Ok(Self {
            base,
            client: Client::builder()
                .timeout(Duration::from_secs(90))
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
        })
    }

    pub fn origin(&self) -> String {
        self.base.origin().ascii_serialization()
    }

    pub fn request(
        &self,
        path: &str,
        request_id: Option<uuid::Uuid>,
        task_key: Option<&str>,
    ) -> Result<RequestBuilder> {
        ensure!(
            path.starts_with("/api/v2/task/")
                && !path.contains(['?', '#', '%', '\\'])
                && !path.split('/').any(|p| p == ".." || p == "."),
            "Expected a guest /api/v2/task/ command path"
        );
        let mut request = self
            .client
            .post(self.base.join(path)?)
            .header("Accept", "application/json")
            .header("Origin", self.base.origin().ascii_serialization());
        if let Some(key) = task_key {
            request = request.header("Authorization", format!("Task {key}"));
        }
        if let Some(id) = request_id {
            request = request.header("Request-UUID", id.to_string());
        }
        Ok(request)
    }

    pub fn response(request: RequestBuilder) -> Result<Value> {
        let response = request.send().context("Could not reach MagicLayers")?;
        let status = response.status();
        let body: Value = response
            .json()
            .context("Service returned a non-JSON response")?;
        if !status.is_success() {
            let code = body["error"]["code"]
                .as_str()
                .context("API error response missing code")?;
            let message = body["error"]["message"]
                .as_str()
                .context("API error response missing message")?;
            return Err(ApiFailure {
                status: status.as_u16(),
                code: code.into(),
                message: message.into(),
            }
            .into());
        }
        Ok(body)
    }

    pub fn post(&self, path: &str, body: Value, task_key: Option<&str>) -> Result<Value> {
        Self::response(self.request(path, None, task_key)?.json(&body))
    }
}
