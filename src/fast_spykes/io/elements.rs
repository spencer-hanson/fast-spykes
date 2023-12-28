// use std::fmt;
// use std::fmt::Formatter;
// use std::fs::File;
// use std::io::Read;
// use std::slice::Iter;
// use ndarray::{Array, Data, IxDyn};
// use ndarray_npy::{ReadableElement, ReadDataError, ReadNpyError, ReadNpyExt};
//
// pub enum DataTypes {
//     I32,
//     I64,
//
//     U32,
//     U64,
// }
//
// impl DataTypes {
//     pub fn to_str(&self) -> &str {
//         return match self {
//             DataTypes::I32 => "i32",
//             DataTypes::I64 => "i64",
//             DataTypes::U32 => "u32",
//             DataTypes::U64 => "u64"
//         }
//     }
// }
//
// impl fmt::Display for DataTypes {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         return write!(f, "{}", self.to_str())
//     }
// }
//
// // pub struct DataElement<T: ReadableElement> {
// //     element_type_checklist: Vec<DataTypes>,
// //     get_array: fn(File, Vec<DataTypes>) -> Array<T, IxDyn>
// // }
// //
// // impl<T: ReadableElement> DataElement<T> {
// //     fn new(v: Vec<DataTypes>) -> Self {
// //         return DataElement{
// //             element_type_checklist: v,
// //             get_array: DataElement::<T>::read_npy
// //         };
// //     }
// //     fn iter(&self) -> Iter<'_, DataTypes> {
// //         return self.element_type_checklist.iter();
// //     }
// //
// //     fn try_read<D: ReadableElement>(file: File) -> Result<Array<D, IxDyn>, ReadNpyError> {
// //         return Array::<T, IxDyn>::read_npy(file);
// //     }
// //
// //     pub fn read_npy(file: File, dtypes: Vec<DataTypes>) -> Array<T, IxDyn>{
// //
// //         let arr: Array<T, IxDyn>;
// //
// //         for dtype in dtypes.iter() {
// //             match dtype {
// //                 DataTypes::I32 => {
// //                     let g = DataElement::try_read::<i32>(file).unwrap();
// //
// //                     break;
// //                 },
// //                 // DataTypes:I64 => {v = Array::<i64, IxDyn>::read_npy(file); break;}
// //                 _ => todo!()
// //             };
// //
// //             match arr {
// //                 Err(e) => (),
// //                 _ => break
// //             }
// //
// //         }
// //
// //         todo!();
// //     }
// // }
//
// // impl ReadableElement for DataElement {
// //     fn read_to_end_exact_vec<R: Read>(reader: R, type_desc: &py_literal::Value, len: usize) -> Result<Vec<Self>, ReadDataError> {
// //         todo!()
// //     }
// // }