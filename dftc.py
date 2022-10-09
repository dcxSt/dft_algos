import numpy as np
import matplotlib.pyplot as plt


def decimal_to_binstr(dec:int,bigits:int):
    print(f"DEBUG: dec={dec}\nbigits={bigits}")
    b=["0"]*bigits
    for i in range(1,bigits+1):
        if dec%2==1:
            b[-i]="1"
        dec=dec//2
    return "".join(b)

def binstr_to_decimal(binstr:str):
    dec=0
    l=len(binstr)
    for idx,i in enumerate(binstr):
        if i=="1":
            dec+= 2**(l-idx-1)
    return dec


def bitswitch(x):
    y=np.zeros(x.shape,dtype=complex)
    bigits=np.log2(len(x)) # assumes power of 2
    if bigits==int(bigits):bigits=int(bigits)
    else: raise Error("must be power of 2")
    for i in range(len(x)):
        reverseidx=binstr_to_decimal(decimal_to_binstr(i,bigits)[::-1])
        y[i]=x[reverseidx]
    return y


def dft_for_loop(x:np.ndarray):
    """Takes complex ndarray"""
    n=x.shape[0]
    k=np.log2(n)
    if int(k)==k:k=int(k)
    else:raise Error("Not power of 2")
    x0 = x.copy()
    x_next = bitswitch(x0)
    x0 = x_next.copy()
    print(f"DEBUG: x={x}, bitswitch {x_next}\n")
    print(f"DEBUG: n={n}, k={k}")
    for s in range(1,k+1):
        x0,x_next=x_next,x0 # swap them
        for t in range(2**(k-s)):
            for w in range(2**(s-1)):
                x_next[t*2**s+w] = x0[t*2**s+w] + np.exp(-2j*np.pi*w/(2**s))*x0[t*2**s+2**(s-1)+w]
                x_next[t*2**s+2**(s-1)+w] = x0[t*2**s+w] - np.exp(-2j*np.pi*w/(2**s))*x0[t*2**s+2**(s-1)+w]
    return x_next


### Unit tests
n=128
bigits=8
print(f"n={n}\nbin={decimal_to_binstr(n,bigits)}\ndec={binstr_to_decimal(decimal_to_binstr(n,bigits))}")

# Test bitswitch
x = np.array([0,1,2,3,4,5,6,7])
print(f"DEBUG: x={x}, bitswitch={bitswitch(x)}")


### TEST
x = np.random.normal(0,1,2**4)
print(f"x={x}")
Xcosher = np.fft.fft(x.copy().astype(complex))
Xcust = dft_for_loop(x.copy().astype(complex))

plt.figure()
plt.plot(np.real(Xcosher),label="numpy")
plt.plot(np.real(Xcust),"x",label="custom")
plt.plot(np.imag(Xcosher),label="numpy imag")
plt.plot(np.imag(Xcust),"x",label="custom imag")

plt.legend()

plt.show(block=True)


