use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toad_core::{ToadError, ToadResult};

pub struct ProjectConfig<'a> {
    pub name: &'a str,
    pub root_dir: PathBuf,
    pub dry_run: bool,
}

pub fn create_project(config: ProjectConfig) -> ToadResult<()> {
    let project_path = config.root_dir.join(config.name);

    if project_path.exists() {
        return Err(ToadError::OperationFailed(format!(
            "Project directory already exists: {:?}",
            project_path
        )));
    }

    if config.dry_run {
        return Ok(());
    }

    fs::create_dir_all(project_path.join("docs"))?;

    let readme_content = format!("# {}\n\n## Overview", config.name);
    fs::write(project_path.join("README.md"), readme_content)?;

    let gitignore_content = "target/\n.DS_Store\n.env\n";
    fs::write(project_path.join(".gitignore"), gitignore_content)?;

    init_git(&project_path)?;

    Ok(())
}

fn init_git(path: &Path) -> ToadResult<()> {
    let status = Command::new("git").arg("init").current_dir(path).status()?;

    if !status.success() {
        return Err(ToadError::Git(format!(
            "Git init failed with status: {}",
            status
        )));
    }
    Ok(())
}

pub fn open_in_editor(project_name: &str, root_dir: &Path, editor: &str) -> ToadResult<()> {
    let project_path = root_dir.join(project_name);

    let command = match editor {
        "vscode" => "code",
        "windsurf" => "windsurf",
        _ => return Err(ToadError::Config(format!("Unknown editor: {}", editor))),
    };

    let status = Command::new(command)
        .arg(".")
        .current_dir(project_path)
        .status()
        .map_err(|e| {
            ToadError::OperationFailed(format!(
                "Failed to launch {}. Is it in your PATH? {}",
                command, e
            ))
        })?;

    if !status.success() {
        return Err(ToadError::OperationFailed(
            "Editor launch failed".to_string(),
        ));
    }

    Ok(())
}
