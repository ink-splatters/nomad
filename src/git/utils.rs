//! Contains useful utilities that support Git functionality.

use crate::{errors::NomadError, traverse::format::highlight_matched};

use ansi_term::Colour;
use anyhow::{anyhow, Result};
use git2::{Branch, Commit, ObjectType, Repository};

/// Try to discover a Git repository at or above the current path.
fn discover_repo(target_directory: &str) -> Option<Repository> {
    if let Ok(repo) = Repository::discover(target_directory) {
        if repo.is_bare() {
            println!("\n{}", Colour::Fixed(172).paint("Git repository is bare!"));
            None
        } else {
            Some(repo)
        }
    } else {
        None
    }
}

/// Try to get Git metadata from the target directory.
pub fn get_repo(target_directory: &str) -> Option<Repository> {
    if let Ok(repo) = Repository::open(target_directory) {
        if repo.is_bare() {
            println!("\n{}", Colour::Fixed(172).paint("Git repository is bare!"));
            None
        } else {
            Some(repo)
        }
    } else {
        discover_repo(target_directory)
    }
}

/// Try to get the current Git branch's name.
pub fn get_repo_branch(repo: &Repository) -> Option<String> {
    if let Ok(reference) = repo.head() {
        if let Ok(Some(name)) = Branch::wrap(reference).name() {
            let branch_name = name.to_string();
            Some(branch_name)
        } else {
            println!(
                "\n{}\n",
                Colour::Red
                    .bold()
                    .paint("Could not get the current Git branch name!")
            );
            None
        }
    } else {
        println!(
            "\n{}\n",
            Colour::Red.bold().paint("Could not get repository HEAD!")
        );
        None
    }
}

/// Get the last commit in the Git repository.
pub fn get_last_commit(repo: &Repository) -> Result<Commit, NomadError> {
    let object = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    object.into_commit().map_err(|_| {
        NomadError::Error(anyhow!(
            "Could not find the last commit in this Git repository!"
        ))
    })
}

/// Add color/style to the filename depending on its Git status.
pub fn paint_git_item(filename: &str, marker: &str, matched: Option<(usize, usize)>) -> String {
    let staged_deleted = Colour::Red.bold().paint("SD").to_string();
    let staged_modified = Colour::Yellow.bold().paint("SM").to_string();
    let staged_new = Colour::Green.bold().paint("SA").to_string();
    let staged_renamed = Colour::Fixed(172).bold().paint("SR").to_string();
    let conflicted = Colour::Red.bold().paint("CONFLICT").to_string();

    let formatted_filename = if let Some(ranges) = matched {
        highlight_matched(filename.to_string(), ranges)
    } else {
        filename.to_string()
    };

    match marker.to_string() {
        _ if marker == staged_deleted => Colour::Red
            .bold()
            .strikethrough()
            .paint(format!("{formatted_filename}"))
            .to_string(),
        _ if marker == staged_modified => Colour::Yellow
            .bold()
            .paint(format!("{formatted_filename}"))
            .to_string(),
        _ if marker == staged_new => Colour::Green
            .bold()
            .paint(format!("{formatted_filename}"))
            .to_string(),
        _ if marker == staged_renamed => Colour::Fixed(172)
            .bold()
            .paint(format!("{formatted_filename}"))
            .to_string(),
        _ if marker == conflicted => Colour::Red
            .bold()
            .paint(format!("{formatted_filename}"))
            .to_string(),
        _ => formatted_filename,
    }
}
