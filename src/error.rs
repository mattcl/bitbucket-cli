use std::env;
use std::io;
use url;

use clap;
use eprompt;
use git2;
use hyper;
use serde_json;
use yaml_rust::ScanError;

error_chain! {
    links {
        EPrompt(eprompt::Error, eprompt::ErrorKind);
    }

    foreign_links {
        VarError(env::VarError);
        IoError(io::Error);
        GitError(git2::Error);
        UrlParseError(url::ParseError);
        HyperError(hyper::Error);
        YamlScanError(ScanError);
        SerdeJsonError(serde_json::Error);
    }

    errors {
        DryRun {
            description("dry run not a real error")
            display("dry run not a real error")
        }
        RepoEmpty {
            description("repo is empty")
            display("repo is empty")
        }
        InvalidConfig(t: String) {
            description("invalid config file")
            display("invalid config file. missing {}", t)
        }
        InvalidReference {
            description("git reference is invalid")
            display("git reference is invalid")
        }
        RequestError(response: String) {
            description("request error")
            display("request error. response: {}", response)
        }
        MissingSelfLink {
            description("response missing self link")
            display("response missing self link")
        }
        MissingSubcommand(command: String) {
            description("missing subcommand")
            display("missing subcommand: {}", command)
        }
        ProjectNotFound(project: String) {
            description("project not found")
            display("project not found: {}", project)
        }
        GroupNotFound(group: String) {
            description("group not found")
            display("group not found: {}", group)
        }
        InvalidPullRequest(reason: String) {
            description("invalid pull request")
            display("invalid pull request: {}", reason)
        }
        TargetBranchExists(branch: String) {
            description("The current branch already exists on the target")
            display("The current branch '{}' already exists on the target", branch)
        }
    }
}

pub trait UnwrapOrExit<T>
where
    Self: Sized,
{
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T;

    fn unwrap_or_exit(self, message: &str) -> T {
        let err = clap::Error::with_description(message, clap::ErrorKind::InvalidValue);
        self.unwrap_or_else(|| err.exit())
    }
}

impl<T> UnwrapOrExit<T> for Option<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.unwrap_or_else(f)
    }
}

impl<T> UnwrapOrExit<T> for Result<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.unwrap_or_else(|_| f())
    }

    fn unwrap_or_exit(self, message: &str) -> T {
        self.unwrap_or_else(|e| {
            let err = clap::Error::with_description(
                &format!("{}: {}", message, e),
                clap::ErrorKind::InvalidValue,
            );
            err.exit()
        })
    }
}
