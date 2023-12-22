mod fast_spykes;

use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use ndarray::Array2;
use rand::Rng;
// use crate::fast_spykes::io::numpy::{load_arr1, load_arr2};
use fast_spykes::io::numpy::{NumpyArr};

fn read_i16(file: &mut File) -> i16 {
    let mut buf: [u8 ; 2] = [0; 2];
    file.read_exact(&mut buf).ok();
    let v: i16 = i16::from_le_bytes(buf);
    return v;
    // println!("Raw: {v}");
    // println!("Adjusted: {}", v as f64 * 0.1949999928474426);
}


fn read_continuous() {
    let dir = "..\\simple-spykes\\data\\Record Node 105\\experiment1\\recording1\\continuous\\Neuropix-PXI-104.ProbeA-AP\\continuous.dat";
    let metadata = fs::metadata(dir);
    let file_size: u64;
    let num_channels: u64 = 384; // TODO Find this out?

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
    println!("Number of channels: {}", num_channels);
    println!("Number of samples per channel: {}", file_size/(num_channels*2));

    let f = File::open(dir);
    let mut data = vec![];

    match f {
        Ok(mut file) => {

            // file.seek(SeekFrom::Start((*rand)*2)).unwrap();
            data.push(read_i16(&mut file));
            println!("Data: {:?}", data);
        },
        Err(err) => {
            panic!("Cannot open file: {}", err);
        }
    }

}

fn read_kilosort() {
    /*
    Useful files (for all see https://github.com/kwikteam/phy-contrib/blob/master/docs/template-gui.md)

    - spike_times.npy - 1 x n_spikes arr with the times of each spike in *samples*
    - templates.npy - [nTemplates, nTimePoints, nTempChannels] single matrix giving the template
        shapes on the channels given in templates_ind.npy
    - spike_clusters.npy - [nSpikes, ] int32 vector giving the cluster identity of each spike.
        This file is optional and if not provided will be automatically created the first time you
        run the template gui, taking the same values as spike_templates.npy until you do any
        merging or splitting.
    - spike_templates.npy - [nSpikes, ] uint32 vector specifying the identity of the template that
        was used to extract each spike
    - amplitudes.npy - [nSpikes, ] double vector with the amplitude scaling factor that was applied
        to the template when extracting that spike

    Notes:
    - spike 'duration' is going to be the width of the waveform template for the given spike
    - To get spikes for a particular cluster
        1. Use spike_clusters.npy or spike_templates.npy for spike_num -> cluster_num
        2. Reverse to cluster_num -> spike_num
    - To get amplitudes for particular cluster
        1. cluster_num -> spike_num
        2. use amplitudes.npy for spike_num -> amplitude
    - Amplitude cutoff link https://github.com/AllenInstitute/ecephys_spike_sorting/blob/master/ecephys_spike_sorting/modules/quality_metrics/metrics.py#L627

    */

    let dir = "..\\simple-spykes\\data\\Record Node 105\\experiment1\\recording1\\continuous\\Neuropix-PXI-104.ProbeA-AP\\";

    let amplitudes = NumpyArr::<f64>::from_filename(&format!("{}{}", dir, "amplitudes.npy")).unwrap();
    // let amplitudes = NumpyArr::<f64, Array2>::from_filename(&format!("{}{}", dir, "amplitudes.npy")).unwrap();
    // let clusters = load_arr1::<i32>(&format!("{}{}", dir, "spike_clusters.npy")).unwrap();

    // println!("Amp len {}", amplitudes.len());
    // println!("Cluster len {}", clusters.len());

    // let file_len = arr.len();
    // let mut randoms = vec![];
    // let mut data: Vec<f64> = vec![];
    //
    // let mut rng = rand::thread_rng();
    // randoms.append(&mut vec![0]);
    // data.push(arr[[0, 0]]);
    //
    // for _ in 0..100 {
    //     let num = rng.gen_range(0..file_len);
    //     randoms.push(num);
    //     data.push(arr[[num, 0]]);
    // }
    //
    // println!("gen_vals = {:?}", data);
    // println!("idxes = {:?}", randoms);
    //
    // println!("arr len {}", data.len());
}

fn main() {
    read_kilosort();
}