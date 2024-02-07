import json
import os
from typing import Callable
import numpy as np


def read_numpy(filename: str) -> Callable[[int], np.ndarray]:
    arr = np.load(filename)

    def get(idx: int) -> np.ndarray:
        return arr[idx]

    return get


def read_binary(filename: str) -> Callable[[int], np.ndarray]:
    # Open ephys impl of reading continuous.dat
    # https://github.com/open-ephys/open-ephys-python-tools/blob/aecf82a916ca1f1592eeba8af2070362300a82c2/src/open_ephys/analysis/formats/BinaryRecording.py#L90
    arr = np.memmap(filename, mode='r', dtype='int16')

    def get(idx: int) -> np.ndarray:
        return arr[idx]

    return get


def verify_samples(indexes: list[int], samples: list[float], truth: Callable[[int], np.ndarray]):
    for i, idx in enumerate(indexes):
        tru = truth(idx)
        if tru.shape == ():
            assert samples[i] == tru
        else:
            assert samples[i] == tru[0]


def verify_dataset(filename: str):
    fp = open(filename, "r")
    testdata = json.load(fp)
    print(f"Verifying {filename}")
    for sp_k, spikesortings in testdata["spikesortings"].items():
        print(f"  Verifying spikesorting '{sp_k}'")
        for typ in ["amplitudes", "clusters"]:
            print(f"    Verifying spikesorting {typ}")
            v = spikesortings[typ]
            verify_samples(v["indexes"], v["values"], read_numpy(v["filepath"]))

    print("  Verifying raw")
    raw = testdata["raw_samples"]
    verify_samples(raw["indexes"], raw["values"], read_binary(raw["filepath"]))
    tw = 2


def main():
    # filenames = ["test_dataset.json"]
    filenames = []
    os.chdir("..")  # Go out of the 'tests' dir
    for f in os.listdir():
        if f.startswith("2023") and f.endswith(".json"):
            filenames.append(f)

    for filename in filenames:
        verify_dataset(filename)

    print("Done")
    tw = 2


if __name__ == "__main__":
    main()

