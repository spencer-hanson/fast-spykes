use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use rand::Rng;
use serde::Serialize;
use crate::fast_spykes::dataset::Dataset;
use crate::fast_spykes::io::{FileArray, load_file};
use crate::get_datasets;


fn rand_idx(fsize: &usize) -> usize {
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..*fsize);
    return num;
}

fn get_rand_idxs(samples: usize, siz: usize) -> Vec<usize> {
    let mut idxes = vec![];
    for _ in 0..samples {
        idxes.push(rand_idx(&siz));
    }

    return idxes;
}

fn get_arr_by_idxs(mut filearr: Box<dyn FileArray>, idxs: &Vec<usize>) -> Vec<f64> {
    let mut vals = vec![];
    for idx_num in 0..idxs.len() {
        // println!("Idx num {}", idx_num);
        let v_idx = vec![idxs[idx_num]];
        let v = filearr.get(v_idx.clone());
        // println!("Read idx {:?} Value {}", v_idx, v);
        vals.push(v);
    }

    return vals;
}


#[derive(Serialize)]
struct DatasetSamples {
    curated_amps: Vec<f64>,
    curated_clusts: Vec<f64>,
    raw_amps: Vec<f64>,
    raw_clusts: Vec<f64>,
    raw_samps: Vec<f64>
}

pub fn dataset_to_testfile(dataset: Box<Dataset>) {
    let num_idxs = 1000;
    let filename = format!("{}.json", dataset.filename);

    let curated_indexes = get_rand_idxs(num_idxs, dataset.curated_spikes);
    let raw_indexes = get_rand_idxs(num_idxs, dataset.raw_spikes);
    let sample_indexes = get_rand_idxs(num_idxs, dataset.raw_samples);


    println!("Getting curated amplitudes");
    let curated_amps = get_arr_by_idxs(dataset.curated_amplitudes, &curated_indexes);

    println!("Getting curated clusters");
    let curated_clusts = get_arr_by_idxs(dataset.curated_clusters, &curated_indexes);

    println!("Getting raw amplitudes");
    let raw_amps = get_arr_by_idxs(dataset.raw_amplitudes, &raw_indexes);

    println!("Getting raw clusters");
    let raw_clusts = get_arr_by_idxs(dataset.raw_clusters, &raw_indexes);

    println!("Getting raw samples");
    let raw_samps = get_arr_by_idxs(dataset.raw, &sample_indexes);

    let samples = DatasetSamples {
        curated_amps,
        curated_clusts,
        raw_amps,
        raw_clusts,
        raw_samps
    };

    let s = serde_json::to_string(&samples).unwrap();
    fs::write(filename, s).unwrap();

    println!("Done");
}


// fn main() {
//     let dir = "..\\simple-spykes\\data\\Record Node 105\\experiment1\\recording1\\continuous\\Neuropix-PXI-104.ProbeA-AP\\continuous.dat";
//     let metadata = fs::metadata(dir);
//     let file_size: u64;
//     let num_channels: u64 = 384; // TODO Find this out?
//
//     // Check file exists and isn't a directory
//     match metadata {
//         Ok(m) => {
//             if m.is_dir() {
//                 panic!("Cannot use provided path '{}' Is a directory!", dir);
//             } else {
//                 file_size = m.len();
//                 println!("File Size: '{}'", file_size);
//                 if file_size % 2*num_channels != 0 {
//                     panic!("File is not in expected format! Total bytes should be divisible by 2*num_channels! File size: '{}'", file_size);
//                 }
//             }
//         }
//         Err(err) => {
//             panic!("Error reading/finding file! Does '{}' exist? Error: '{}'", dir, err);
//         }
//     }
//     /*
//     Binary format spec from:
//     https://open-ephys.github.io/gui-docs/User-Manual/Recording-data/Binary-format.html
//     A simple binary file containing N channels x M samples 16-bit integers in little-endian format.
//     Data is saved as ch1_samp1, ch2_samp1, ... chN_samp1, ch1_samp2, ch2_samp2, ..., chN_sampM.
//     The value of the least significant bit needed to convert the 16-bit integers to physical units
//     is specified in the bitVolts field of the relevant channel in the structure.oebin JSON file
//     */
//     println!("Number of channels: {}", num_channels);
//     println!("Number of samples per channel: {}", file_size/(num_channels*2));
//
//
//     // let file_len = arr.len();
//     // let mut randoms = vec![];
//     // let mut data: Vec<f64> = vec![];
//     //
//     // let mut rng = rand::thread_rng();
//     // randoms.append(&mut vec![0]);
//     // data.push(arr[[0, 0]]);
//     //
//     // for _ in 0..100 {
//     //     let num = rng.gen_range(0..file_len);
//     //     randoms.push(num);
//     //     data.push(arr[[num, 0]]);
//     // }
//     //
//     // println!("gen_vals = {:?}", data);
//     // println!("idxes = {:?}", randoms);
//     //
//     // println!("arr len {}", data.len());
//
//
//     let f = File::open(dir);
//     let mut data = vec![];
//     let mut randoms = vec![];
//     randoms.append(&mut vec![0]);
//     for _ in 0..100 {
//         let rand = rand_idx(file_size);
//         randoms.push(rand);
//     }
//
//
//     match f {
//         Ok(mut file) => {
//             for rand in randoms.iter() {
//                 file.seek(SeekFrom::Start((*rand)*2)).unwrap();
//                 data.push(read_i16(&mut file));
//             }
//             // TODO Check these outputs against NEO, see link for exact line info (currently manual)
//             // https://github.com/NeuralEnsemble/python-neo/blob/51063dbf581cc6aaeb35858023a147acc1b66ccf/neo/rawio/openephysbinaryrawio.py#L149
//             /* PYTHON CHECK CODE
//             idxes = [<randoms>]
//             g = memmap, NOT .reshape'd
//             myprog = <data>
//
//             assert all([g[idxes[i]] == myprog[i] for i in range(len(idxes))])
//             */
//
//             println!("Data: {:?}", data);
//             println!("Rands: {:?}", randoms);
//         },
//         Err(err) => {
//             panic!("Cannot open file: {}", err);
//         }
//     }
//
//     // TODO automate check with python?
//
//
// }