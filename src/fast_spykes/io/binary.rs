use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use ndarray::{IxDyn, NdIndex};
use crate::fast_spykes::io::{FileArray, load_file};

pub fn load_binary_file(filename: &str, num_channels: usize) -> Result<File, String> {
    let file = load_file(filename, |m| {
        if m.len() % (num_channels as u64) * 2 != 0 {
            Err(format!("File is not in expected format! Total bytes should be divisible by 2*num_channels! File size: '{}'", m.len()))
        } else {
            Ok(())
        }
    })?;
    return Ok(file);
}

pub struct BinaryArray {
    file: File,
    pub num_channels: usize,
    pub file_size: u64,
    pub samples_per_channel: u64
}

impl BinaryArray {
    /*
    Binary format spec from:
    https://open-ephys.github.io/gui-docs/User-Manual/Recording-data/Binary-format.html
    A simple binary file containing N channels x M samples 16-bit integers in little-endian format.
    Data is saved as ch1_samp1, ch2_samp1, ... chN_samp1, ch1_samp2, ch2_samp2, ..., chN_sampM.
    The value of the least significant bit needed to convert the 16-bit integers to physical units
    is specified in the bitVolts field of the relevant channel in the structure.oebin JSON file
    */
    pub fn from_filename(filename: &str, num_channels: usize) -> Self {
        println!("Loading binary file");
        let file = load_binary_file(filename, num_channels).unwrap();

        println!("Getting binary file size");
        let file_size = fs::metadata(filename).unwrap().len();

        println!("Creating binary arr struct");
        BinaryArray{
            file,
            num_channels,
            file_size,
            samples_per_channel: file_size/(num_channels as u64) * 2
        }
    }
}

impl FileArray for BinaryArray {
    fn get(&mut self, idx_vec: Vec<usize>) -> f64 {
        let mut buf: [u8; 2] = [0; 2];
        self.file.seek(SeekFrom::Start(idx_vec[0] as u64)).unwrap();
        self.file.read_exact(&mut buf).ok();
        return i16::from_le_bytes(buf) as f64;
        // self.cached_data.push(i16::from_le_bytes(buf));
        // return self.cached_data.get(self.cached_data.len()).unwrap();
    }

    fn shape(&self) -> Vec<usize> {
        return vec![self.len()];
    }

    fn len(&self) -> usize {
        return (self.file_size / 2) as usize;
    }
}
