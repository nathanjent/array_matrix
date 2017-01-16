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

        impl $st {
            /// Creates an identity matrix.
            #[allow(dead_code)]
            fn identity() -> $st {
                let mut m = $st([0 as $t; $row * $col]);
                for i in 0..$row {
                    m[(i, i)] = 1 as $t;
                }
                m
            }
        }

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

            fn swap(&mut self, (a_i, a_j): (usize, usize), (b_i, b_j): (usize, usize)) {
                self.0.swap(a_i * $col + a_j, b_i * $col + b_j);
            }

            fn transpose(&self) -> Self {
                let mut trans = $st([0 as $t; $row * $col]);
                for i in 0..self.0.len() {
                   let r = i / $col;
                   let c = i % $col;
                   trans[(c, r)] = self[(r, c)].clone();
                }
                trans
            }

            fn transpose_mut(&mut self) {
                let mut positions = (0..self.0.len()).map(|i| {
                   (i / $col, i % $col)
                });
                loop {
                    if let Some((r, c)) = positions.next() {
                        if r == c {
                            if r < $row - 1 {
                                // Consume the rest of the row to avoid double swapping
                                let _ = positions.nth($row - r - 2);
                            }
                        } else {
                            let a = r * $col + c;
                            let b = c * $row + r;
                            //assert_eq!(self[(r, c)], self.0[a]);
                            //assert_eq!(self[(c, r)], self.0[b]);
                            self.0.swap(a, b);
                        }
                    } else {
                        break
                    }
                }
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

        impl Eq for $st {}

        impl PartialEq for $st {
            fn eq(&self, other: &$st) -> bool {
                self.0[..] == other.0[..]
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
    fn ident() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m = TestMatrix::identity();

        assert_eq!(m, TestMatrix([1, 0, 0, 1]));
    }

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
    fn transpose_mut() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let mut m = TestMatrix([1, 2, 3, 4]);
        m.transpose_mut();

        assert_eq!(m, TestMatrix([1, 3, 2, 4]));
    }

    #[test]
    fn transpose_large() {
        impl_matrix!(TestMatrix([i32; (6, 6)]));
        let m = TestMatrix([
                            0,  1,  2,  3,  4,  5,
                            6,  7,  8,  9, 10, 11,
                           12, 13, 14, 15, 16, 17,
                           18, 19, 20, 21, 22, 23,
                           24, 25, 26, 27, 28, 29,
                           30, 31, 32, 33, 34, 35,
        ]);
        let trans = m.transpose();

        assert_eq!(trans, TestMatrix([
                                 0,  6, 12, 18, 24, 30,
                                 1,  7, 13, 19, 25, 31,
                                 2,  8, 14, 20, 26, 32,
                                 3,  9, 15, 21, 27, 33,
                                 4, 10, 16, 22, 28, 34,
                                 5, 11, 17, 23, 29, 35,
        ]));
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

    #[test]
    fn large_matrix() {
        impl_matrix!(TestMatrixA([i32; (6, 6)]));
        let m_a = TestMatrixA([1; 36]);
        let m_b = TestMatrixA([1; 36]);

        assert_eq!(m_a, m_b);
    }
}
