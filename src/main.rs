use std::fs;
use std::fs::File;
use std::io::Read;


fn main() {
    let dir = "C:\\Users\\spenc\\Documents\\GitHub\\simple-spykes\\data\\Record Node 105\\experiment1\\recording1\\continuous\\Neuropix-PXI-104.ProbeA-AP\\continuous.dat";
    let metadata = fs::metadata(dir);
    let file_size: u64;
    let num_channels: u64 = 384;
    // Check file exists and isn't a directory
    match metadata {
        Ok(m) => {
            if m.is_dir() {
                panic!("Cannot use provided path '{}' Is a directory!", dir);
            } else {
                file_size = m.len();
                println!("File Size: '{}'", file_size);
                if file_size % 2*num_channels != 0 {
                    panic!("File is not in expected format! Total bytes should be divisible by 2*num_channels! File size: '{}'", file_size);
                }
            }
        }
        Err(err) => {
            panic!("Error reading/finding file! Does '{}' exist? Error: '{}'", dir, err);
        }
    }
    /*
    Binary format spec from:
    https://open-ephys.github.io/gui-docs/User-Manual/Recording-data/Binary-format.html
    A simple binary file containing N channels x M samples 16-bit integers in little-endian format.
    Data is saved as ch1_samp1, ch2_samp1, ... chN_samp1, ch1_samp2, ch2_samp2, ..., chN_sampM.
    The value of the least significant bit needed to convert the 16-bit integers to physical units
    is specified in the bitVolts field of the relevant channel in the structure.oebin JSON file
    */
    
    let f = File::open(dir);
    match f {
        Ok(mut file) => {
            let mut buf: [u8; 2] = [0; 2];
            file.read_exact(&mut buf).ok();
            let v: u16 = u16::from_le_bytes(buf);
            print!("{v}");

        },
        Err(err) => {
            panic!("Cannot open file: {}", err);
        }
    }

}