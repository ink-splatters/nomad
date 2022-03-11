//! Create an application state for the TUI.

use super::stateful_widgets::{StatefulWidget, WidgetMode};
use crate::{
    cli::Args,
    errors::NomadError,
    style::models::NomadStyle,
    traverse::{modes::NomadMode, utils::build_walker, walk_directory},
};

use std::{
    ffi::OsStr,
    fs::File,
    io::Read,
    path::{Component, Path},
};

use ptree::write_tree_with;
use tui::{
    style::{Color, Style},
    widgets::{Cell, ListState, Row, TableState},
};

/// Contains the different modes that may be evoked based on user interaction.
///
/// These variants correspond to the different widgets in the UI.
pub enum UIMode {
    /// Move focus to the breadcrumbs at the top of the user interface.
    Breadcrumbs,
    /// Enter the help menu.
    Help,
    /// Move focus to the `cat`ed file and enable vertical and horizontal scrolling.
    Inspect,
    /// Normal mode.
    Normal,
    /// The UI is reloading its `App` data.
    /// This is evoked when the user enters a new directory.
    Loading,
}

/// Contains the different popup modes that may be evoked based on user interaction.
pub enum PopupMode {
    /// No popup is rendered.
    Disabled,
    /// Render a popup with the error message if applicable.
    Error(String),
    /// Render a popup that accepts a target filename to export the current tree.
    Export,
    /// Nothing was found after a pattern was provided.
    NothingFound,
    /// Render a popup that accepts a pattern.
    PatternInput,
    /// Render a popup indicating the tree is reloading.
    Reloading,
    /// Render the settings menu as a popup.
    Settings,
}

/// Contains the UI's current state.
pub struct App<'a> {
    /// Hold each `Row` of settings displayed in the settings popup.
    pub app_settings: StatefulWidget<Row<'a>, TableState>,
    /// Hold a `BreadcrumbState` for UI navigation.
    pub breadcrumbs: StatefulWidget<String, ListState>,
    /// Collected user input.
    pub collected_input: Vec<String>,
    /// The current directory of the tree that is displayed.
    pub current_directory: String,
    /// All items in the target directory.
    pub directory_items: StatefulWidget<String, ListState>,
    /// The directory tree.
    pub directory_tree: StatefulWidget<String, ListState>,
    /// Stores `None` or `Some(file contents)`.
    pub file_contents: Option<Vec<String>>,
    /// Hold the current popup mode.
    pub popup_mode: PopupMode,
    /// Hold the scroll position for `Scroll` mode.
    pub scroll: u16,
    /// Hold the current UI mode.
    pub ui_mode: UIMode,
    /// Hold the user input for popup prompts.
    pub user_input: String,
}

impl<'a> App<'a> {
    /// Create a new interactive instance with the target directory.
    pub fn new(
        args: &Args,
        nomad_style: &NomadStyle,
        target_directory: &str,
    ) -> Result<App<'a>, NomadError> {
        let (tree, items) = get_tree(args, nomad_style, target_directory)?;
        let mut directory_tree = StatefulWidget::new(tree, ListState::default(), WidgetMode::Files);
        let mut directory_items = StatefulWidget::new(
            match items {
                Some(paths) => paths,
                None => Vec::new(),
            },
            ListState::default(),
            WidgetMode::Files,
        );

        directory_tree.state.select(Some(0));
        directory_items.state.select(Some(0));

        Ok(App {
            app_settings: StatefulWidget::new(
                get_settings(args),
                TableState::default(),
                WidgetMode::Standard,
            ),
            breadcrumbs: StatefulWidget::new(
                get_breadcrumbs(target_directory)?,
                ListState::default(),
                WidgetMode::Standard,
            ),
            collected_input: Vec::new(),
            current_directory: Path::new(target_directory)
                .file_name()
                .unwrap_or(&OsStr::new("?"))
                .to_str()
                .unwrap_or("?")
                .to_string(),
            directory_items,
            directory_tree,
            file_contents: None,
            popup_mode: PopupMode::Disabled,
            scroll: 0,
            ui_mode: UIMode::Normal,
            user_input: String::new(),
        })
    }

    /// `cat` the selected file if the selected item is a file.
    pub fn cat_file(&mut self) -> Result<(), NomadError> {
        match self.directory_tree.state.selected() {
            Some(index) => {
                let selected_item = &self.directory_items.items[index];

                if Path::new(selected_item).canonicalize()?.is_dir() {
                    self.file_contents = None;
                } else {
                    let mut buffer = Vec::new();
                    let mut file = File::open(&self.directory_items.items[index])?;

                    file.read_to_end(&mut buffer)?;

                    self.file_contents = Some(
                        String::from_utf8_lossy(&buffer)
                            .split("\n")
                            .map(|line| line.to_string())
                            .collect::<Vec<String>>(),
                    )
                }
            }
            None => self.file_contents = None,
        }

        Ok(())
    }

    /// Update the app's `breadcrumbs`, `directory_items`, and `directory_tree`.
    pub fn refresh(
        &mut self,
        args: &Args,
        nomad_style: &NomadStyle,
        target_directory: &str, // TODO: THIS SHOULDN'T BE USED FOR THE REFRESH
    ) -> Result<(), NomadError> {
        self.popup_mode = PopupMode::Reloading;

        self.app_settings = StatefulWidget::new(
            get_settings(args),
            TableState::default(),
            WidgetMode::Standard,
        );
        self.file_contents = None;
        self.scroll = 0;

        self.breadcrumbs = StatefulWidget::new(
            get_breadcrumbs(target_directory)?,
            ListState::default(),
            WidgetMode::Standard,
        );

        let (tree, items) = get_tree(args, nomad_style, target_directory)?;

        self.directory_tree = StatefulWidget::new(tree, ListState::default(), WidgetMode::Files);
        self.directory_items = StatefulWidget::new(
            match items {
                Some(paths) => paths,
                None => Vec::new(),
            },
            ListState::default(),
            WidgetMode::Files,
        );

        self.popup_mode = PopupMode::Disabled;

        Ok(())
    }

    /// Refresh the app after the user searched for a pattern.
    pub fn pattern_search(
        &mut self,
        args: &mut Args,
        nomad_style: &NomadStyle,
        target_directory: &str,
    ) -> Result<(), NomadError> {
        self.popup_mode = PopupMode::Reloading;

        args.pattern = self.collected_input.pop();

        if let Err(error) = self.refresh(args, nomad_style, target_directory) {
            match error {
                NomadError::NothingFound => self.popup_mode = PopupMode::NothingFound,
                _ => {}
            }
        } else {
            self.popup_mode = PopupMode::Disabled;
        }

        Ok(())
    }
}

