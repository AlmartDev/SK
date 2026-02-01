#
# --- IMPORTANT!: This test file is not SK code but a python prototype of the SK language. ---
# This means that this file and the interpreter's 'prototype' are deprecated.
# 
# The python-protoype was kept to have a base in order to develop the Rust interpreter,
# so it will not have any new updates, alghtough it won't be removed.
#
# Find the actual SK test files inside interpreter/examples/
# https://github.com/AlmartDev/SK/tree/main/interpreter/examples
#

from sk import *

x = Sunknown()          # Unknown variable
y = Sknown(2)           # Known
z = Sinterval(-3, 42)   # Partialy known, interval

print(x.kind, y.kind, z.kind)

print(z / y)
print(y ** z)
print(y ** x)
print(y / y)