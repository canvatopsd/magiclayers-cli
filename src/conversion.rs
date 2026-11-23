use crate::{arguments::ConversionArguments, client::ProductClient, task_access, task_id};
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub fn submit(input: ConversionArguments, client: &ProductClient, service: &str) -> Result<Value> {
    let ConversionArguments {
        file,
        print,
        print_settings,
        email,
        locale,
        task_id,
        dry_run,
    } = input;
    let spot_print: Option<Value> = if print {
        Some(match print_settings {
            Some(path) => serde_json::from_slice(&std::fs::read(path)?)?,
            None => {
                json!({"process":"spot-screen","substrate":"white","dpi":300,"palette":"auto"})
            }
        })
    } else {
        ensure!(
            print_settings.is_none(),
            "Print settings belong only to spot-print tasks"
        );
        None
    };
    let method = if print {
        "spot-separation".to_string()
    } else {
        "editable-text".to_string()
    };
    let id = task_id.unwrap_or_else(uuid::Uuid::now_v7);
    ensure!(
        id.get_version_num() == 7,
        "Task submissions require a UUIDv7 ID"
    );
    let media = match file
        .extension()
        .and_then(|s| s.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        _ => anyhow::bail!("Expected PNG, JPEG or WebP"),
    };
    ensure!(
        std::fs::metadata(&file)?.len() <= 30 * 1024 * 1024,
        "Image exceeds 30 MiB"
    );
    let bytes = std::fs::read(&file)?;
    ensure!(
        !bytes.is_empty() && bytes.len() <= 30 * 1024 * 1024,
        "Image must contain 1 byte to 30 MiB"
    );
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    let mut metadata =
        json!({"email":email,"locale":locale,"processingMethod":method,"sha256":sha256});
    if let Some(settings) = &spot_print {
        metadata["spotPrint"] = settings.clone();
    }
    if dry_run {
        return Ok(
            json!({"dryRun":true,"taskId":task_id::compact(id),"file":file,"metadata":metadata,"validation":"local arguments only; API validates metadata and image content"}),
        );
    }
    let access = task_access::prepare(task_access::TaskAccess {
        version: 1,
        task_id: id,
        service: service.to_string(),
        key: hex::encode(rand::random::<[u8; 32]>()),
        email,
        locale,
        method,
        spot_print,
        sha256,
    })?;
    let form = reqwest::blocking::multipart::Form::new()
        .part(
            "image",
            reqwest::blocking::multipart::Part::bytes(bytes)
                .file_name(
                    file.file_name()
                        .context("Image filename missing")?
                        .to_string_lossy()
                        .into_owned(),
                )
                .mime_str(media)?,
        )
        .text("metadata", metadata.to_string());
    let mut result = ProductClient::response(
        client
            .request("/api/v2/task/create", Some(id), None)?
            .header("Task-Access-Key", &access.key)
            .multipart(form),
    )
    .with_context(|| {
        format!("Task {id} submission failed; inspect or retry this same ID with identical input")
    })?;
    if let Some(object) = result.as_object_mut() {
        object.remove("privateTaskURL");
        object.insert("accessSaved".into(), json!(true));
    }
    Ok(result)
}
