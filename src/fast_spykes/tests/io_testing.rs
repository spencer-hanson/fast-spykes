use std::collections::HashMap;
use std::fs;
use std::io::Write;
use rand::Rng;
use serde::Serialize;
use crate::fast_spykes::dataset::Dataset;
use crate::fast_spykes::io::{FileArray};

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

fn get_arr_by_idxs(mut filearr: &mut Box<dyn FileArray>, idxs: &Vec<usize>) -> Vec<f64> {
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
struct SampleData {
    values: Vec<f64>,
    indexes: Vec<usize>
}

#[derive(Serialize)]
struct DatasetSamples<'a> {
    raw_samples: SampleData,
    // {"spikesorting_name": {"amplitudes": .., "clusters": ... }, ..}
    spikesortings: HashMap<&'a str, HashMap<&'a str, SampleData>>
}

pub fn dataset_to_testfile(mut dataset: Box<Dataset>) {
    let num_idxs = 1000;
    let filename = format!("{}.json", dataset.name);

    let mut spikesortings: HashMap<&str, HashMap<&str, SampleData>> = HashMap::new();

    let sample_idxs = get_rand_idxs(num_idxs, dataset.num_samples);
    let raw_samples = get_arr_by_idxs(&mut dataset.raw_continuous, &sample_idxs);

    for spike_sorting in dataset.get_spikesortings() {
        let amp_indexes = get_rand_idxs(num_idxs, spike_sorting.num_spikes);
        let clust_indexes = get_rand_idxs(num_idxs, spike_sorting.num_spikes);

        let amp_data = get_arr_by_idxs(&mut spike_sorting.amplitudes, &amp_indexes);
        let clust_data = get_arr_by_idxs(&mut spike_sorting.cluster_labels, &clust_indexes);

        let mut m: HashMap<&str, SampleData> = HashMap::new();
        m.insert("amplitudes", SampleData{values: amp_data, indexes: amp_indexes});
        m.insert("clusters", SampleData{values: clust_data, indexes: clust_indexes});
        spikesortings.insert(&spike_sorting.name, m);
    }



    let samples = DatasetSamples {
        raw_samples: SampleData{
            values: raw_samples,
            indexes: sample_idxs
        },
        spikesortings: spikesortings
    };

    let s = serde_json::to_string(&samples).unwrap();
    fs::write(filename, s).unwrap();

    println!("Wrote sampling of dataset '{}' to file", dataset.name);
}
