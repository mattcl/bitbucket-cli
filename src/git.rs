use std::env;
use std::path::PathBuf;
use git2::Repository;
use error::{ErrorKind, Result};

fn repository() -> Result<Repository> {
    let current_dir = try!(env::current_dir());
    let repo = try!(Repository::discover(current_dir));
    Ok(repo)
}

pub fn repo_dir() -> Result<PathBuf> {
    let repo = try!(repository());
    match repo.workdir() {
        Some(path) => Ok(path.to_owned()),
        None => Err(ErrorKind::RepoEmpty.into())
    }
}

pub fn current_branch() -> Result<String> {
    let repo = try!(repository());
    let head = try!(repo.head());
    match head.shorthand() {
        Some(name) => Ok(name.to_string()),
        None => Err(ErrorKind::InvalidReference.into())
    }
}

pub fn repo_name() -> Result<String> {
    if let Some(name) = try!(repo_dir()).file_name() {
        if let Some(name_str) = name.to_str() {
            return Ok(name_str.to_string())
        }
    }
    Err(ErrorKind::RepoEmpty.into())
}
