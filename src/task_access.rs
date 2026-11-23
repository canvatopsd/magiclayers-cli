use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct TaskAccess {
    pub version: u8,
    pub task_id: uuid::Uuid,
    pub service: String,
    pub key: String,
    pub email: String,
    pub locale: String,
    pub method: String,
    #[serde(default)]
    pub spot_print: Option<serde_json::Value>,
    pub sha256: String,
}

fn access_path(id: uuid::Uuid) -> Result<PathBuf> {
    let root = match std::env::var_os("MAGICLAYERS_ACCESS_DIRECTORY") {
        Some(path) => PathBuf::from(path),
        None => dirs::data_local_dir()
            .context("Cannot locate the local application data directory")?
            .join("magiclayers/tasks"),
    };
    Ok(root.join(format!("{id}.json")))
}

pub fn read(id: uuid::Uuid, service: &str) -> Result<TaskAccess> {
    let access: TaskAccess = serde_json::from_slice(&fs::read(access_path(id)?)
        .context("Task access is missing; restore the original access file rather than creating another payment")?)?;
    ensure!(
        access.version == 1 && access.task_id == id && access.service == service,
        "Saved task access belongs to another task or service"
    );
    ensure!(
        access.key.len() == 64
            && access
                .key
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "Saved task access key is invalid"
    );
    Ok(access)
}

pub fn prepare(input: TaskAccess) -> Result<TaskAccess> {
    let path = access_path(input.task_id)?;
    if path.exists() {
        let access = read(input.task_id, &input.service)?;
        ensure!(access.email == input.email && access.locale == input.locale && access.method == input.method && access.spot_print == input.spot_print && access.sha256 == input.sha256, "Task is bound to the original image, email, locale and method; inspect that task before changing input");
        return Ok(access);
    }
    let parent = path
        .parent()
        .context("Access file has no parent directory")?;
    let mut directory = fs::DirBuilder::new();
    directory.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        directory.mode(0o700);
    }
    directory.create(parent)?;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    use std::io::Write;
    let bytes = serde_json::to_vec_pretty(&input)?;
    let mut file = options.open(&path)?;
    let written = file.write_all(&bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = written {
        fs::remove_file(&path).context("Could not remove incomplete task access file")?;
        return Err(error).context("Could not save task access; no request was sent");
    }
    eprintln!(
        "Task {} access saved before submission: {}",
        input.task_id,
        path.display()
    );
    Ok(input)
}
