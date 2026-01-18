from .value import SValue
from .kind import SKind
from .keywords import *

def greater_than(a: SValue, b: SValue) -> SValue:
    # Unknown dominates
    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()

    # Known / interval logic
    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_min > b_max:
        return Strue()

    if a_max <= b_min:
        return Sfalse()

    return Spartial()

def less_than(a: SValue, b: SValue) -> SValue:
    return greater_than(b, a)

def greater_equal(a: SValue, b: SValue) -> SValue:
    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_min >= b_max:
        return Strue()

    if a_max < b_min:
        return Sfalse()

    return Spartial()

def less_equal(a: SValue, b: SValue) -> SValue:
    return greater_equal(b, a)

def equal(a: SValue, b: SValue) -> SValue:
    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    # No overlap
    if a_max < b_min or b_max < a_min:
        return Sfalse()

    # Both known and equal
    if a.kind is SKind.known and b.kind is SKind.known and a_min == b_min:
        return Strue()

    return Spartial()

def not_equal(a: SValue, b: SValue) -> SValue:
    if a.kind is SKind.unknown or b.kind is SKind.unknown:
        return SValue()

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_max < b_min or b_max < a_min:
        return Strue()

    if a.kind is SKind.known and b.kind is SKind.known and a_min == b_min:
        return Sfalse()

    return Spartial()