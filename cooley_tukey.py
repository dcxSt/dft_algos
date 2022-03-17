import numpy as np
from numpy.fft import fft

# Steve and Rosie's Radix-2 Decimation In Time (DIT) algorithm
def fft_radix2(arr):
    # assume len(arr) is a power of 2
    n = arr.shape[0]
    if n == 1:
        return arr[0]
    else:
        even = fft_radix2(arr[::2]) # this is the FFT of even indices
        odd = fft_radix2(arr[1::2]) * np.exp(2*np.pi*1.j / n * np.arange(n/2)) # FFT of odd indices
        return np.concatenate([even + odd, even - odd])

# Radix-2 with DIT with padding
def fft_radix2_pad(arr):
    log2len = np.log2(len(arr)) # first check if len pow 2 
    zeros = np.zeros(int(2**np.ceil(log2len)) - len(arr))
    padded_arr = np.concatenate((arr,zeros)) # pad with zeros
    return fft_radix2(padded_arr)


# TESTS
if __name__ == "__main__":
    x1 = np.array([0,1,2,3,4,5,6,7])
    x2 = np.array([1,2,3,4,5])
    x2_pad = np.array([1,2,3,4,5,0,0,0])

    assert fft(x1) == fft_radix2(x1) 
    assert fft(x1) == fft_radix2_pad(x1) 
    assert fft(x2_pad) == fft_radix2(x2_pad) 
    assert fft(x2_pad) == fft_radix2_pad(x2) 
    