/// Return all app settings formatted in `Row`s.
fn get_settings<'a>(args: &Args) -> Vec<Row<'a>> {
    let assign_boolean_flag = |label: &'a str, flag| -> Row<'a> {
        Row::new(vec![
            Cell::from(label),
            Cell::from(format!("{}", flag)).style(Style::default().fg(if flag {
                Color::Green
            } else {
                Color::Red
            })),
        ])
    };

    vec![
        assign_boolean_flag(" all labels", args.all_labels),
        assign_boolean_flag(" dirs", args.dirs),
        assign_boolean_flag(" disrespect", args.disrespect),
        assign_boolean_flag(" hidden", args.hidden),
        assign_boolean_flag(" label directories", args.label_directories),
        Row::new(vec![
            Cell::from(" max depth"),
            Cell::from(format!(
                "{}",
                if let Some(ref depth) = args.max_depth {
                    depth.to_string()
                } else {
                    "None".to_string()
                }
            ))
            .style(Style::default().fg(if args.max_depth.is_some() {
                Color::Green
            } else {
                Color::Red
            })),
        ]),
        Row::new(vec![
            Cell::from(" max filesize"),
            Cell::from(format!(
                "{}",
                if let Some(ref size) = args.max_filesize {
                    size.to_string()
                } else {
                    "None".to_string()
                }
            ))
            .style(Style::default().fg(if args.max_filesize.is_some() {
                Color::Green
            } else {
                Color::Red
            })),
        ]),
        assign_boolean_flag(" metadata", args.metadata),
        assign_boolean_flag(" no Git", args.no_git),
        assign_boolean_flag(" no icons", args.no_icons),
        assign_boolean_flag(" numbered", args.numbers),
        Row::new(vec![
            Cell::from(" pattern"),
            Cell::from(format!(
                "{}",
                if let Some(ref pattern) = args.pattern {
                    pattern.to_string()
                } else {
                    "None".to_string()
                }
            ))
            .style(Style::default().fg(if args.pattern.is_some() {
                Color::Green
            } else {
                Color::Red
            })),
        ]),
        assign_boolean_flag(" plain", args.plain),
    ]
}

/// Get the breadcrumbs for the target directory.
fn get_breadcrumbs(target_directory: &str) -> Result<Vec<String>, NomadError> {
    let mut breadcrumbs = Vec::new();
    for component in Path::new(target_directory).canonicalize()?.components() {
        match component {
            Component::Normal(section) => {
                breadcrumbs.push(section.to_str().unwrap_or("?").to_string());
            }
            _ => {}
        }
    }

    Ok(breadcrumbs)
}

/// Get the directory tree as a `Vec<String>` and the directory items as an `Option<Vec<String>>`.
fn get_tree(
    args: &Args,
    nomad_style: &NomadStyle,
    target_directory: &str,
) -> Result<(Vec<String>, Option<Vec<String>>), NomadError> {
    let (tree, config, directory_items) = walk_directory(
        args,
        NomadMode::Interactive,
        nomad_style,
        target_directory,
        &mut build_walker(args, target_directory, None)?,
    )?;

    // Write the tree to a buffer, then convert it to a `Vec<String>`.
    let mut tree_buf = Vec::new();
    write_tree_with(&tree, &mut tree_buf, &config)?;

    Ok((
        String::from_utf8_lossy(&tree_buf)
            .split("\n")
            .map(|line| line.to_string())
            .collect::<Vec<String>>(),
        directory_items,
    ))
}