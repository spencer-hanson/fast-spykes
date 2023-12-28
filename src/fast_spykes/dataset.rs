use crate::fast_spykes::io::FileArray;

pub struct Dataset {
    pub curated_amplitudes: Box<dyn FileArray>,
    pub curated_clusters: Box<dyn FileArray>,
    pub raw_amplitudes: Box<dyn FileArray>,
    pub raw_clusters: Box<dyn FileArray>,
    pub raw: Box<dyn FileArray>,
    pub raw_samples: usize,
    pub raw_spikes: usize,
    pub curated_spikes: usize
}

impl Dataset {
    pub fn new(curated_amplitudes: impl FileArray + 'static, curated_clusters: impl FileArray + 'static,
               raw_amplitudes: impl FileArray + 'static, raw_clusters: impl FileArray + 'static, raw: impl FileArray + 'static) -> Box<Self> {
        if curated_amplitudes.len() != curated_clusters.len() {
            panic!("Curated amplitudes != Curated clusters!");
        }

        if raw_amplitudes.len() != raw_clusters.len() {
            panic!("Raw amplitudes != Raw clusters!");
        }
        let raw_samples = raw.len();;
        let raw_spikes = raw_amplitudes.len();
        let curated_spikes = curated_amplitudes.len();

        Box::new(Dataset{
            curated_amplitudes: Box::new(curated_amplitudes),
            curated_clusters: Box::new(curated_clusters),
            raw_amplitudes: Box::new(raw_amplitudes),
            raw_clusters: Box::new(raw_clusters),
            raw: Box::new(raw),
            raw_samples,
            raw_spikes,
            curated_spikes
        })
    }
}