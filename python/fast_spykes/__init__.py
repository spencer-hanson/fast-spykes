from .config import KNOWN_QUALITY_METRICS

from .wrappers import quality_metrics
from .fast_spykes import *


__doc__ = fast_spykes.__doc__
if hasattr(fast_spykes, "__all__"):
    __all__ = fast_spykes.__all__