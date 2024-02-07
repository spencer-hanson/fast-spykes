mod fast_spykes;

use crate::fast_spykes::dataset::Dataset;
use crate::fast_spykes::tests::io_testing::dataset_to_testfile;


fn main() {

    // let folder = String::from("E:\\NeuroPixelsTest\\continuous\\Neuropix-PXI-104.ProbeA-AP");
    // let dataset = Dataset::create_dataset(
    //     String::from("test_dataset"),
    //     384,
    //     format!("{}\\continuous.dat", folder.clone()),
    //     vec![(String::from("main"), folder.clone())]
    // );
    // dataset_to_testfile(dataset);
    // todo!();

    let data_path: &str = "E:\\QualityMetrics\\datasets";
    let dataset = "josh";
    let raw_spikesorting = ("raw", format!("{data_path}\\{dataset}\\raw"), "continuous\\Neuropix-PXI-100.ProbeA-AP");

    let spikesorting_names = vec![
        // (name, prefix, suffix) -> prefix/<date>/suffix/<kilosort>
        raw_spikesorting.clone(),
        ("curated", format!("{data_path}\\{dataset}\\curated"), "")
    ];

    let num_channels = 384; // TODO Find this out?

    let all_files = vec![
        "2023-04-11",
        "2023-04-12",
        "2023-04-13",
        "2023-04-14",
        "2023-04-17",
        "2023-04-19",
        "2023-04-21",
        "2023-04-24",
        "2023-04-25",
        "2023-05-12",
        "2023-05-15",
        "2023-05-16",
        "2023-05-17",
        "2023-05-19",
        "2023-05-23",
        "2023-07-24",
        "2023-07-26",
        // "2023-07-28", // Doesn't exist on Bard Brive
    ];

    let mut datasets = vec![];


    for file in all_files {
        println!("Reading in '{}'", file);

        let mut spikesorting_paths = vec![];
        for spikesorting_name in &spikesorting_names {
            spikesorting_paths.push((
                String::from(spikesorting_name.clone().0),
                format!(
                    "{prefix}\\{file}\\{suffix}\\",
                    prefix = spikesorting_name.clone().1,
                    suffix = spikesorting_name.clone().2
                )
            ));
        }
        let continuous = format!(
            "{prefix}\\{file}\\{suffix}\\continuous.dat",
            prefix = raw_spikesorting.clone().1,
            suffix = raw_spikesorting.clone().2
        );

        datasets.push(Dataset::create_dataset(
            String::from(file),
            num_channels,
            continuous,
            spikesorting_paths
        ));
    }

    println!("Todo processing here");

    // for d in datasets {
    //     println!("Creating testdata for dataset {}", d.name);
    //     dataset_to_testfile(d);
    // }
    // println!("Done processing all datasets");
}
