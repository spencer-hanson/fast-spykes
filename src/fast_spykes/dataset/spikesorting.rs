use crate::fast_spykes::io::FileArray;
use crate::fast_spykes::io::numpy::NumpyArray;

pub struct SpikeSorting {
    pub amplitudes: Box<dyn FileArray>,
    pub cluster_labels: Box<dyn FileArray>,
    pub num_spikes: usize,
    pub name: String,
}


impl SpikeSorting {
    pub fn new(name: String, amplitudes: impl FileArray + 'static, cluster_labels: impl FileArray + 'static) -> Self {
        if amplitudes.len() != cluster_labels.len() {
            panic!("len Amplitudes != len Clusters! for {}", name);
        }
        let amp_len = amplitudes.len();

        return SpikeSorting {
            amplitudes: Box::new(amplitudes),
            cluster_labels: Box::new(cluster_labels),
            num_spikes: amp_len,
            name: name
        };
    }

    pub fn from_kilosort_directory(name: String, filename: String) -> Self {
        let kilo = SpikeSorting::read_kilosort(&filename);

        return SpikeSorting::new(
            name,
            kilo.0,
            kilo.1
        );
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
        fn format_filename(base_path: &str, filename: &str) -> String {
            let sep = std::path::MAIN_SEPARATOR_STR;
            if base_path.len() > sep.len() {
                if &base_path[base_path.len()-sep.len()..] != sep {
                    return format!("{}{}{}", base_path, sep, filename);
                } else {
                    return format!("{}{}", base_path, filename);
                }
            } else {
                panic!("Given base path is not larger than the OS file separator!");
            }
            todo!();
        }
        // Expect shape to be [spikes, 0]
        let amp_fn = &format_filename(filename, "amplitudes.npy");
        let mut amplitudes = NumpyArray::<f64, f64>::from_filename(amp_fn).unwrap();
        // println!("Amplitude first {:?}", amplitudes.get(vec![0, 0]));
        // println!("Amplitude shape {:?}", amplitudes.shape());

        // Expect shape to be [spikes]
        let clust_fn = &format_filename(filename, "spike_clusters.npy");
        let mut clusters = NumpyArray::<u32, i32>::from_filename(clust_fn).unwrap();

        // println!("Cluster first {:?}", clusters.get(vec![0]));
        // println!("Cluster shape {:?}", clusters.shape());

        return (amplitudes, clusters);
    }

}
