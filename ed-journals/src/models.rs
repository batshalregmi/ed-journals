pub mod journal_event;

/// This module contains detailed structs and enums for the different kinds of events that are read
/// from a [JournalFile].
pub mod journal_event_content;
pub mod journal_dir;
pub mod journal_file;

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "async")]
pub mod r#async;
