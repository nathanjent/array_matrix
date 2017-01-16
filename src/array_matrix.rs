/// Basic matrix trait.
pub trait ArrayMatrix {
    /// Get the row length.
    fn row(&self) -> usize;
    /// Get the column length.
    fn column(&self) -> usize;
    /// Get row and column length as a tuple.
    fn size(&self) -> (usize, usize);
    /// Get transpose of the matrix.
    fn transpose(&self) -> Self;
    /// Transpose in place.
    fn transpose_mut(&mut self);
    /// Swaps two elements in a matrix.
    fn swap(&mut self, a: (usize, usize), b: (usize, usize));
}

// Non-macro test implementation 
// This is where new features are tested before migrating into the macro
#[cfg(test)]
mod tests {
    use array_matrix::ArrayMatrix;
    use std::ops::{Index, IndexMut};
    use std::fmt;

    struct NonMacroMatrix([f32; 9]);

    #[allow(dead_code)]
    impl NonMacroMatrix {
        fn identity() -> NonMacroMatrix {
            let mut m = NonMacroMatrix([0.; 9]);
            for i in 0..m.row() {
                m[(i, i)] = 1.;
            }
            m
        }
    }

    impl ArrayMatrix for NonMacroMatrix {
        fn row(&self) -> usize {
            3
        }

        fn column(&self) -> usize {
            self.0.len() / self.row()
        }

        fn size(&self) -> (usize, usize) {
            (self.row(), self.column())
        }

        fn swap(&mut self, (a_i, a_j): (usize, usize), (b_i, b_j): (usize, usize)) {
            let cols = self.column();
            self.0.swap(a_i * cols + a_j, b_i * cols + b_j);
        }

        fn transpose(&self) -> Self {
            let mut trans = NonMacroMatrix([0f32; 9]);
            for i in 0..self.0.len() {
               let r = i / self.column();
               let c = i % self.column();
               //println!("({0}, {1}): {2} <-> ({1}, {0}): {3}",
               //    r, c, self[(r, c)], self[(c, r)]);
               trans[(c, r)] = self[(r, c)].clone();
            }
            trans
        }

        fn transpose_mut(&mut self) {
            let rows = self.row();
            let cols = self.column();
            let mut positions = (0..self.0.len()).map(|i| {
               (i / cols, i % cols)
            });
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

    impl Index<(usize, usize)> for NonMacroMatrix {
        type Output = f32;

        #[inline]
        fn index(&self, (i, j): (usize, usize)) -> &f32 {
            assert!(i < self.row() && j < self.column());
            &self.0[i * self.column() + j]
        }
    }

    impl IndexMut<(usize, usize)> for NonMacroMatrix {
        #[inline]
        fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f32 {
            let column_len = self.column();
            assert!(i < self.row() && j < column_len);
            &mut self.0[i * column_len + j]
        }
    }

    impl fmt::Debug for NonMacroMatrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.0.iter()).finish()
        }
    }

    impl PartialEq for NonMacroMatrix {
        fn eq(&self, other: &NonMacroMatrix) -> bool {
            self.0 == other.0
        }
    }

    #[test]
    fn test_trait() {
        let mut m = NonMacroMatrix([3.; 9]);
        m[(2,1)] = 8.1;
        //println!("{:?}", m);
        //println!("{}", m.row());
        assert_eq!(m.row(), 3);
        assert_eq!(m.column(), 3);
    }

    #[test]
    fn transpose_mut() {
        let mut m = NonMacroMatrix([
                                   1., 2., 3.,
                                   4., 5., 6.,
                                   7., 8., 9.,
        ]);
        m.transpose_mut();

        assert_eq!(m, NonMacroMatrix([
                                 1., 4., 7.,
                                 2., 5., 8.,
                                 3., 6., 9.,
        ]));
    }
}
