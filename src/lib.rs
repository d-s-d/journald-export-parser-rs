//! # Journald Journal Export Format Parser.
//!
//! This library contains a small parser for the [Journald Journal Export
//! Format](https://systemd.io/JOURNAL_EXPORT_FORMATS/).
//!
//! The format is very versatile. Each stream consists of a series of journal
//! entries, each of which comprising one or more fields. A field is simply a
//! key value pair, similar to Unix process environment variables.
//!
//! The parser avoids heap allocations and operates on a buffer with bounded
//! size. The lengths of the fields and overall entry size can be controlled
//! using with the [config::JournalExportLimits] struct.
//!
//! The parsing logic is separated out from any i/o-logic.
//! [self::journald::parser::JournalExportParser] contains the parser logic and
//! manages the buffer. The structs [JournalExportAsyncRead] and
//! [JournalExportRead] provide async and sync versions of a parser.
//!
//! ## Implementation notes
//!
//! Both, [JournalExportRead] and [JournalExportAsyncRead] are stateful objects
//! that buffer the last parsed journal entry. The latter can be accessed using
//! the `get_entry()`-method which returns a [journald::parser::RefEntry] object.

pub mod config;
pub mod fieldname;
pub mod journald;
pub mod shiftbuffer;

pub use journald::{
    parser::{JournalExportParser, RefEntry},
    JournalExportAsyncRead, JournalExportRead, JournalExportReadError,
};
