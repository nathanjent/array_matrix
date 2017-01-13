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
        use std::ops::{Index, IndexMut};

        #[derive(Debug)]
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
    }
}

trait ArrayMatrix {
    fn row(&self) -> usize;
    fn column(&self) -> usize;
    fn size(&self) -> (usize, usize);
}

// Non-macro test implementation 

//#[derive(Debug)]
//struct MyMatrix([f32; 9]);
//
//impl Matrix for MyMatrix {
//    fn row(&self) -> usize {
//        3
//    }
//
//    fn column(&self) -> usize {
//        self.0.len() / self.row()
//    }
//
//    fn size(&self) -> (usize, usize) {
//        (self.row(), self.column())
//    }
//}
//
//impl Index<(usize, usize)> for MyMatrix {
//    type Output = f32;
//
//    #[inline]
//    fn index(&self, (i, j): (usize, usize)) -> &f32 {
//        assert!(i < self.row() && j < self.column());
//        &self.0[i * self.column() + j]
//    }
//}
//
//impl IndexMut<(usize, usize)> for MyMatrix {
//    #[inline]
//    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f32 {
//        let column_len = self.column();
//        assert!(i < self.row() && j < column_len);
//        &mut self.0[i * column_len + j]
//    }
//}
//
//#[test]
//fn test_matrix() {
//    let mut m = MyMatrix([3.; 9]);
//    m[(2,1)] = 8.1;
//    println!("{:?}", m);
//    println!("{}", m.row());
//    assert_eq!(m.row(), 3);
//    assert_eq!(m.column(), 3);
//}

#[test]
fn test_impl_matrix() {
    const ROW: usize = 3;
    const COLUMN: usize = 3;

    impl_matrix!(TestMatrix([i32; (ROW, COLUMN)]));

    let mut m = TestMatrix([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    
    println!("{:?}", m);

    assert_eq!(m[(1, 2)], 6);
    m[(1, 2)] = 30;
    assert_eq!(m[(1, 2)], 30);
    assert_eq!(m.row(), ROW);
    assert_eq!(m.column(), COLUMN);
}
