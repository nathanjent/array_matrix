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
}

// Non-macro test implementation 
// This is where new features are tested before migrating into the macro
#[cfg(test)]
mod tests {
    use array_matrix::ArrayMatrix;
    use std::ops::{Index, IndexMut};
    use std::fmt;

    struct NonMacroMatrix([f32; 9]);

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

        fn transpose(&self) -> Self {
            let mut trans = NonMacroMatrix([0f32; 9]);
            for i in 0..self.0.len() {
               let r = i / self.column();
               let c = i % self.column();
               trans[(c, r)] = self[(r, c)].clone();
            }
            trans
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
        println!("{:?}", m);
        println!("{}", m.row());
        assert_eq!(m.row(), 3);
        assert_eq!(m.column(), 3);
    }
}
