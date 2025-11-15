/*
MIT License

Copyright (c) 2025 Philipp Schuster

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

#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::must_use_candidate
)]
// I can't do anything about this; fault of the dependencies
#![allow(clippy::multiple_crate_versions)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::all)]
#![no_std]

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
//! next months in Rust stable. `bit_ops` will adapt, as soon as this
//! changes.
//!
//! Note that the most trivial bit operations, such as `"shift_bits"` or
//! `"keep_bits"` won't be covered by the API, as this would introduce a
//! convoluted way around the standard operators `<<`, `>>`, and `&`. Only
//! non-trivial non-oneliners are covered by the API as well as operations,
//! where the semantic name provides a value-add over a (possible even oneliner)
//! (combination of) bit operation.
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
//! use x86_ioapic::*;
//!
//!  // ACTUAL LIBRARY USAGE BEGINS HERE
//!
//! let redirection_entry = bit_ops::bitops_u64::set_bits_exact_n(
//!     0,
//!     &[
//!         (7, VECTOR_BITS, VECTOR_SHIFT),
//!         (0b111 /* ExtInt */, DELIVERY_MODE_BITS, DELIVERY_MODE_SHIFT),
//!         (0 /* physical */, DESTINATION_MODE_BITS, DESTINATION_MODE_SHIFT),
//!         (1 /* low-active */, PIN_POLARITY_BITS, PIN_POLARITY_SHIFT),
//!         (1 /* level-triggered */, TRIGGER_MODE_BITS, TRIGGER_MODE_SHIFT),
//!         (1 /* masked */, MASKED_BITS, MASKED_SHIFT),
//!         (13 /* APIC ID */, DESTINATION_BITS, DESTINATION_SHIFT),
//!     ],
//! );
//! assert_eq!(redirection_entry, 0xd0000000001a707);
//! ```
//!
//! ### Trait API
//!
//! The trait [`BitOps`] is implemented for [`u8`], [`u16`], [`u32`], [`u64`],
//! [`u128`], and [`usize`]. It provides the same functionality as the function
//! API, but you call each operation as associated function.
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
//!
//! ### Other Functionality
//!
//! `bit_ops` also offers additional functionality which is tightly connected
//! to working with raw bits:
//!
//! - [`BitsIter`] and [`BitmapIter`]
//!
//! ## Comparison to other Crates
//!
//! `bit_ops` (this crate) is fresher, more feature-complete, and fully `no_std`
//! and `const` compatible compared to `bit_op`, `bitops`, and `bitwise`. Unlike
//! in `bitwise` for example, functionality from `libcore` is not replicated.
//!
//! ## MSRV
//!
//! 1.85.1 stable
//!
//! ## License
//!
//! MIT License.

#[cfg(test)]
extern crate std;

mod bitpos_iter;
mod function_api;
mod trait_api;

pub use bitpos_iter::*;
pub use function_api::*;
pub use trait_api::*;
