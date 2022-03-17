//! Traversal modes for `nomad`.

/// Modes in which `nomad` may operate.
pub enum NomadMode {
    /// Run `nomad` in `git branch` mode.
    GitBranch,
    /// Run `nomad` in `git status` mode.
    GitStatus,
    /// Run `nomad` in interactive mode.
    Interactive,
    /// Run `nomad` in normal mode.
    Normal,
}
