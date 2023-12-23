use std::str::from_utf8;
use axum::body::Bytes;
use axum::http::{StatusCode};
use git2::{Commit, Repository};
use tar::Archive;
use tempfile::tempdir;
use tracing::info;

pub fn router() -> axum::Router {
    let archives = axum::Router::new()
        .route("/archive_files", axum::routing::post(day20_archive_files))
        .route("/archive_files_size", axum::routing::post(day20_archive_files_size))
        .route("/cookie", axum::routing::post(day20_cookie));

    axum::Router::new().nest("/", archives)
}

async fn day20_archive_files(request: Bytes) -> Result<String, StatusCode> {
    info!("Archive files called.");
    let mut archive = Archive::new(&request[..]);
    let files = archive.entries().unwrap().collect::<Vec<_>>().len();
    Ok(files.to_string())
}

async fn day20_archive_files_size(request: Bytes) -> Result<String, StatusCode> {
    info!("Archive files size called.");
    let mut archive = Archive::new(&request[..]);
    let mut size = 0;
    for file in archive.entries().unwrap() {
        let file = file.unwrap();
        size += file.size();
    }
    Ok(size.to_string())
}

async fn day20_cookie(request: Bytes) -> Result<String, StatusCode> {
    info!("Cookie called.");
    let mut archive = Archive::new(&request[..]);
    let tmp_path = tempdir().expect("Could not create tempdir").into_path();
    archive.unpack(&tmp_path).expect("Could not unpack archive");
    let repo = Repository::open(&tmp_path).expect("Could not open repository");
    let commit = repo.find_branch("christmas", git2::BranchType::Local)
        .expect("Could not find branch")
        .get()
        .peel_to_commit()
        .expect("Could not peel to commit");
    info!("Initial commit: {}", commit.id());
    let (author, commit_id) = search_tree(&repo, commit);
    if let (Some(author), Some(commit_id)) = (author, commit_id) {
        return Ok(format!("{} {}", author, commit_id));
    }

    Ok("not found".into())
}

fn search_tree(repo: &Repository, commit: Commit) -> (Option<String>, Option<String>) {
    info!("Current commit: {}", commit.id());
    let tree = commit.tree().expect("Could not get tree");
    let mut author = None;
    let mut commit_id = None;
    tree.walk(git2::TreeWalkMode::PreOrder, |_, entry| {
        info!("Entry: {:?}", entry.name());
        if entry.name() == Some("santa.txt") {
            let object = entry.to_object(repo).expect("Could not get object");
            let blob = object.peel_to_blob().expect("Could not peel to blob");
            let content = blob.content();
            let content = from_utf8(content).expect("Could not convert to utf8");
            info!("Commit content: {}", content);
            if content.contains("COOKIE") {
                info!("Found cookie!");
                info!("Commit: {}, author: {}", commit.id(), commit.author());
                author = Some(commit.author().name().unwrap().to_string());
                commit_id = Some(commit.id().to_string());
                return git2::TreeWalkResult::Abort;
            }
        }
        git2::TreeWalkResult::Ok
    }).expect("Could not walk tree");

    if author.is_some() && commit_id.is_some() {
        info!("Found cookie in commit: {}", commit.id());
        return (author, commit_id);
    }

    commit.parents().for_each(|parent| {
        (author, commit_id) = search_tree(repo, parent);
    });

    info!("No cookie found in commit: {}", commit.id());
    (author, commit_id)
}