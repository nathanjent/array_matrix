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
pub use array_matrix::ArrayMatrix;
mod array_matrix;

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
