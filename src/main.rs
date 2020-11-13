use ansi_term::Colour;
use git2::{StatusOptions,Repository,Reference,Oid};
use std::env;

fn get_prompt_pwd(path: &str) -> String {
    let home = env::var("HOME").unwrap();
    let path = path.replace(&home, "~");
    let dirs: Vec<&str> = path.split('/').collect();
    let len = dirs.len();
    let max_dirs = 4;
    if len <= max_dirs {
        return Colour::Fixed(4).paint(dirs.join(&"/")).to_string();
    }
    return Colour::Fixed(4).paint(
        [".../", &dirs[(len-max_dirs)..len].join(&"/")].concat()
        ).to_string();
}

fn get_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(_e) => None,
    };
    let head = head.as_ref().and_then(|h| h.shorthand());
    return Colour::Fixed(8).paint(head.unwrap_or("HEAD")).to_string();
}

fn get_status(repo: &Repository) -> Option<String> {
    let index_statuses = [
        git2::Status::INDEX_NEW,
        git2::Status::INDEX_MODIFIED,
        git2::Status::INDEX_DELETED,
        git2::Status::INDEX_RENAMED,
        git2::Status::INDEX_TYPECHANGE,
    ];
    let working_directory_statuses = [
        git2::Status::WT_MODIFIED,
        git2::Status::WT_DELETED,
        git2::Status::WT_RENAMED,
        git2::Status::WT_TYPECHANGE,
    ];
    if let Ok(statuses) = repo.statuses(Some(&mut StatusOptions::new())) {
        match statuses.iter().find(|entry| working_directory_statuses.contains(&entry.status())) {
            Some(_status) => {
                return Some(Colour::Fixed(9).paint("*").to_string());
            },
            None => {
                if let Some(_status) = statuses.iter().find(|entry| index_statuses.contains(&entry.status())) {
                    return Some(Colour::Fixed(10).paint("*").to_string());
                }
            }
        };
    }

    return None;
}

fn get_ahead_behind(repo: &Repository) -> Option<String> {
    let head: Reference = match repo.head() {
        Ok(head) => head,
        Err(_e) => return None,
    };
    let oid: Oid = match head.target() {
        Some(oid) => oid,
        None => return None,
    };
    let upstream_name = match repo.branch_upstream_name(head.name().unwrap()) {
        Ok(buf) => buf.as_str().unwrap().to_string(),
        Err(_e) => return None,
    };
    let upstream_oid = match repo.find_reference(&upstream_name) {
        Ok(upstream) => upstream.target().unwrap(),
        Err(_e) => return None,
    };
    if let Ok(r) = repo.graph_ahead_behind(oid, upstream_oid) {
        let (ahead, behind): (usize, usize) = r;
        if ahead != 0 && behind != 0 {
            return Some(Colour::Fixed(6).paint("⇡⇣").to_string());
        }
        if ahead != 0 {
            return Some(Colour::Fixed(6).paint("⇡").to_string());
        }
        if behind != 0 {
            return Some(Colour::Fixed(6).paint("⇣").to_string());
        }
    }
    return None;
}

fn main() {
    let cwd = env::var("PWD").unwrap();
    print!("\n{}", get_prompt_pwd(&cwd));

    if let Ok(repo) = Repository::open(&cwd) {
        print!(" {}", get_branch(&repo));
        if let Some(status) = get_status(&repo) {
            print!("{}", status);
        }
        if let Some(ahead_behind) = get_ahead_behind(&repo) {
            print!(" {}", ahead_behind);
        }
    }

    print!("\n{}", Colour::Fixed(8).paint("❯ "));
}
