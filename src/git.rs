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


// this 2 methods are just what I need, but not test yet
pub fn commit_and_push<F>(repo: &git2::Repository, mut f: F) -> CargoResult<()>
    where
        F: FnMut() -> CargoResult<(String, PathBuf)>,
{
    let repo_path = repo.workdir().unwrap();

    // Attempt to commit in a loop. It's possible that we're going to need to
    // rebase our repository, and after that it's possible that we're going to
    // race to commit the changes. For now we just cap out the maximum number of
    // retries at a fixed number.
    for _ in 0..20 {
        let (msg, dst) = f()?;

        // git add $file
        let mut index = repo.index()?;
        let mut repo_path = repo_path.iter();
        let dst = dst.iter()
            .skip_while(|s| Some(*s) == repo_path.next())
            .collect::<PathBuf>();
        index.add_path(&dst)?;
        index.write()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        // git commit -m "..."
        let head = repo.head()?;
        let parent = repo.find_commit(head.target().unwrap())?;
        let sig = repo.signature()?;
        repo.commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[&parent])?;

        // git push
        let mut ref_status = None;
        let mut origin = repo.find_remote("origin")?;
        let res = {
            let mut callbacks = git2::RemoteCallbacks::new();
            callbacks.credentials(credentials);
            callbacks.push_update_reference(|refname, status| {
                assert_eq!(refname, "refs/heads/master");
                ref_status = status.map(|s| s.to_string());
                Ok(())
            });
            let mut opts = git2::PushOptions::new();
            opts.remote_callbacks(callbacks);
            origin.push(&["refs/heads/master"], Some(&mut opts))
        };
        match res {
            Ok(()) if ref_status.is_none() => return Ok(()),
            Ok(()) => info!("failed to push a ref: {:?}", ref_status),
            Err(e) => info!("failure to push: {}", e),
        }

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(credentials);
        origin.update_tips(
            Some(&mut callbacks),
            true,
            git2::AutotagOption::Unspecified,
            None,
        )?;

        // Ok, we need to update, so fetch and reset --hard
        origin.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;
        let head = repo.head()?.target().unwrap();
        let obj = repo.find_object(head, None)?;
        repo.reset(&obj, git2::ResetType::Hard, None)?;
    }

    Err(internal("Too many rebase failures"))
}

pub fn credentials(
    _user: &str,
    _user_from_url: Option<&str>,
    _cred: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    match get_git_credentials_from_yml() {
        (Ok(u), Ok(p)) => {
            println!("we got: {:?}", u);
            git2::Cred::userpass_plaintext(&u, &p)
        },
        _ => Err(git2::Error::from_str("no authentication set")),
    }
}

fn get_git_credentials_from_yml() -> (&username, &password){
    let username = "jinfagang";
    let password = "123456";
    return (username, password)
}