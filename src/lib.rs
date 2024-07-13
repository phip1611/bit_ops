//! # bit-ops
//!
//! ## About
//!
//! Bitwise operations on primitive integer types, but no_std and const-compatible.
//! Provides a collection of typical bit manipulation operations that are primarily
//! required in low-level development. Unlike other crates that provide tooling to
//! create sophisticated high-level types with bitfields, the focus of bit-ops is
//! to work on raw integers. Thus, bit-ops chose a manual and more direct approach.
//!
//!
//! ## API
//!
//! This crate exports a "Function API" and a "Trait API". The function API is
//! the foundation and provides no_std and const-compatible functions. The
//! Trait API won't be const-compatible unless const trait methods are
//! supported. This is not the case in Mid-2024 and the upcoming months in
//! Rust stable.
//!
//! ### Function API
//!
//! For each primitive unsigned integer type, there is a module with the
//! same set of functions:
//!
//! - [`bitops_u8`]
//! - [`bitops_u16`]
//! - [`bitops_u32`]
//! - [`bitops_u64`]
//! - [`bitops_usize`]
//!
//! ### Trait API
//!
//! The trait [`BitOps`] is implemented for `u8`, `u16`, `u32`, `u64`, and
//! `usize`. It provides the same functionality as the function API, but you
//! call each function as method.
//!
//! ## Comparison to Other Crates

mod function_api;
mod trait_api;

pub use function_api::*;
pub use trait_api::*;
