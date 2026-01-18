from sk import *

# A simple program for a self-driving car written in SK-prototype

velocity = Sunknown() # in km/h

refined_velocity = (velocity * 10) / 36    # Symbolic variable, velocity in m/s
print(refined_velocity)

reader_online = Strue()

def reader_true():
    velocity.setInterval(60 , 70)

def reader_false():
    print("PANIC! cant retrieve speed")

epistemic_if(reader_online == Strue(), reader_true, reader_false)

print(refined_velocity.resolve())