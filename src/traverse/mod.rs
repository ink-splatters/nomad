//! Traverse the target directory.

pub mod format;
pub mod models;
pub mod modes;
pub mod traits;
pub mod utils;

use self::{
    models::FoundItem,
    modes::NomadMode,
    traits::{ToTree, TransformFound},
};
use crate::{
    cli::Args, errors::NomadError, git::markers::extend_marker_map, style::models::NomadStyle,
    utils::paths::canonicalize_path,
};

use anyhow::{private, Result};
use ignore::{self, Walk};
use ptree::{item::StringItem, PrintConfig};
use regex::Regex;

use std::{collections::HashMap, ffi::OsStr, path::Path};

/// Traverse the directory and display files and directories accordingly.
pub fn walk_directory(
    args: &Args,
    nomad_mode: NomadMode,
    nomad_style: &NomadStyle,
    target_directory: &str,
    walker: &mut Walk,
) -> Result<(StringItem, PrintConfig, Option<Vec<String>>), NomadError> {
    let regex_expression = if let Some(ref pattern) = args.pattern {
        match Regex::new(&pattern.clone()) {
            Ok(regex) => Some(regex),
            Err(error) => return private::Err(NomadError::RegexError(error)),
        }
    } else {
        None
    };

    let mut git_markers: HashMap<String, String> = HashMap::new();
    extend_marker_map(
        args,
        &mut git_markers,
        nomad_style,
        Path::new(target_directory).to_str().unwrap_or("?"),
    );

    let (tree, config, directory_items) = walker
        .filter_map(|dir_entry| {
            if let Ok(entry) = dir_entry {
                if entry.path().is_dir() {
                    extend_marker_map(
                        args,
                        &mut git_markers,
                        nomad_style,
                        entry.path().to_str().unwrap_or("?"),
                    );
                    None
                } else {
                    if let Some(ref regex) = regex_expression {
                        if let Some(matched) = regex.find(
                            &entry
                                .path()
                                .file_name()
                                .unwrap_or(OsStr::new("?"))
                                .to_str()
                                .unwrap_or("?")
                                .to_string(),
                        ) {
                            Some(FoundItem {
                                marker: git_markers
                                    .get(
                                        &canonicalize_path(entry.path().to_str().unwrap_or("?"))
                                            .unwrap_or("?".to_string()),
                                    )
                                    .map_or(None, |marker| Some(marker.to_string())),
                                matched: Some((matched.start(), matched.end())),
                                path: entry.path().to_str().unwrap_or("?").to_string(),
                            })
                        } else {
                            None
                        }
                    } else {
                        Some(FoundItem {
                            marker: git_markers
                                .get(
                                    &canonicalize_path(entry.path().to_str().unwrap_or("?"))
                                        .unwrap_or("?".to_string()),
                                )
                                .map_or(None, |marker| Some(marker.to_string())),
                            matched: None,
                            path: entry.path().to_str().unwrap_or("?").to_string(),
                        })
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<FoundItem>>()
        .transform(target_directory)?
        .to_tree(args, nomad_mode, nomad_style, target_directory)?;

    Ok((tree, config, directory_items))
}
