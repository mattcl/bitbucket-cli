use std::env;
use std::path::Path;
use git2::Repository;
use error::{ErrorKind, Result};

pub fn repo_dir() -> Result<&Path> {
    let current_dir = try!(env::current_dir());
    let repo = try!(Repository::discover(current_dir));
    match repo.workdir() {
        Some(path) => Ok(path),
        None => Err(ErrorKind::RepoEmpty.into())
    }
}
