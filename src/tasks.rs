use crate::{client::ProductClient, delivery, task_access};
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::path::Path;
use uuid::Uuid;

pub fn status(client: &ProductClient, service: &str, id: Uuid) -> Result<Value> {
    let access = task_access::read(id, service)?;
    client.post("/api/v2/task/read", json!({"taskId":id}), Some(&access.key))
}

pub fn download(
    client: &ProductClient,
    service: &str,
    id: Uuid,
    kind: &str,
    output: &Path,
) -> Result<Value> {
    let access = task_access::read(id, service)?;
    let response = client.post(
        "/api/v2/task/delivery/read",
        json!({"taskId":id}),
        Some(&access.key),
    )?;
    let file = response
        .get("files")
        .and_then(|files| files.get(kind))
        .with_context(|| {
            format!("Delivery has no {kind} file; inspect task status before downloading")
        })?;
    delivery::save(file, output)
}
