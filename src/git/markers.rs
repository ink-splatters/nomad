//! Set Git status markers for items within the tree.

use super::utils::get_repo;
use crate::{errors::NomadError, style::models::NomadStyle};

use anyhow::Result;
use git2::{Repository, Status, StatusOptions, StatusShow};

use std::{collections::HashMap, path::Path};

/// Try to extend the `HashMap` containing status markers and their corresponding
/// filenames with new Git repository items.
pub fn extend_marker_map(
    git_markers: &mut HashMap<String, String>,
    nomad_style: &NomadStyle,
    target_directory: &str,
) {
    if let Some(repo) = get_repo(target_directory) {
        if let Ok(top_level_map) = get_status_markers(&nomad_style, &repo, target_directory) {
            git_markers.extend(top_level_map);
        }
    }
}

/// Get the status markers (colored initials) that correspond with the Git status
/// of tracked files in the repository.
pub fn get_status_markers(
    nomad_style: &NomadStyle,
    repo: &Repository,
    target_directory: &str,
) -> Result<HashMap<String, String>, NomadError> {
    let mut status_options = StatusOptions::new();
    status_options
        .show(StatusShow::IndexAndWorkdir)
        .include_untracked(true)
        .recurse_untracked_dirs(true);

    let mut formatted_items = HashMap::new();

    for repo_item in repo.statuses(Some(&mut status_options))?.iter() {
        let item_name = Path::new(target_directory)
            .join(Path::new(repo_item.path().unwrap_or("?")))
            .to_str()
            .unwrap_or("?")
            .to_string();

        match repo_item.status() {
            s if s.contains(Status::INDEX_DELETED) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .staged_deleted_color
                        .paint(&nomad_style.staged_deleted_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::INDEX_MODIFIED) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .staged_modified_color
                        .paint(&nomad_style.staged_modified_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::INDEX_NEW) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .staged_added_color
                        .paint(&nomad_style.staged_added_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::INDEX_RENAMED) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .staged_renamed_color
                        .paint(&nomad_style.staged_renamed_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::WT_DELETED) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .deleted_color
                        .paint(&nomad_style.deleted_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::WT_MODIFIED) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .modified_color
                        .paint(&nomad_style.modified_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::WT_NEW) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .untracked_color
                        .paint(&nomad_style.untracked_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::WT_RENAMED) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .renamed_color
                        .paint(&nomad_style.renamed_marker)
                        .to_string(),
                );
            }
            s if s.contains(Status::CONFLICTED) => {
                formatted_items.insert(
                    item_name,
                    nomad_style
                        .conflicted_color
                        .paint(&nomad_style.conflicted_marker)
                        .to_string(),
                );
            }
            _ => {}
        }
    }

    Ok(formatted_items)
}
