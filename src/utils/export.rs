//! Export a directory's tree to a file instead of saving.

use crate::errors::NomadError;

use ansi_term::*;
use anyhow::Result;
use ptree::{item::StringItem, write_tree_with, PrintConfig};

use std::{env, fs::File};

/// Get the absolute path for the file name.
fn get_absolute_path(file_name: &str) -> Result<String, NomadError> {
    Ok(env::current_dir()?
        .join(file_name)
        .into_os_string()
        .into_string()
        .expect("Could not get the current directory!")
        .clone())
}

pub fn export_tree(
    config: PrintConfig,
    file_name: &str,
    tree: StringItem,
) -> Result<(), NomadError> {
    let file_path = get_absolute_path(&file_name)?;
    let file = File::create(&file_path)?;

    write_tree_with(&tree, file, &config).map_or_else(
        |error| {
            Err(NomadError::PTreeError {
                context: format!("Unable to export directory tree to {file_path}"),
                source: error,
            })
        },
        |_| {
            let success_message = Colour::Green
                .bold()
                .paint(format!("Tree was exported to {file_path}\n"));
            println!("{success_message}");

            Ok(())
        },
    )
}
