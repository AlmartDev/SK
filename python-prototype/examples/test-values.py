

from sk import *

x = Sunknown()          # Unknown variable
y = Sknown(2)           # Known
z = Sinterval(-3, 42)   # Partialy known, interval

print(x.kind, y.kind, z.kind)

print(z / y)
print(y ** z)
print(y ** x)
print(y / y)