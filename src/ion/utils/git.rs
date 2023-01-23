use super::*;
use anyhow::{format_err, Result};
use std::path::PathBuf;
use std::process::{Command, Output};

// `git show -s --format="%H" $branch`
pub fn sha_256(path: &PathBuf, branch: &str) -> Result<String> {
    Command::new("git")
        .arg("show")
        .arg("-s")
        .arg("--format=%H")
        .arg(branch)
        .current_dir(path)
        .read_command()
}

pub fn remote_repo(local: &PathBuf) -> Result<(String, String)> {
    let url = remote_origin_url(local)?;
    let url = url.trim_end_matches(".git");
    let url = url.trim_end_matches("/");
    let url = url.trim_start_matches("https://");

    // https://github.com/QuEraComputing/Bloqade.jl
    let mut parts = url.split("/");
    parts.next();
    let name = parts.next()
        .ok_or_else(|| format_err!("Invalid remote url"))?;
    let owner = parts.next()
        .ok_or_else(|| format_err!("Invalid remote url"))?;

    Ok((owner.to_string(), name.to_string()))
}

pub fn remote_origin_url(repo: &PathBuf) -> Result<String> {
    remote_repo_url(repo, "origin")
}

pub fn remote_repo_url(repo: &PathBuf, remote: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg(format!("remote.{}.url", remote))
        .current_dir(repo)
        .read_command()?;
    Ok(output)
}

pub fn get_toplevel_path(path: &PathBuf) -> Result<PathBuf> {
    let raw = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .current_dir(path)
        .read_command()?;
    let path = PathBuf::from(raw);
    Ok(normalize_path(path.as_path()))
}

pub fn current_branch(path: &PathBuf) -> Result<String> {
    Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(path)
        .read_command()
}

pub fn default_branch(path: &PathBuf) -> Result<String> {
    // git symbolic-ref refs/remotes/origin/HEAD
    let refs = Command::new("git")
        .arg("symbolic-ref")
        .arg("refs/remotes/origin/HEAD")
        .current_dir(path)
        .read_command()?;
    // refs/remotes/origin/master
    let mut parts = refs.split("/");
    let default = parts
        .last()
        .ok_or(format_err!("Invalid default branch"))?
        .to_string();
    Ok(default)
}

pub fn isdirty(path: &PathBuf) -> Result<bool> {
    let p = Command::new("git")
        .arg("diff")
        .arg("--quiet")
        .arg("--exit-code")
        .current_dir(path)
        .status()?;
    Ok(!p.success())
}

pub fn isdirty_cached(path: &PathBuf) -> Result<bool> {
    let p = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--quiet")
        .arg("--exit-code")
        .current_dir(path)
        .status()?;
    Ok(!p.success())
}

pub fn commit(path: &PathBuf, msg: &str) -> Result<Output> {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(msg)
        .current_dir(path)
        .output()?;

    if output.status.success() {
        Ok(output)
    } else {
        return Err(format_err!("Failed to commit"));
    }
}

pub fn pull(path: &PathBuf) -> Result<Output> {
    let output = Command::new("git").arg("pull").current_dir(path).output()?;

    if output.status.success() {
        Ok(output)
    } else {
        return Err(format_err!("Failed to pull"));
    }
}

pub fn push(path: &PathBuf) -> Result<Output> {
    let output = Command::new("git").arg("push").current_dir(path).output()?;

    if output.status.success() {
        Ok(output)
    } else {
        return Err(format_err!("Failed to push"));
    }
}
