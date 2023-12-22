mod fast_spykes;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use rand::Rng;
use crate::fast_spykes::io::numpy::{NumpyArray};
use crate::fast_spykes::io::binary::{BinaryArray};
use crate::fast_spykes::io::FileArray;

fn read_i16(file: &mut File) -> i16 {
    let mut buf: [u8 ; 2] = [0; 2];
    file.read_exact(&mut buf).ok();
    let v: i16 = i16::from_le_bytes(buf);
    return v;
    // println!("Raw: {v}");
    // println!("Adjusted: {}", v as f64 * 0.1949999928474426);
}


fn read_continuous() {
    let continuous_filename = "..\\simple-spykes\\data\\Record Node 105\\experiment1\\recording1\\continuous\\Neuropix-PXI-104.ProbeA-AP\\continuous.dat";
    let num_channels: u64 = 384; // TODO Find this out?


    let mut continuous = BinaryArray::from_filename(continuous_filename, num_channels);
    let el: i16 = continuous.get(vec![100]);

    println!("Continuous {:?}", el);
    // Expect shape to be [samples*channels]

    println!("Number of channels: {}", continuous.num_channels);
    println!("Number of samples per channel: {}", continuous.samples_per_channel);
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

    // Expect shape to be [spikes, 0]
    let mut amplitudes: NumpyArray<f64> = NumpyArray::<f64>::from_filename(&format!("{}{}", dir, "amplitudes.npy")).unwrap();
    println!("First amplitude {:?}", amplitudes.get(vec![0, 0]));


    // Expect shape to be [spikes]
    let mut clusters: NumpyArray<i32> = NumpyArray::<i32>::from_filename(&format!("{}{}", dir, "spike_clusters.npy")).unwrap();
    println!("First cluster {:?}", clusters.get(vec![0]));
}

fn main() {
    read_kilosort();
    read_continuous();
}