/*
MIT License

Copyright (c) 2024 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
//! # bit_ops
//!
//! ## About
//!
//! Common bit-oriented operations on primitive integer types with a focus on
//! `no_std` and `const` compatibility. Unlike other crates that provide tooling to
//! create sophisticated high-level types with bitfields, the focus of `bit_ops` is
//! on raw primitive integer types.
//!
//! ## API
//!
//! This crate exports a Function API and a Trait API. The Function API is
//! the foundation and provides `no_std` and `const`-compatible functions. The
//! Trait API won't be `const`-compatible unless `const` trait methods are
//! supported by Rust (stable). This is not the case in Mid-2024 and the
//! next months in Rust stable. This crate will adapt, as soon as this
//! changes.
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
//! - [`bitops_u128`]
//! - [`bitops_usize`]
//!
//! #### Example
//!
//! The following example shows a real-world scenario where several properties
//! are encoded in a single `u64` representing an x86 IOAPIC redirection entry.
//!
//! ```rust
//! use bit_ops::bitops_u64;
//!
//! // PREREQUISITES: Some Definitions
//!
//! /// See specification of the x86 IOAPIC redirection entry for more details.
//! mod x86_ioapic {
//!     pub const VECTOR_BITS: u64 = 8;
//!     pub const VECTOR_SHIFT: u64 = 0;
//!     pub const DELIVERY_MODE_BITS: u64 = 3;
//!     pub const DELIVERY_MODE_SHIFT: u64 = 8;
//!     pub const DESTINATION_MODE_BITS: u64 = 1;
//!     pub const DESTINATION_MODE_SHIFT: u64 = 11;
//!     pub const PIN_POLARITY_BITS: u64 = 1;
//!     pub const PIN_POLARITY_SHIFT: u64 = 13;
//!     pub const TRIGGER_MODE_BITS: u64 = 1;
//!     pub const TRIGGER_MODE_SHIFT: u64 = 15;
//!     pub const MASKED_BITS: u64 = 1;
//!     pub const MASKED_SHIFT: u64 = 16;
//!     pub const DESTINATION_BITS: u64 = 8;
//!     pub const DESTINATION_SHIFT: u64 = 56;
//! }
//!
//! let vector = 7;
//! let delivery_mode = 0b111; // ExtInt
//! let destination_mode = 0; // physical
//! let pin_polarity = 1; // low-active
//! let trigger_mode = 1; // level-triggered
//! let masked = 1;
//! let destination = 13;
//!
//! use x86_ioapic::*;
//!
//!  // ACTUAL LIBRARY USAGE BEGINS HERE
//!
//! let redirection_entry = bitops_u64::set_bits_exact_n(
//!     0,
//!     &[
//!         (vector, VECTOR_BITS, VECTOR_SHIFT),
//!         (delivery_mode, DELIVERY_MODE_BITS, DELIVERY_MODE_SHIFT),
//!         (destination_mode, DESTINATION_MODE_BITS, DESTINATION_MODE_SHIFT),
//!         (pin_polarity, PIN_POLARITY_BITS, PIN_POLARITY_SHIFT),
//!         (trigger_mode, TRIGGER_MODE_BITS, TRIGGER_MODE_SHIFT),
//!         (masked, MASKED_BITS, MASKED_SHIFT),
//!         (destination, DESTINATION_BITS, DESTINATION_SHIFT),
//!     ],
//! );
//! assert_eq!(redirection_entry, 0xd0000000001a707);
//! ```
//!
//! ### Trait API
//!
//! The trait [`BitOps`] is implemented for [`u8`], [`u16`], [`u32`], [`u64`],
//! [`u128`], and [`usize`]. It provides the same functionality as the function
//! API, but you call each bit operation as associated function.
//!
//! #### Example
//!
//! Unlike in the Function API, you can use chaining here:
//!
//! ```rust
//! use bit_ops::BitOps;
//!
//! let raw = 0_u64.set_bit(1).set_bit(2);
//! assert_eq!(raw, 0b110);
//! ```

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::must_use_candidate,
    // clippy::restriction,
    // clippy::pedantic
)]
// now allow a few rules which are denied by the above statement
// --> they are ridiculous and not necessary
#![allow(
    clippy::suboptimal_flops,
    clippy::redundant_pub_crate,
    clippy::fallible_impl_from
)]
// I can't do anything about this; fault of the dependencies
#![allow(clippy::multiple_crate_versions)]
// allow: required because of derive macro.. :(
#![allow(clippy::use_self)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]

mod function_api;
mod trait_api;

pub use function_api::*;
pub use trait_api::*;
