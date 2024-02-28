//! Functionality for interacting with daemons

use super::codematrix::CodeMatrix;

/// Represents a daemon with a sequence
pub(crate) struct Daemon {
    variant: DaemonType,
    sequence: Vec<CodeMatrix>
}

pub(crate) enum DaemonType {
    BasicDatamine
}

