//! A macro that takes a struct-like definition with array type and a
//! tuple of (row, column) size arguments and implements the
//! [`ArrayMatrix`](trait.ArrayMatrix.html) trait.
///
/// Example:
///
/// ```
/// # #[macro_use] extern crate array_matrix;
///
/// # fn main() {
/// // Include traits that will be implemented in the macro.
/// use array_matrix::ArrayMatrix;
/// use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
/// use std::fmt; // Debug impl in macro needs fmt
///
/// impl_matrix!(MyMatrix([f32; (3, 3)]));
///
/// let mut matrix = MyMatrix([0.0; 9]);
///
/// // Indexing is available
/// matrix[(2,1)] = 8.1;
/// matrix[(0,2)] += 3.1;
/// matrix[(1,1)] -= 0.1;
///
/// // Standard ops too
/// matrix += 8.;
///
/// assert_eq!(matrix,
///     MyMatrix([
///         8.0, 8.0, 11.1,
///         8.0, 7.9, 8.0,
///         8.0, 16.1, 8.0,
///     ])
/// );
///
/// # }
/// ```
///
#[macro_export]
macro_rules! impl_matrix {
    ($st:ident([$t:ty; ($row:expr, $col:expr)])) => {
        struct $st([$t; $row * $col]);

        impl $st {
            // Create matrix from an array of type [$t; $row * $col]
            #[allow(dead_code)]
            fn from_array(array: [$t; $row * $col]) -> Self {
                $st(array)
            }

            // Creates an identity matrix.
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
                let mut positions = (0..self.0.len()).map(|i| (i / $col, i % $col));
                loop {
                    if let Some((r, c)) = positions.next() {
                        //println!("({}, {}) {}", r, c, self[(r, c)]);
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
                        break;
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

        impl Add for $st {
            type Output = $st;

            fn add(self, other: $st) -> $st {
                let mut a = [0 as $t; $row * $col];
                for i in 0..a.len() {
                    a[i] = self.0[i].clone() + other.0[i].clone();
                }
                $st(a)
            }
        }

        impl Add<$t> for $st {
            type Output = $st;

            fn add(self, other: $t) -> $st {
                let mut a = [0 as $t; $row * $col];
                for i in 0..a.len() {
                    a[i] = self.0[i].clone() + other;
                }
                $st(a)
            }
        }

        impl AddAssign for $st {
            fn add_assign(&mut self, other: $st) {
                for i in 0..self.0.len() {
                    self.0[i] += other.0[i];
                }
            }
        }

        impl AddAssign<$t> for $st {
            fn add_assign(&mut self, other: $t) {
                for i in 0..self.0.len() {
                    self.0[i] += other;
                }
            }
        }

        impl Sub for $st {
            type Output = $st;

            fn sub(self, other: $st) -> $st {
                let mut a = [0 as $t; $row * $col];
                for i in 0..a.len() {
                    a[i] = self.0[i].clone() - other.0[i].clone();
                }
                $st(a)
            }
        }

        impl Sub<$t> for $st {
            type Output = $st;

            fn sub(self, other: $t) -> $st {
                let mut a = [0 as $t; $row * $col];
                for i in 0..a.len() {
                    a[i] = self.0[i].clone() - other;
                }
                $st(a)
            }
        }

        impl SubAssign for $st {
            fn sub_assign(&mut self, other: $st) {
                for i in 0..self.0.len() {
                    self.0[i] -= other.0[i];
                }
            }
        }

        impl SubAssign<$t> for $st {
            fn sub_assign(&mut self, other: $t) {
                for i in 0..self.0.len() {
                    self.0[i] -= other;
                }
            }
        }

        impl<T> Mul<T> for $st
        where
            T: ArrayMatrix + Index<(usize, usize), Output = $t>,
        {
            type Output = $st;

            fn mul(self, other: T) -> $st {
                assert_eq!(self.row(), other.column());
                let mut result = $st([0 as $t; $row * $col]);
                let mut positions =
                    (0..result.0.len()).map(|i| (i / self.column(), i % self.column()));

                loop {
                    if let Some((i, j)) = positions.next() {
                        let mut sum = self[(i, 0)].clone() * other[(0, j)].clone();
                        for k in 1..other.row() {
                            sum += self[(i, k)].clone() * other[(k, j)].clone();
                        }
                        result[(i, j)] = sum;
                    } else {
                        break;
                    }
                }
                result
            }
        }

        impl Mul<$t> for $st {
            type Output = $st;

            fn mul(self, other: $t) -> $st {
                let mut a = [0 as $t; $row * $col];
                for i in 0..a.len() {
                    a[i] = self.0[i].clone() * other;
                }
                $st(a)
            }
        }

        impl MulAssign<$t> for $st {
            fn mul_assign(&mut self, other: $t) {
                for i in 0..self.0.len() {
                    self.0[i] *= other;
                }
            }
        }

        impl Div<$t> for $st {
            type Output = $st;

            fn div(self, other: $t) -> $st {
                let mut a = [0 as $t; $row * $col];
                for i in 0..a.len() {
                    a[i] = self.0[i].clone() / other;
                }
                $st(a)
            }
        }

        impl DivAssign<$t> for $st {
            fn div_assign(&mut self, other: $t) {
                for i in 0..self.0.len() {
                    self.0[i] /= other;
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::array_matrix::ArrayMatrix;
    use std::fmt;
    use std::ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign,
    };

    #[test]
    fn ident() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m = TestMatrix::identity();

        assert_eq!(m, TestMatrix::from_array([1, 0, 0, 1]));
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
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
        ]);
        let trans = m.transpose();

        assert_eq!(
            trans,
            TestMatrix([
                0, 6, 12, 18, 24, 30, 1, 7, 13, 19, 25, 31, 2, 8, 14, 20, 26, 32, 3, 9, 15, 21, 27,
                33, 4, 10, 16, 22, 28, 34, 5, 11, 17, 23, 29, 35
            ])
        );
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
    fn add() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m_a = TestMatrix([1, 2, 3, 4]);
        let m_b = TestMatrix([1, 2, 3, 4]);
        let m_c = m_a + m_b;

        assert_eq!(m_c, TestMatrix([2, 4, 6, 8]));
    }

    #[test]
    fn add_assign() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let mut m_a = TestMatrix([1, 2, 3, 4]);
        let m_b = TestMatrix([1, 2, 3, 4]);
        m_a += m_b;

        assert_eq!(m_a, TestMatrix([2, 4, 6, 8]));
    }

    #[test]
    fn add_scalar() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m_a = TestMatrix([1, 2, 3, 4]);
        let m_b = m_a + 1;

        assert_eq!(m_b, TestMatrix([2, 3, 4, 5]));
    }

    #[test]
    fn add_scalar_assign() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let mut m_a = TestMatrix([1, 2, 3, 4]);
        m_a += 1;

        assert_eq!(m_a, TestMatrix([2, 3, 4, 5]));
    }

    #[test]
    fn subtract() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m_a = TestMatrix([1, 2, 3, 4]);
        let m_b = TestMatrix([1, 2, 3, 4]);
        let m_c = m_a - m_b;

        assert_eq!(m_c, TestMatrix([0, 0, 0, 0]));
    }

    #[test]
    fn subtract_assign() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let mut m_a = TestMatrix([1, 2, 3, 4]);
        let m_b = TestMatrix([1, 2, 3, 4]);
        m_a -= m_b;

        assert_eq!(m_a, TestMatrix([0, 0, 0, 0]));
    }

    #[test]
    fn subtract_scalar() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m_a = TestMatrix([1, 2, 3, 4]);
        let m_b = m_a - 1;

        assert_eq!(m_b, TestMatrix([0, 1, 2, 3]));
    }

    #[test]
    fn subtract_scalar_assign() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let mut m_a = TestMatrix([1, 2, 3, 4]);
        m_a -= 1;

        assert_eq!(m_a, TestMatrix([0, 1, 2, 3]));
    }

    //    #[test]
    //    fn multiply() {
    //        impl_matrix!(TestMatrix([i32; (2, 2)]));
    //        let m_a = TestMatrix([1, 2, 3, 4]);
    //        let m_b = TestMatrix([1, 2, 3, 4]);
    //        let m_c = m_a * m_b;
    //
    //        assert_eq!(m_c[..], [7, 22]);
    //    }

    #[test]
    fn multiply_scalar() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m_a = TestMatrix([1, 2, 3, 4]);
        let m_b = m_a * 3;

        assert_eq!(m_b, TestMatrix([3, 6, 9, 12]));
    }

    #[test]
    fn multiply_scalar_assign() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let mut m_a = TestMatrix([1, 2, 3, 4]);
        m_a *= 3;

        assert_eq!(m_a, TestMatrix([3, 6, 9, 12]));
    }

    #[test]
    fn divide_scalar() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let m_a = TestMatrix([9, 12, 21, 36]);
        let m_b = m_a / 3;

        assert_eq!(m_b, TestMatrix([3, 4, 7, 12]));
    }

    #[test]
    fn divide_scalar_assign() {
        impl_matrix!(TestMatrix([i32; (2, 2)]));
        let mut m_a = TestMatrix([9, 12, 21, 36]);
        m_a /= 3;

        assert_eq!(m_a, TestMatrix([3, 4, 7, 12]));
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
