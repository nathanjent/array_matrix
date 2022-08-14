use std::fmt;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

pub trait Matrix<Inner> {
    /// Get the row length.
    fn row_len(&self) -> usize;
    /// Get the column length.
    fn column_len(&self) -> usize;
}

pub struct VecMatrix<T> {
    inner: Vec<T>,
    row_len: usize,
}

impl<T> Matrix<T> for VecMatrix<T> {
    fn row_len(&self) -> usize {
        self.row_len
    }
    fn column_len(&self) -> usize {
        self.inner.len() / self.row_len()
    }
}

impl<T> Index<(usize, usize)> for VecMatrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        assert!(i < self.row_len() && j < self.column_len());
        &self.inner[i * self.column_len() + j]
    }
}

impl<T> IndexMut<(usize, usize)> for VecMatrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let column_len = self.column_len();
        assert!(i < self.row_len() && j < column_len);
        &mut self.inner[i * column_len + j]
    }
}

    impl<T: fmt::Debug> fmt::Debug for VecMatrix<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.inner.iter()).finish()
        }
    }

    impl<T: PartialEq> PartialEq for VecMatrix<T> {
        fn eq(&self, other: &VecMatrix<T>) -> bool {
            self.inner == other.inner
        }
    }

impl<T: Add<Output = T> + Copy> Add for VecMatrix<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner.iter()
                .zip(rhs.inner.iter())
                .map(|(&i, &j)| i + j)
                .collect(),
            row_len: self.row_len(),
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for VecMatrix<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output { 
        Self {
            inner: self.inner.iter()
                .map(|&i| i + rhs)
                .collect(),
            row_len: self.row_len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use matrix::VecMatrix;

    #[test]
    fn add_matrix_test() {
        let m1 = VecMatrix {
            inner: vec![
                1, 0,
                0, 1,
            ],
            row_len: 2,
        };
        let m2 = VecMatrix {
            inner: vec![
                0, 1,
                1, 0,
            ],
            row_len: 2,
        };
        assert_eq!(m1 + m2, VecMatrix {
            inner: vec![
                1, 1,
                1, 1,
            ],
            row_len: 2,
        });

    }
}
