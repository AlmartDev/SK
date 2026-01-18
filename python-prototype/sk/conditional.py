from .value import SValue
from .kind import SKind
from .keywords import Ssymbolic

def epistemic_if(condition, if_fn, else_fn, policy="run_both"):
    resolved = condition.resolve() if hasattr(condition, "resolve") else condition

    if resolved.kind == SKind.known:
        if bool(resolved.lower):
            return if_fn()
        else:
            return else_fn()
    
    elif resolved.kind == SKind.interval:
        if policy == "run_both":
            if_fn()
            else_fn()
        elif policy == "strict":
            return None
        elif policy == "fail_on_partial":
            raise ValueError(f"Partial condition: {condition}")
    
    elif resolved.kind == SKind.unknown:
        if policy in ("run_both", "strict"):
            return Ssymbolic("epistemic_if", [condition, if_fn, else_fn])
        elif policy == "fail_on_partial":
            raise ValueError(f"Unknown condition: {condition}")
