// Copyright 2017 array_matrix Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A macro to create a 2d matrix struct with manipulation methods using an internal array.
#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
                unused_qualifications, unused_results)]

pub use array_matrix::ArrayMatrix;
mod array_matrix;

/// A macro that takes a struct-like definition that takes an array type and (row, column)
/// size arguments and implements the required methods.
///
/// Usage:
/// #impl_matrix!{
/// #    struct MyMatrix([f32; (3, 3)]);
/// #};
///
/// #let mut matrix =  MyMatrix([0.0; 9]);
/// #matrix[(2,1)] = 8.1;
/// #println!("{:?}", matrix);
/// #println!("{}", m.row());
///
/// The matrix methods are based on the [`generic_matrix`] crate which uses an internal Vec<T> to
/// hold data.
///
/// [`generic_matrix`]: http://gifnksm.github.io/generic-matrix-rs/generic_matrix/index.html
#[macro_export]
macro_rules! impl_matrix {
    ($st:ident([$t:ty; ($row:expr, $col:expr)])) => {
        struct $st([$t; $row * $col]);

        impl ArrayMatrix for $st {
            fn row(&self) -> usize {
                $row
            }
        
            fn column(&self) -> usize {
                $col
            }
        
            fn size(&self) -> (usize, usize) {
                (self.row(), self.column())
            }

            fn transpose(&self) -> Self {
                let mut trans = $st(self.0.clone());
                for i in 0..self.0.len() {
                   let r = i / $col;
                   let c = i % $col;
                   trans[(c, r)] = self[(r, c)].clone();
                }
                trans
            }
        }

        impl Index<(usize, usize)> for $st {
            type Output = $t;

            #[inline]
            fn index(&self, (i, j): (usize, usize)) -> &$t {
                assert!(i < $row && j < $col);
                &self.0[i * $col + j]
            }
        }

        impl IndexMut<(usize, usize)> for $st {

            #[inline]
            fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut $t {
                assert!(i < $row && j < $col);
                &mut self.0[i * $col + j]
            }
        }

        impl fmt::Debug for $st {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        impl PartialEq for $st {
            fn eq(&self, other: &$st) -> bool {
                self.0 == other.0
            }
        }
    }
}

#[macro_use]
#[cfg(test)]
mod tests {
    use array_matrix::ArrayMatrix;
    use std::ops::{Index, IndexMut};
    use std::fmt;

    #[test]
    fn row_col() {
        const ROW: usize = 3;
        const COLUMN: usize = 3;
        impl_matrix!(TestMatrix([i32; (ROW, COLUMN)]));
        let m = TestMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(m.row(), ROW);
        assert_eq!(m.column(), COLUMN);
    }

    #[test]
    fn transpose() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m = TestMatrix([1, 2, 3, 4]);
        let trans = m.transpose();

        assert_eq!(trans, TestMatrix([1, 3, 2, 4]));
    }

    #[test]
    fn index() {
        impl_matrix!(TestMatrix([i32; (3, 3)]));
        let m = TestMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(m[(1, 2)], 6);
    }

    #[test]
    fn index_mut() {
        impl_matrix!(TestMatrix([i32; (3, 3)]));
        let mut m = TestMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(m[(1, 2)], 6);
        m[(1, 2)] = 30;
        assert_eq!(m[(1, 2)], 30);
    }

    #[test]
    fn eq() {
        impl_matrix!(TestMatrix([i32; (3, 3)]));
        let m_a = TestMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let m_b = TestMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(m_a, m_b);
    }

    #[test]
    fn impl_two_matrix() {
        impl_matrix!(TestMatrixA([i32; (2, 2)]));
        impl_matrix!(TestMatrixB([f32; (2, 2)]));

        let m_a = TestMatrixA([1, 2, 3, 4]);
        let m_b = TestMatrixB([1., 2., 3., 4.]);
        
        assert_eq!(m_a[(0, 0)] as f32, m_b[(0, 0)]);
    }
}
