from typing import Any, Union
from ..typing import QualityMetricParamType

KNOWN_QUALITY_METRICS = {"qm_name": "QualityMetricSchema"}  # TODO populate with schema files found in config directory

#TODO defaults?
# From https://spikeinterface.readthedocs.io/en/latest/modules/qualitymetrics.html

# Default parameters for all Quality Metrics
ALL_QUALITY_METRICS_DEFAULTS = {
    # Amplitude cutoff
    "amplitude_cutoff": {},
    # Amplitude CV
    "amplitude_cv_median": {},
    "amplitude_cv_range": {},
    # Amplitude median
    "amplitude_median": {},
    # D-prime
    "d_prime": {},
    # Drift metrics
    "drift_ptp": {},
    "drift_std": {},
    "drift_mad": {},
    # Firing range
    "firing_range": {},
    # Firing rate
    "firing_rate": {},
    # Inter-spike-interval (ISI) violations
    "isi_violation": {},
    "rp_violation": {},
    # Isolation distance
    "isolation_distance"
    # L-ratio
    "l_ratio"
    # Nearest Neighbor Metrics
    "nn_hit_rate": {},
    "nn_miss_rate": {},
    "nn_isolation": {},
    "nn_noise_overlap": {},
    # Noise cutoff (not currently implemented)
    # Presence ratio
    "presence_ratio": {},
    # Standard Deviation (SD) ratio
    "sd_ratio": {},
    # Silhouette score
    "silhouette": {},
    "silhouette_full": {},
    # Sliding refractory period violations
    "sliding_rp_violations": {},
    # Signal-to-noise ratio
    "snr": {},
    # Synchrony Metrics
    "synchrony": {}
}




class _QualityMetricRuntime(object):
    def __init__(self, iid: str, params: QualityMetricParamType):
        self.idd = iid
        self.params = params


class QualityMetricSchema(object):
    def __init__(self, iid: str, params: QualityMetricParamType, defaults: QualityMetricParamType):
        self.iid = iid
        self.params = params
        self.defaults = defaults

    def _check_type(self, type_schema, value) -> (bool, str):
        typs = {
            "int": int,
            "string": str,
            "bool": bool,
            "dict": dict
        }

        # Check basic types
        for typ_name, typ in typs.items():
            if type_schema == typ_name:
                if not isinstance(value, typ):
                    return False, f"Expected '{str(typ_name)}', got \"{value}\" instead"
                else:
                    return True, ""

        # Check list type
        if isinstance(type_schema, list):
            if not isinstance(value, list):
                return False, f"Expected list, got \"{value}\" instead"
            else:
                typ = typs[type_schema[0]]  # entry like ["type"] in basic typ dict
                for v in value:
                    if not isinstance(v, typ):
                        return False, f"List is inhomogeneous, expected all values to be '{str(typ)}' found \"{v}\""
                return True, ""
        else:  # Error in config schema
            raise ValueError(f"Invalid type schema '{type_schema}'! Check config files")

    def verify_params(self, given_params: dict) -> (bool, str):
        intro = f"Error with QM config for '{self.iid}'"

        for k, v in given_params.items():
            if not isinstance(k, str):
                return False, f"{intro} Param keys must be strings, got '{k}'"
            if k not in self.params:
                return False, f"{intro} Param not recognized '{k}' Available \"{list(self.params.keys())}\""
            passed, err = self._check_type(self.params[k], v)
            if not passed:
                return False, f"{intro} Error with param {k}: {err}"

        return True

    def create_runtime(self, given_params: dict) -> _QualityMetricRuntime:
        passes, err = self.verify_params(given_params)
        if not passes:
            raise ValueError(err)

        return _QualityMetricRuntime(self.iid, given_params)
