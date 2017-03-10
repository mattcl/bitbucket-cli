use std::env;
use std::path::PathBuf;
use git2::Repository;
use error::{ErrorKind, Result};

fn repository() -> Result<Repository> {
    let current_dir = env::current_dir()?;
    let repo = Repository::discover(current_dir)?;
    Ok(repo)
}

pub fn repo_dir() -> Result<PathBuf> {
    let repo = repository()?;
    match repo.workdir() {
        Some(path) => Ok(path.to_owned()),
        None => Err(ErrorKind::RepoEmpty.into()),
    }
}

pub fn current_branch() -> Result<String> {
    let repo = repository()?;
    let head = repo.head()?;
    match head.shorthand() {
        Some(name) => Ok(name.to_string()),
        None => Err(ErrorKind::InvalidReference.into()),
    }
}

pub fn repo_name() -> Result<String> {
    if let Some(name) = repo_dir()?.file_name() {
        if let Some(name_str) = name.to_str() {
            return Ok(name_str.to_string());
        }
    }
    Err(ErrorKind::RepoEmpty.into())
}

pub fn commit_summary() -> Result<String> {
    let repo = repository()?;
    let head = repo.head()?;
    let mut commit = match head.target() {
        Some(oid) => repo.find_commit(oid)?,
        None => return Err(ErrorKind::InvalidReference.into())
    };
    match commit.summary() {
        Some(msg) => Ok(msg.to_string()),
        None => Err(ErrorKind::InvalidReference.into())
    }
}

pub fn commit_message() -> Result<String> {
    let repo = repository()?;
    let head = repo.head()?;
    let mut commit = match head.target() {
        Some(oid) => repo.find_commit(oid)?,
        None => return Err(ErrorKind::InvalidReference.into())
    };
    match commit.summary() {
        Some(msg) => Ok(msg.to_string()),
        None => Err(ErrorKind::InvalidReference.into())
    }
}
