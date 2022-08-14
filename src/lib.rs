// Copyright 2017 array_matrix Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A macro to create a 2d matrix struct with manipulation methods using an internal array.
//! The matrix methods are based on the [`generic_matrix`] crate which uses an internal Vec<T> to
//! hold data.
//!
//! [`generic_matrix`]: http://gifnksm.github.io/generic-matrix-rs/generic_matrix/index.html
#![warn(
    bad_style,
    missing_docs,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

pub use array_matrix::ArrayMatrix;
mod array_matrix;
mod macros;
mod matrix;
