from sk import *

def test_interval_dependency():
    x = Sinterval(1, 2)
    
    expr = x - x
    result = expr.resolve()
    
    print(f"Dependency Test: x={x}")
    print(f"Expression: x - x -> {expr}")
    print(f"Resolved: {result}")
    
    assert result.kind == Sknown(0).kind and result.lower == 0

def test_lazy_resolution():
    a = SValue(10)
    b = Sknown(5)
    
    expr = a + b
    first_resolve = expr.resolve()
    print(f"Lazy Test Initial: {a} + {b} = {first_resolve}")
    
    a.setKnown(20)
    second_resolve = expr.resolve()
    print(f"Lazy Test Updated: {a} + {b} = {second_resolve}")
    
    assert first_resolve.lower == 15
    assert second_resolve.lower == 25

def test_non_numeric_intervals():
    status = SValue("low", "high")
    print(f"Non-numeric Interval: {status}")
    
    try:
        invalid = SValue("high", "low")
    except ValueError as e:
        print(f"Caught expected error: {e}")

    assert status.lower == "low"
    assert status.higher == "high"

if __name__ == "__main__":
    test_interval_dependency()
    print("---")
    test_lazy_resolution()
    print("---")
    test_non_numeric_intervals()
