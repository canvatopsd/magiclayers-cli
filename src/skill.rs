use crate::arguments::{SkillCommand, SkillTarget};
use anyhow::{ensure, Context, Result};
use serde_json::{json, Value};
use std::{fs, io::Write, path::PathBuf};

const CONTENT: &str = include_str!("../skills/magiclayers/SKILL.md");

fn destination(target: &SkillTarget) -> Result<PathBuf> {
    let parent = match &target.dir {
        Some(path) => std::path::absolute(path)?,
        None => {
            let home = dirs::home_dir().context("Cannot find home directory; use --dir")?;
            match target.agent.as_str() {
                "codex" => std::env::var_os("CODEX_HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| home.join(".codex"))
                    .join("skills"),
                "claude-code" => home.join(".claude/skills"),
                "cursor" => home.join(".cursor/skills"),
                "copilot" => home.join(".copilot/skills"),
                "agents" => home.join(".agents/skills"),
                _ => anyhow::bail!("Unknown Agent; use --dir"),
            }
        }
    };
    Ok(parent.join("magiclayers"))
}

pub fn run(command: &SkillCommand) -> Result<Value> {
    let (target, install) = match command {
        SkillCommand::Show => return Ok(json!({"content": CONTENT})),
        SkillCommand::Install(target) => (target, true),
        SkillCommand::Status(target) => (target, false),
    };
    let directory = destination(target)?;
    match fs::symlink_metadata(&directory) {
        Ok(metadata) => ensure!(
            metadata.is_dir(),
            "Skill directory must not be a symlink or file"
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    let path = directory.join("SKILL.md");
    let current = match fs::symlink_metadata(&path) {
        Ok(metadata) => {
            ensure!(metadata.is_file(), "Skill entry must be a regular file");
            Some(fs::read(&path)? == CONTENT.as_bytes())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    if install && current != Some(true) {
        ensure!(
            current.is_none(),
            "Skill differs; review existing edits before removing it and reinstalling"
        );
        fs::create_dir_all(&directory)?;
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?
            .write_all(CONTENT.as_bytes())?;
    }
    Ok(
        json!({"path":path,"installed":install || current.is_some(),"current":install || current == Some(true),"version":env!("CARGO_PKG_VERSION"),"next":"Start a new Agent session after installation"}),
    )
}
