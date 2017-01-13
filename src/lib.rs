use std::ops::{Index, IndexMut};

trait Matrix {
    fn row(&self) -> usize;
    fn column(&self) -> usize;
    fn size(&self) -> (usize, usize);
}

// TODO implement
//
// A macro that takes a struct-like definition that takes an array type and row
// length arguments and implements the required methods.
//
// Usage:
//     impl_matrix!{
//         struct MyMatrix([f32; 9], 3);
//     };
//
//     let mut matrix =  MyMatrix([0.0; 9]);
//     matrix[(2,1)] = 8.1;
//     println!("{:?}", matrix);
//     println!("{}", m.row());
//
#[macro_export]
macro_rules! impl_matrix {
    ($struct_name:ident ([$elem:ty; $len:expr], $row_len:expr)) => {
        {
            #[derive(Debug)]
            struct $struct_name([$elem; $len]);

            impl Matrix for $struct_name {
                fn row(&self) -> usize {
                    $row_len
                }

                fn column(&self) -> usize {
                    self.0.len() / self.row()
                }

                fn size(&self) -> (usize, usize) {
                    (self.row(), self.column())
                }
            }

            impl Index<(usize, usize)> for $struct_name {
                type Output = $elem;

                #[inline]
                fn index(&self, (i, j): (usize, usize)) -> &$elem {
                    assert!(i < self.row() && j < self.column());
                    &self.0[i * self.column() + j]
                }
            }

            impl IndexMut<(usize, usize)> for $struct_name {
                #[inline]
                fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut $elem {
                    let column_len = self.column();
                    assert!(i < self.row() && j < column_len);
                    &mut self.0[i * column_len + j]
                }
            }
        }
    }
}


// Non-macro test implementation 

#[derive(Debug)]
struct MyMatrix([f32; 9]);

impl Matrix for MyMatrix {
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

#[test]
fn test_matrix() {
    let mut m = MyMatrix([3.; 9]);
    m[(2,1)] = 8.1;
    println!("{:?}", m);
    println!("{}", m.row());
    assert_eq!(m.row(), 3);
    assert_eq!(m.column(), 3);
}

macro_rules! struct_test {
    ($st:ident([$t:ty; ($row:expr, $col:expr)])) => {
        #[derive(Debug)]
        struct $st([$t; $row * $col]);

        //impl Matrix for $st {
        //    fn row(&self) -> usize {
        //        $row
        //    }
        //
        //    fn column(&self) -> usize {
        //        $col
        //    }
        //
        //    fn size(&self) -> (usize, usize) {
        //        (self.row(), self.column())
        //    }
        //}

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

#[test]
fn test_test() {
    struct_test!(Test([i32; (3, 3)]));

    let mut test = Test([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{:?}", test);
    println!("{:?}", test[(1, 2)]);
    test[(1, 2)] = 30;
    println!("{:?}", test[(1, 2)]);
}

//#[test]
//fn test_impl_matrix() {
//    impl_matrix!(MacroMatrix([i32; 12], 3));
//
//    let mut m = MacroMatrix([42; 12]);
//    m[(2,2)] = 0.1;
//    println!("{:?}", m);
//    println!("{}", m.row());
//    assert_eq!(m.row(), m.column());
//}