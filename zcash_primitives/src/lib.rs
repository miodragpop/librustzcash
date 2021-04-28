//! *General Zcash primitives.*
//!
//! `zcash_primitives` is a library that provides the core structs and functions necessary
//! for working with Zcash.

#![cfg_attr(docsrs, feature(doc_cfg))]
// Catch documentation errors caused by code changes.
#![deny(broken_intra_doc_links)]
// Temporary until we have addressed all Result<T, ()> cases.
#![allow(clippy::result_unit_err)]

#[cfg(feature = "transparent-inputs")]
extern crate ripemd160;

#[cfg(feature = "transparent-inputs")]
extern crate secp256k1;

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

#[cfg(test)]
extern crate rand_xorshift;

pub mod block;
pub mod consensus;
pub mod constants;
pub mod legacy;
pub mod memo;
pub mod merkle_tree;
pub mod sapling;
pub mod serialize;
pub mod transaction;
pub mod zip32;

#[cfg(feature = "zfuture")]
pub mod extensions;

#[cfg(test)]
mod test_vectors;
