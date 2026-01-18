from .kind import SKind
from .value import SValue
from .keywords import Ssymbolic

from enum import Enum, auto

class IfPolicy(Enum):
    merge = auto()  # Run both, return the convex hull (Union)
    none = auto()   # Run neither, return SValue() (Unknown)
    strict = auto()  # Raise an exception (crash)
    symbolic = auto() # Return a symbolic 'mux' node

DEFAULT_POLICY = IfPolicy.merge

def epistemic_if(condition, if_fn, else_fn, policy=IfPolicy.merge):
    # Resolve the condition (it might be a symbolic comparison like 'temp > threshold')
    resolved = condition.resolve() if hasattr(condition, "resolve") else condition

    # 1. CERTAIN CASE
    if resolved.kind == SKind.known:
        return if_fn() if resolved.lower == 1 else else_fn()

    # 2. UNCERTAIN CASE
    if policy == IfPolicy.strict:
        raise ValueError(f"Partial or unknown condition: {resolved}")
    
    if policy == IfPolicy.none:
        return None # Or SValue() if you want an unknown return

    if policy == IfPolicy.merge:
        val_true = if_fn()
        val_false = else_fn()
        return merge_universes(val_true, val_false)
    
    return None

def merge_universes(a, b):
    # Handle side-effect functions (like print) that return None
    if a is None and b is None: return None
    if a is None: return b
    if b is None: return a

    # Get bounds
    try:
        a_min, a_max = a.bounds()
        b_min, b_max = b.bounds()
        return SValue(min(a_min, b_min), max(a_max, b_max))
    except AttributeError:
        # Fallback if the functions return raw numbers instead of SValues
        return SValue(min(a, b), max(a, b))