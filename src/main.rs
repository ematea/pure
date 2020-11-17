use ansi_term::Colour;
use git2::{Repository,Reference,Oid, DiffOptions};
use std::env;
use hostname;
use users;

fn get_prompt_pwd(path: &str) -> String {
    let home = env::var("HOME").unwrap();
    let path = path.replace(&home, "~");
    let dirs: Vec<&str> = path.split('/').collect();
    let len = dirs.len();
    let max_dirs = 4;
    if len <= max_dirs {
        return Colour::Fixed(8).paint(dirs.join(&"/")).to_string();
    }
    return Colour::Fixed(8).paint(
        [".../", &dirs[(len-max_dirs)..len].join(&"/")].concat()
        ).to_string();
}

fn get_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(_e) => None,
    };
    let head = head.as_ref().and_then(|h| h.shorthand());
    return Colour::Fixed(4).paint(head.unwrap_or("HEAD")).to_string();
}

fn get_status(repo: &Repository) -> Option<String> {
    let mut diff_opts = DiffOptions::new();
    diff_opts.ignore_submodules(true);
    diff_opts.skip_binary_check(true);

    let index = match repo.index() {
        Ok(index) => index,
        Err(_e) => return None,
    };
    let diff_w = repo.diff_index_to_workdir(Some(&index), Some(&mut diff_opts)).unwrap();
    let (size_w, _opt) = diff_w.deltas().size_hint();
    if size_w > 0 {
        return Some(Colour::Fixed(9).paint("*").to_string());
    }

    let head = match repo.head() {
        Ok(head) => head.peel_to_tree().unwrap(),
        Err(_e) => return None,
    };
    let diff_i = repo.diff_tree_to_index(Some(&head), Some(&index), Some(&mut diff_opts)).unwrap();
    let (size_i, _opt) = diff_i.deltas().size_hint();
    if size_i > 0 {
        return Some(Colour::Fixed(10).paint("*").to_string());
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

fn get_ssh_hostname() -> Option<String> {
    if let Ok(_ssh_tty) = env::var("SSH_TTY") {
        if let Ok(host) = hostname::get() {
            let host_str = host.into_string().unwrap();
            return Some(Colour::Fixed(8).paint(host_str).to_string());
        }
    }
    return None;
}

fn get_username() -> Option<String> {
    if let Some(username) = users::get_current_username() {
        if let Ok(username_str) = username.into_string() {
            return Some(Colour::Fixed(8).paint(username_str).to_string());
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
    if let Some(hostname) = get_ssh_hostname() {
        if let Some(username) = get_username() {
            print!(" {}{}{}", username, Colour::Fixed(8).paint("@"), hostname);
        }
    }
    print!("\n{}", Colour::Fixed(8).paint("❯ "));
}
