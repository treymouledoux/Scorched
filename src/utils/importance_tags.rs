//! This file contains the tags for the different levels of importance.
//! These are used in front of the message when logging and are colored.

/// Error tag displays red.
pub(crate) const ERROR_TAG: &str = "\x1b[31m[ERROR]\x1b[0m";

/// Warning tag displays yellow.
pub(crate) const WARNING_TAG: &str = "\x1b[33m[WARNING]\x1b[0m";

/// Info tag displays light gray.
pub(crate) const INFO_TAG: &str = "\x1b[37m[INFO]\x1b[0m";

/// Debug tag displays pink.
pub(crate) const DEBUG_TAG: &str = "\x1b[35m[DEBUG]\x1b[0m";
