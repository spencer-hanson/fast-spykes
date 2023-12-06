use std::fs::File;
use std::io::Read;


fn main() {
    let dir = "C:\\Users\\spenc\\Documents\\GitHub\\simple-spykes\\data\\Record Node 105\\experiment1\\recording1\\continuous\\Neuropix-PXI-104.ProbeA-AP\\continuous.dat";

    // TODO Check exists, and is file not directory

    let f = File::open(dir); // TODO Check file size fits byte size
    match f {
        Ok(mut v) => {
            let mut buf: [u8; 2] = [0; 2];
            v.read_exact(&mut buf).ok();
            let v: u16 = u16::from_le_bytes(buf);
            print!("{v}");

        },
        Err(err) => {
            panic!("Cannot open file: {}", err);
        }
    }

}