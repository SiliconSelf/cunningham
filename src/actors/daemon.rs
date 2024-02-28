//! Functionality for interacting with daemons

use super::codematrix::CodeMatrix;

/// Represents a daemon with a sequence
#[derive(Debug)]
pub(crate) struct Daemon {
    variant: DaemonType,
    sequence: Vec<CodeMatrix>
}

#[derive(Debug)]
pub(crate) enum DaemonType {
    BasicDatamine
}

