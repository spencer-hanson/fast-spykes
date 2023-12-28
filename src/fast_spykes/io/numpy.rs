use std::fs::{File};
use std::io::Read;
use std::marker::PhantomData;
use ndarray::{IxDyn, Array, NdIndex};
use ndarray_npy::{ReadableElement, ReadNpyError, ReadNpyExt};
use crate::fast_spykes::io::{FileArray, load_file};

pub trait Number: Copy + ReadableElement {
    fn to_f64(self) -> f64;
}

impl Number for i32 { fn to_f64(self) -> f64 {self.into()} }
impl Number for u32 { fn to_f64(self) -> f64 {self.into()} }
impl Number for f64 { fn to_f64(self) -> f64 { self }}
impl Number for i64 { fn to_f64(self) -> f64 { self as f64}}

enum DataNum {
    D1,
    D2
}

pub struct NumpyArray<T1: Number, T2: Number> {
    data_t1: Option<Array<T1, IxDyn>>,
    data_t2: Option<Array<T2, IxDyn>>,
    filename: String,
    data_num: DataNum,
    phantom: PhantomData<T1>,
}

impl<'a, T1: Number, T2: Number> NumpyArray<T1, T2> {
    // T1 - first type to try, T2 - second type to try, F - final type used for output (must be castable from T1 and T2)
    pub fn from_filename(filename: &str) -> Result<Self, String> {
        let t1_err: ReadNpyError;

        match Array::<T1, IxDyn>::read_npy(load_numpy_file(filename)?) {
            Ok(t) => return Ok(NumpyArray{
                data_t1: Some(t),
                data_t2: None,
                filename: String::from(filename),
                data_num: DataNum::D1,
                phantom: PhantomData::default()
            }),
            Err(e) => t1_err = e
        }

        return match Array::<T2, IxDyn>::read_npy(load_numpy_file(filename)?) {
            Ok(t) => return Ok(NumpyArray{
                data_t1: None,
                data_t2: Some(t),
                filename: String::from(filename),
                data_num: DataNum::D2,
                phantom: PhantomData::default()
            }),
            Err(e) => {
                Err(format!("Error reading .npy file '{}'! First try Error: \"{:?}\" Second try error: {:?}", filename, t1_err, e))
            }
        };
    }

    fn get_slice<'b, T:  Number>(data: &'b Array<T, IxDyn>, slice: &[usize], filename: &str) -> f64 {
        let mut newslice = slice;
        let a = [slice[0], 0];

        if data.shape().len() == 2 && data.shape()[1] == 1 {
            newslice = &a;
        }
        return data.get(newslice).expect(&format!("Unable to get index {:?} from array file '{}'", slice, filename)).to_owned().to_f64();
    }
}

impl<'a, T1: Number, T2: Number> FileArray for NumpyArray<T1, T2> {
    fn get(&mut self, idx_vec: Vec<usize>) -> f64 {
        let slice: &[usize] = idx_vec.as_slice();

        return match self.data_num {
            DataNum::D1 => {
                let data = self.data_t1.as_mut().unwrap();
                NumpyArray::<T1, T2>::get_slice::<T1>(data, slice, &self.filename)
            },
            DataNum::D2 => {
                let data = self.data_t2.as_mut().unwrap();
                NumpyArray::<T1, T2>::get_slice::<T2>(data, slice, &self.filename)
            }
        };
    }
    fn shape(&self) -> Vec<usize> {
        return match self.data_num {
            DataNum::D1 => self.data_t1.as_ref().unwrap().shape().to_owned(),
            DataNum::D2 => self.data_t2.as_ref().unwrap().shape().to_owned()
        };
    }

    fn len(&self) -> usize {
        return match self.data_num {
            DataNum::D1 => self.data_t1.as_ref().unwrap().len(),
            DataNum::D2 => self.data_t2.as_ref().unwrap().len()
        };
    }
}

pub fn load_numpy_file(filename: &str) -> Result<File, String> {
    let file = load_file(filename, |_| {Ok(())})?;
    return Ok(file);
}

