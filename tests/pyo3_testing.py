import fast_spykes as fsp


def main():
    prefix = "F:\\QualityMetrics\\datasets\\josh\\"
    fsp.quality_metrics(
        continuous_filepath=f"{prefix}\\raw\\2023-05-12\\continuous\\Neuropix-PXI-100.ProbeA-AP\\continuous.dat",
        spike_sorting_folderpath=f"{prefix}\\curated\\2023-05-12",
        num_channels=384,
        options=["amplitude_cutoff"]
    )


if __name__ == "__main__":
    main()

