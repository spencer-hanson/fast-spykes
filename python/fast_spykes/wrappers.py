from typing import Any, Union, Optional

from .typing import QualityMetricOutputType
from .consts import ALL_QUALITY_METRICS_DEFAULTS


def _check_qm_names(qm_names: list[str]):
    for name in qm_names:
        if name not in ALL_QUALITY_METRICS_DEFAULTS:
            raise ValueError(f"Quality Metric name \"{name}\" not recognized! Available metrics '{list(ALL_QUALITY_METRICS_DEFAULTS.keys())}'")


def quality_metrics(
    continuous_filepath: str, spike_sorting_folderpath: str, num_channels: int = 384,
    options: Optional[Union[list[str], dict[str, dict[str, Any]]]] = None
) -> QualityMetricOutputType:
    chosen_options: dict[str, dict[str, Any]] = None

    if options is None:
        chosen_options = ALL_QUALITY_METRICS_DEFAULTS
    elif isinstance(options, list):
        _check_qm_names(options)
        chosen_options = {o: ALL_QUALITY_METRICS_DEFAULTS[o] for o in options}
    elif isinstance(options, dict):
        _check_qm_names(list(options.keys()))
        for k, v in options.items():
            if not isinstance(v, dict):
                raise ValueError(f"Invalid entry for '{k}' Parameters must be a dict! Got \"{v}\" instead!")
        chosen_options = options
        # TODO Verify and create a QM runtime with the stuff in .config
    else:
        raise ValueError(f"Unknown type given for options '{type(options)}' Acceptable inputs [\"qm_name\", ..] or {{\"qm_name\": {{ params }}, .. }} or None")

    print("Info:\n cdat:'{}'\n sp:'{}'\n chan:'{}'".format(continuous_filepath, spike_sorting_folderpath, num_channels))

