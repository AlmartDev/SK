from .value import SValue
from .kind import SKind
from .keywords import *

def greater_than(a: SValue, b: SValue) -> SValue:
    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        from .symbolic import SSymbolic
        return SSymbolic("gt", [a, b])

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_min > b_max:
        return Strue()
    if a_max <= b_min:
        return Sfalse()

    return Spartial()

def less_than(a: SValue, b: SValue) -> SValue:
    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        from .symbolic import SSymbolic
        return SSymbolic("lt", [a, b])
    return greater_than(b, a)

def greater_equal(a: SValue, b: SValue) -> SValue:
    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        from .symbolic import SSymbolic
        return SSymbolic("ge", [a, b])

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_min >= b_max:
        return Strue()
    if a_max < b_min:
        return Sfalse()

    return Spartial()

def less_equal(a: SValue, b: SValue) -> SValue:
    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        from .symbolic import SSymbolic
        return SSymbolic("le", [a, b])
    return greater_equal(b, a)

def equal(a: SValue, b: SValue) -> SValue:
    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        from .symbolic import SSymbolic
        return SSymbolic("eq", [a, b])

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_max < b_min or b_max < a_min:
        return Sfalse()

    if a.kind == SKind.known and b.kind == SKind.known and a_min == b_min:
        return Strue()

    return Spartial()

def not_equal(a: SValue, b: SValue) -> SValue:
    if a.kind in (SKind.unknown, SKind.symbolic) or b.kind in (SKind.unknown, SKind.symbolic):
        from .symbolic import SSymbolic
        return SSymbolic("ne", [a, b])

    a_min, a_max = a.bounds()
    b_min, b_max = b.bounds()

    if a_max < b_min or b_max < a_min:
        return Strue()
    if a.kind == SKind.known and b.kind == SKind.known and a_min == b_min:
        return Sfalse()

    return Spartial()
