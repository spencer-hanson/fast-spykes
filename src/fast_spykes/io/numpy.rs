use std::fs::{File};
use ndarray::{IxDyn, Array, NdIndex};
use ndarray_npy::{ReadableElement, ReadNpyExt};
use crate::fast_spykes::io::{FileArray, load_file};

pub struct NumpyArray<T: ReadableElement> {
    data: Array<T, IxDyn>
}

impl<T: ReadableElement> NumpyArray<T> {
    pub fn from_filename(filename: &str) -> Result<Self, String> {
        let file = load_numpy_file(filename)?;

        return match Array::<T, IxDyn>::read_npy(file) {
            Ok(data) =>
                Ok(NumpyArray {
                    data
                }),
            Err(e) => {
                Err(format!("Error reading .npy file '{}'! Error: {:?}", filename, e))
            }
        };
    }
}

impl<T: ReadableElement + Clone> FileArray<T> for NumpyArray<T> {
    fn get(&mut self, idx_vec: Vec<usize>) -> T {
        return self.data.get(idx_vec.as_slice()).unwrap().clone();
    }
}
pub fn load_numpy_file(filename: &str) -> Result<File, String> {
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
