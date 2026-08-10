import sys

from .xmpkit import *
from .xmpkit import ns

sys.modules["xmpkit.ns"] = ns

__doc__ = xmpkit.__doc__

if hasattr(xmpkit, "__all__"):
    __all__ = xmpkit.__all__
