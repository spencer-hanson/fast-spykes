pub mod numpy;
pub mod binary;
pub mod elements;

use std::fs;
use std::fs::{File, Metadata};


pub trait FileArray {
    fn get(&mut self, idx_vec: Vec<usize>) -> f64;
    fn shape(&self) -> Vec<usize>;
    fn len(&self) -> usize; // length of full array, if N x M x ... return N*M*...
    fn get_filepath(&self) -> String;
}

pub fn load_file(filename: &str, filecheck: impl Fn(Metadata) -> Result<(), String>) -> Result<File, String> {
    let metadata = fs::metadata(filename);

    // Check file exists and isn't a directory
    match metadata {
        Ok(m) => {
            if m.is_dir() {
                return Err(format!("Cannot use provided path '{}' Is a directory!", filename))
            } else {
                filecheck(m)?
            }
        }
        Err(err) => {
            return Err(format!("Error reading/finding file! Does '{}' exist? Error: '{}'", filename, err));
        }
    }

    let f = File::open(filename);

    return match f {
        Ok(file) => Ok(file),
        Err(err) => {
            Err(format!("Cannot open file: {}", err))
        }
    }
}