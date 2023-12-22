use std::fs;
use std::fs::{File, Metadata};
use ndarray_npy::ReadableElement;

pub mod numpy;

pub fn load_file(filename: &str, filecheck: fn(Metadata) -> Result<(), String>) -> Result<File, String> {
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