use std::result;
use std::env;
use std::io;
use url;

use clap;
use eprompt;
use git2;
use hyper;
use rustc_serialize::json::{EncoderError, ParserError};
use yaml_rust::ScanError;

error_chain! {
    links {
        eprompt::Error, eprompt::ErrorKind, EPrompt;
    }

    foreign_links {
        env::VarError, VarError;
        io::Error, IoError;
        git2::Error, GitError;
        url::ParseError, UrlParseError;
        EncoderError, EncoderError;
        ParserError, ParserError;
        hyper::Error, HyperError;
        ScanError, YamlScanError;
    }

    errors {
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
        RequestError(t: String) {
            description("request error")
            display("request error. response: {}", t)
        }
        MissingSelfLink {
            description("response missing self link")
            display("response missing self link")
        }
    }
}

pub trait UnwrapOrExit<T>
    where Self: Sized
{
    fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T;

    fn unwrap_or_exit(self, message: &str) -> T {
        let err = clap::Error::with_description(message, clap::ErrorKind::InvalidValue);
        self.unwrap_or_else(|| err.exit())
    }
}

impl<T> UnwrapOrExit<T> for Option<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
        where F: FnOnce() -> T
    {
        self.unwrap_or_else(f)
    }
}

impl<T> UnwrapOrExit<T> for Result<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
        where F: FnOnce() -> T
    {
        self.unwrap_or_else(|_| f())
    }

    fn unwrap_or_exit(self, message: &str) -> T {
        self.unwrap_or_else(|e| {
            let err = clap::Error::with_description(&format!("{}: {}", message, e), clap::ErrorKind::InvalidValue);
            err.exit()
        })
    }
}
