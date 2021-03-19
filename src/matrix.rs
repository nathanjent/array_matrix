/// Basic matrix trait.
pub trait Matrix<T> {
    type Output;

    /// Get the row length.
    fn row_len(&self) -> usize;

    /// Get the column length.
    fn column_len(&self) -> usize;

    /// Get row and column length as a tuple.
    fn size(&self) -> (usize, usize);

    /// Get transpose of the matrix.
    fn transpose(&self) -> Self
    where
        T: Copy;

    /// Transpose in place.
    fn transpose_mut(&mut self);

    /// Swaps two elements in a matrix.
    fn swap(&mut self, a: (usize, usize), b: (usize, usize));
}

#[derive(Debug, PartialEq)]
struct BasicMatrix<T> {
    inner: Vec<T>,
    row_len: usize,
}

impl<T> BasicMatrix<T> {
    pub fn new(row_len: usize) -> Self {
        Self {
            inner: Vec::new(),
            row_len,
        }
    }
}

impl<T> Matrix<T> for BasicMatrix<T> {
    type Output = T;
    fn row_len(&self) -> usize {
        self.row_len
    }

    fn column_len(&self) -> usize {
        self.inner.len() / self.row_len()
    }

    fn size(&self) -> (usize, usize) {
        (self.row_len(), self.column_len())
    }

    fn transpose(&self) -> Self
    where
        T: Copy,
    {
        let mut trans = BasicMatrix::new(self.row_len());
        for i in 0..self.inner.len() {
            let r = i / self.column_len();
            let c = i % self.column_len();
            // println!("({0}, {1}): {2} <-> ({1}, {0}): {3}",
            //    r, c, self[(r, c)], self[(c, r)]);
            trans[(c, r)] = self[(r, c)].clone();
        }
        trans
    }

    fn transpose_mut(&mut self) {
        let rows = self.row_len();
        let cols = self.column_len();
        let mut positions = (0..self.inner.len()).map(|i| (i / cols, i % cols));
        loop {
            if let Some((r, c)) = positions.next() {
                if r == c {
                    if r < rows - 1 {
                        // Consume the rest of the row to avoid double swapping
                        let _ = positions.nth(rows - r - 2);
                    }
                } else {
                    let a = r * cols + c;
                    let b = c * rows + r;
                    // assert_eq!(self[(r, c)], self.0[a]);
                    // assert_eq!(self[(c, r)], self.0[b]);
                    self.inner.swap(a, b);
                }
            } else {
                break;
            }
        }
    }

    fn swap(&mut self, (a_i, a_j): (usize, usize), (b_i, b_j): (usize, usize)) {
        let cols = self.column_len();
        self.inner.swap(a_i * cols + a_j, b_i * cols + b_j);
    }
}

use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

impl<T> Index<(usize, usize)> for BasicMatrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        assert!(i < self.row_len() && j < self.column_len());
        &self.inner[i * self.column_len() + j]
    }
}

impl<T> IndexMut<(usize, usize)> for BasicMatrix<T> {
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        let column_len = self.column_len();
        assert!(i < self.row_len() && j < column_len);
        &mut self.inner[i * column_len + j]
    }
}

impl<T: Add<Output = T>> Add for BasicMatrix<T>
where
    T: Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self
                .inner
                .iter()
                .enumerate()
                .map(|(i, lhs)| rhs.inner[i] + *lhs)
                .collect(),
            row_len: self.row_len(),
        }
    }
}

impl<T> AddAssign<T> for BasicMatrix<T>
where T: Copy + Add<Output = T>
{
    fn add_assign(&mut self, rhs: T) {
        *self = Self {
            inner: self
                .inner
                .iter()
                .map(|lhs| rhs + *lhs)
                .collect(),
            row_len: self.row_len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::BasicMatrix;

    #[test]
    fn add() {
        let lhs = BasicMatrix::<i32>::new(3);
        let rhs = BasicMatrix::<i32>::new(3);
        let expected = BasicMatrix::<i32>::new(3);
        assert_eq!(lhs + rhs, expected);
    }
}
