use std::any::TypeId;
use std::fs::{File};
use ndarray::{ArrayBase, Array1, Array2, OwnedRepr, AsArray, IxDyn, Array, Dimension};
use ndarray_npy::{ReadableElement, ReadNpyExt};
use crate::fast_spykes::io::load_file;

// pub struct NumpyArr<'a, T, A: AsArray<'a, T>> {
//     data: dyn A,
// }


pub struct NumpyArr<T: ReadableElement> {
    data: Array<T, IxDyn>
}

impl<T: ReadableElement> NumpyArr<T> {
    pub fn from_filename(filename: &str) -> Result<Self, String> {
        let file = load_numpy(filename)?;
       // let dims: i32 = D::get_dimensions();

        return match Array::<T, IxDyn>::read_npy(file) {
            Ok(data) => Ok(NumpyArr{data}),
            _ => panic!("asdfads")
        };
    }

    
}
pub fn load_numpy(filename: &str) -> Result<File, String> {
    let file = load_file(filename, |_| {Ok(())})?;
    return Ok(file);
}

// impl<'a, T: ReadableElement, A: AsArray<'a, T>> NumpyArr<T, A> {
//     pub fn from_filename(filename: &str) -> Result<Self, String> {
//         let file = load_numpy(filename)?;
//         return match A::<T>::read_npy(file) {
//             Ok(d) =>
//              Ok(NumpyArr{
//                  data: d
//              }),
//             Err(e) => Err(format!("{}", e))
//         };
//     }
// }


// pub fn load_arr2<T: ReadableElement>(filename: &str) -> Result<NumpyArr<T>, String> {
//     let file = load_numpy(filename)?;
//     return match Array2::<T>::read_npy(file) {
//         Ok(t) => Ok(t.slice([.., 0])),
//         Err(e) => Err(format!("{}", e))
//     };
// }
