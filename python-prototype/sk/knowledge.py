from .value import SValue
from .kind import SKind

def certain(cond: SValue) -> SValue:
    if cond.kind == SKind.known and cond.lower == 1:
        return SValue(1)
    
    if cond.kind == SKind.interval and cond.lower >= 1:
        return SValue(1)
    
    return SValue(0)

def impossible(cond: SValue) -> SValue:
    if cond.kind == SKind.known and cond.lower == 0:
        return SValue(1)

    if cond.kind == SKind.interval and cond.higher <= 0:
        return SValue(1)
        
    return SValue(0)

def possible(cond: SValue) -> SValue:
    imp = impossible(cond)
    return SValue(0) if imp.lower == 1 else SValue(1)

def known(cond: SValue) -> SValue:
    if cond.kind == SKind.known:
        return SValue(1)
    if cond.kind == SKind.interval and cond.lower == cond.higher:
        return SValue(1)
    return SValue(0)