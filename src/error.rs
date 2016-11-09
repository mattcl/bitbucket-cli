use std::env;
use std::io;

use git2;
use eprompt;

error_chain! {
    links {
        eprompt::Error, eprompt::ErrorKind, EPrompt;
    }

    foreign_links {
        env::VarError, VarError;
        io::Error, IoError;
        git2::Error, GitError;
    }

    errors {
        RepoEmpty {
            description("repo is empty")
            display("repo is empty")
        }
    }
}
