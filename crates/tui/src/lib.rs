//! Interactive terminal client for Universal Chess Interface (UCI) engines.
//!
//! The crate separates child-process communication, application state, and
//! terminal rendering so protocol and interaction behavior remain testable
//! without a live terminal.
