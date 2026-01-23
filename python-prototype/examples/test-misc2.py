from sk import *

x = Sinterval(0, 1)

condition = x > SValue(0.5)

def then_block():
    return SValue("true") 
def else_block():
    return SValue("false")

result = epistemic_if(
    condition, 
    then_block, 
    else_block, 
    policy=IfPolicy.symbolic
)

print(result.resolve())