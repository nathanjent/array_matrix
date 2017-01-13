/// Basic matrix trait.
pub trait ArrayMatrix {
    fn row(&self) -> usize;
    fn column(&self) -> usize;
    fn size(&self) -> (usize, usize);
}

/// Non-macro test implementation 
#[cfg(test)]
mod tests {
    use array_matrix::ArrayMatrix;
    use std::ops::{Index, IndexMut};
    use std::fmt;

    struct MyMatrix([f32; 9]);

    impl ArrayMatrix for MyMatrix {
        fn row(&self) -> usize {
            3
        }

        fn column(&self) -> usize {
            self.0.len() / self.row()
        }

        fn size(&self) -> (usize, usize) {
            (self.row(), self.column())
        }
    }

    impl Index<(usize, usize)> for MyMatrix {
        type Output = f32;

        #[inline]
        fn index(&self, (i, j): (usize, usize)) -> &f32 {
            assert!(i < self.row() && j < self.column());
            &self.0[i * self.column() + j]
        }
    }

    impl IndexMut<(usize, usize)> for MyMatrix {
        #[inline]
        fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f32 {
            let column_len = self.column();
            assert!(i < self.row() && j < column_len);
            &mut self.0[i * column_len + j]
        }
    }

    impl fmt::Debug for MyMatrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.0.iter()).finish()
        }
    }

    #[test]
    fn test_trait() {
        let mut m = MyMatrix([3.; 9]);
        m[(2,1)] = 8.1;
        println!("{:?}", m);
        println!("{}", m.row());
        assert_eq!(m.row(), 3);
        assert_eq!(m.column(), 3);
    }
}
