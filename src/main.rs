mod fast_spykes;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use rand::Rng;
use crate::fast_spykes::dataset::Dataset;
use crate::fast_spykes::io::numpy::{NumpyArray};
use crate::fast_spykes::io::binary::{BinaryArray};
use crate::fast_spykes::io::FileArray;
use crate::fast_spykes::tests::io_testing::dataset_to_testfile;

fn read_i16(file: &mut File) -> i16 {
    let mut buf: [u8 ; 2] = [0; 2];
    file.read_exact(&mut buf).ok();
    let v: i16 = i16::from_le_bytes(buf);
    return v;
    // println!("Raw: {v}");
    // println!("Adjusted: {}", v as f64 * 0.1949999928474426);
}


fn read_continuous(continuous_filename: &str, num_channels: u64) -> BinaryArray {
    let mut continuous = BinaryArray::from_filename(continuous_filename, num_channels);
    // let el = continuous.get(vec![0]);
    // Expect shape to be [samples*channels]
    // println!("First continuous element {:?}", el);
    // println!("Number of channels: {}", continuous.num_channels);
    // println!("Number of samples per channel: {}", continuous.samples_per_channel);
    return continuous;
}

fn read_kilosort(filename: &str) -> (NumpyArray::<f64, f64>, NumpyArray<u32, i32>) {
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

    // Expect shape to be [spikes, 0]
    let amp_fn = &format!("{}{}", filename, "amplitudes.npy");
    let mut amplitudes = NumpyArray::<f64, f64>::from_filename(amp_fn).unwrap();
    // println!("Amplitude first {:?}", amplitudes.get(vec![0, 0]));
    // println!("Amplitude shape {:?}", amplitudes.shape());

    // Expect shape to be [spikes]
    let clust_fn = &format!("{}{}", filename, "spike_clusters.npy");
    let mut clusters = NumpyArray::<u32, i32>::from_filename(clust_fn).unwrap();

    // println!("Cluster first {:?}", clusters.get(vec![0]));
    // println!("Cluster shape {:?}", clusters.shape());

    return (amplitudes, clusters);
}

fn get_datasets(dates: Vec<&str>) -> Vec<Box<Dataset>> {

    const RAW_FOLDER_PREFIX: &str = "D:\\QualityMetrics\\datasets\\josh\\raw";

    const FOLDER_PREFIX: &str = "D:\\QualityMetrics\\datasets\\josh\\curated";
    const RAW_FOLDER_SUFFIX: &str = "continuous\\Neuropix-PXI-100.ProbeA-AP";
    const FOLDER_SUFFIX: &str = "";



    let num_channels: u64 = 384; // TODO Find this out?

    let mut datasets = vec![];

    for cur_date in dates.iter() {
        let filename1 = format!("{}\\{}\\{}\\", FOLDER_PREFIX, *cur_date, FOLDER_SUFFIX);
        let filename2 = format!("{}\\{}\\{}\\", RAW_FOLDER_PREFIX, *cur_date, RAW_FOLDER_SUFFIX);

        // println!("---\nReading from date {}\n---", cur_date);

        println!("Getting Curated");
        let curated_kilo = read_kilosort(&filename1);
        // let curated_cont = read_continuous(&format!("{FOLDER_PREFIX}\\{cur_date}\\{FOLDER_SUFFIX}\\continuous.dat"), num_channels);

        println!("Getting Raw Kilosorted");
        let raw_kilo = read_kilosort(&filename2);

        println!("Getting Raw Continuous");
        let cont = read_continuous(&format!("{RAW_FOLDER_PREFIX}\\{cur_date}\\{RAW_FOLDER_SUFFIX}\\continuous.dat"), num_channels);

        datasets.push(Dataset::new(curated_kilo.0, curated_kilo.1, raw_kilo.0, raw_kilo.1, cont));
    }

    return datasets;

}

fn main() {
    // let ALL_DATES = vec![
    //     "2023-04-11",
    //     "2023-04-12",
    //     "2023-04-13",
    //     "2023-04-14",
    //     "2023-04-17",
    //     "2023-04-19",
    //     "2023-04-21",
    //     "2023-04-24",
    //     "2023-04-25",
    //     "2023-05-12",
    //     "2023-05-15",
    //     "2023-05-16",
    //     "2023-05-17",
    //     "2023-05-19",
    //     "2023-05-23",
    //     "2023-07-24",
    //     "2023-07-26",
    //     "2023-07-28",
    // ];

    // let datasets = get_datasets(vec!["2023-04-11"]);
    // println!("Curated Amp shape {:?}", datasets[0].curated_amplitudes.shape());
    // println!("Curated Clust shape {:?}", datasets[0].curated_clusters.shape());
    //
    // println!("Raw Amp shape {:?}", datasets[0].raw_amplitudes.shape());
    // println!("Raw Clust shape {:?}", datasets[0].raw_clusters.shape());
    //
    // println!("Continuous shape {:?}", datasets[0].raw.shape());

    let mut dataset = get_datasets(vec!["2023-04-11"]);
    dataset_to_testfile(dataset.pop().unwrap(), "testfile.json");
    println!("Do a dataset");

}