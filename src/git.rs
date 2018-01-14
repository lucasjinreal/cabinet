extern crate git2;
extern crate time;


use git2::{Repository, Commit, ObjectType, Oid, Signature, Direction};
use std::path::Path;
use std::fs;

//pub fn print_submodules(repo: &Repository) -> Result<(), Error> {
//    let modules = repo.submodules()?;
//    println!("# Submodules");
//    for sm in &modules {
//        println!("# - submodule '{}' at {}", sm.name().unwrap(),
//                 sm.path().display());
//    }
//    Ok(())
//}


pub fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

pub fn display_commit(commit: &Commit) {
    let timestamp = commit.time().seconds();
    let tm = time::at(time::Timespec::new(timestamp, 0));
    println!("commit {}\nAuthor: {}\nDate:   {}\nMsg:    {}",
             commit.id(),
             commit.author(),
             tm.rfc822(),
             commit.message().unwrap_or("no commit message"));
}

pub fn add_and_commit(repo: &Repository, path: &Path, message: &str) -> Result<Oid, git2::Error> {
    println!("adding files from {} ...", path.display());
    println!("committing...");
    let p = fs::canonicalize(path).unwrap();
    println!("- adding files from {} ...", p.display());


    let mut index = repo.index()?;
    index.add_path(&p)?;
    let oid = index.write_tree()?;
    let signature = Signature::now("Lucas Jin", "nicholasjela@gmail.com")?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;

    repo.commit(Some("HEAD"), //  point HEAD to our new commit
                &signature, // author
                &signature, // committer
                message, // commit message
                &tree, // tree
                &[&parent_commit])
}

pub fn push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", url)?,
    };
    remote.connect(Direction::Push)?;
    remote.push(&["refs/heads/master:refs/heads/master"], None)
}