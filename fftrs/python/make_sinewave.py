# make a 32-bit sine-wave
import numpy as np

sin = np.sin(np.linspace(0,2*np.pi,(1<<14)+1)[:-1])
sin *= (1<<31) - 1;
sin += 0.5; # round appropriately
sin = np.asarray(sin,dtype="int") # type casting

print("Sneak peak\n", sin[(1<<14)//4-5:(1<<14)//4+5])

with open("sine_32deep_14samp.txt",'w') as f:
    for idx,i in enumerate(sin):
        f.write(f"{i}, ")
        if idx %10 == 0:
            f.write("\n")
print("Done")
