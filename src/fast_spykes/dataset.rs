pub mod spikesorting;

use crate::fast_spykes::dataset::spikesorting::SpikeSorting;
use crate::fast_spykes::io::binary::BinaryArray;
use crate::fast_spykes::io::FileArray;


pub struct Dataset {
    spike_sortings: Vec<SpikeSorting>,
    pub raw_continuous: Box<dyn FileArray>,
    pub num_samples: usize,
    pub name: String
}


impl Dataset {
    pub fn new(name: String, raw_continuous: impl FileArray + 'static) -> Box<Self> {
        let sampl_len = raw_continuous.len();
        return Box::new(Dataset{
            spike_sortings: vec![],
            raw_continuous: Box::new(raw_continuous),
            num_samples: sampl_len,
            name: name
        });
    }

    pub fn from_continuous(name: String, filename: String, num_channels: usize) -> Box<Self> {
        let continuous = BinaryArray::from_filename(&filename, num_channels);
        // let el = continuous.get(vec![0]);
        // Expect shape to be [samples*channels]
        // println!("First continuous element {:?}", el);
        // println!("Number of channels: {}", continuous.num_channels);
        // println!("Number of samples per channel: {}", continuous.samples_per_channel);

        return Dataset::new(
            name,
            continuous
        );
    }

    pub fn create_dataset(name: String, num_channels: usize, continuous_filepath: String, spike_sorting_paths: Vec<(String, String)>) -> Box<Self> {
        let mut dataset = Dataset::from_continuous(name, continuous_filepath, num_channels);

        for sorting_path in spike_sorting_paths {
            dataset.add_spikesorting(SpikeSorting::from_kilosort_directory(sorting_path.0, sorting_path.1));
        }
        return dataset;
    }

    pub fn add_spikesorting(&mut self, sorting: SpikeSorting) {
        self.spike_sortings.push(sorting);
    }

    pub fn get_spikesortings(&mut self) -> &mut Vec<SpikeSorting> {
        return &mut self.spike_sortings;
    }
}
