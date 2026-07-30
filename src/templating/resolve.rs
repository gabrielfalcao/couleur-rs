use std::path::PathBuf;

use chrono::Utc;
use git2::Repository;
use iocore::Path;
use pest::iterators::Pair;
use sanitation::SString;


pub fn git_branch() -> String {
    match Repository::discover::<PathBuf>(Path::cwd().to_path_buf()) {
        Ok(repo) => match repo.head() {
            Ok(head) => {
                format!(
                    "[git:{}]",
                    SString::new(head.name_bytes())
                        .unchecked_safe()
                        .replace("refs/heads/", "")
                        .to_string()
                )
            },
            Err(_) => {
                format!("git[no commits]")
            },
        },
        Err(_) => String::new(),
    }
}

pub fn cwd_name() -> String {
    Path::cwd().name()
}
