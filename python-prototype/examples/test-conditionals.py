from sk import *

print("=== BOOLEAN TESTS ===\n")

# Basic known values
a = SValue(3)
b = SValue(5)
c = SValue(3, 5)
u = SValue()  # unknown

values = [a, b, c, u]
names = ["a=3", "b=5", "c=[3..5]", "u=unknown"]

comparisons = [
    (">", lambda x, y: x > y),
    ("<", lambda x, y: x < y),
    (">=", lambda x, y: x >= y),
    ("<=", lambda x, y: x <= y),
    ("==", lambda x, y: x == y),
    ("!=", lambda x, y: x != y),
]

for i, x in enumerate(values):
    for j, y in enumerate(values):
        print(f"Comparisons between {names[i]} and {names[j]}:")
        for op_name, op_func in comparisons:
            result = op_func(x, y)
            print(f"  {names[i]} {op_name} {names[j]} => {result}")
        print()
        
print("=== BOOLEAN KEYWORDS ===")
print("Strue()   =", Strue())      # 1
print("Sfalse()  =", Sfalse())     # 0
print("Spartial()=", Spartial())   # [0..1]
