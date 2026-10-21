//! # DW16 — Shelf Stock
//!
//! ```text
//! Final stock: 6
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod stock;

use stock::Shelf;

/// Runs one scripted day at the store and returns the final stock level.
///
/// ```
/// assert_eq!(dw16::run_shelf(), 6);
/// ```
pub fn run_shelf() -> i32 {
    let mut shelf = Shelf::new(20);
    shelf.add(12);
    shelf.remove(5);
    shelf.add(20);
    shelf.remove(25);
    shelf.add(6);
    shelf.level()
}
