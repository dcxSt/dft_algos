---
layout: default
---

# Fourier Transforms (FT)
Broadly, the fourier transform is a way of transforming some vectors or functions into other vectors or functions. The fourier transform has a broad range of applications and is central in many mathematical fields, so depending on who you ask, the fourier transform is likely to mean different things.

If you ask a **mathematician** what a fourier transform is they are likely to reply with some gibberish like *"the fourier transform is a **linear operator** that acts on **Hilbert spaces** and **groups** that satisfies a variety of nice properties (such as invertibility) under various conditions."* 

A **physicist** will say *"the fourier transfrom lies at the **core of many fundamental fields** in physics such as quantum mechanics and fluid dynamics. What's more, the fourier transform is a essential to the math behind statistical mechanics, which is the most important and fundamental field in all of physics."* 

A **signal processing engineer**, i.e. an engineer in any of these fields: music; computer vision; stock market prediction; some medical fields such as EEG/EMG/ECG analysis; power grid/energy distribution; and more—will tell you that *the discrete fourier transform is **the most fundamental tool** used in digital sigial processing.*

In this repository we explore some algorithms for computing the discrete fourier transform of vectors. 

# Discrete Fourier Transform (DFT)
The DFT of a vector is a linear transformation which can be represented by a matrix. If the vector in question is \\(n\\) dimensional, and it's either real or complex, then the DFT matrix is a linear operator \\(\mathbb{C}^n\to\mathbb{C}^n\\) (or \\(\mathbb{R}^n\to\mathbb{C}^n\\)). It looks like this

$$
\begin{bmatrix}
  1 & 1            & 1            & 1            & \cdots & 1 \\ 
  1 & e^{-2\pi i/n} & e^{-4\pi i/n} & e^{-6\pi i/n} & \cdots & e^{-(n-1)2\pi i/n} \\ 
  1 & e^{-4\pi i/n} & e^{-8\pi i/n} & e^{-12\pi i/n} & \cdots & e^{-2(n - 1)2\pi i/n} \\ 
  1 & e^{-6\pi i/n} & e^{-12\pi i/n} & e^{-18\pi i/n} & \cdots & e^{-3(n - 1)2\pi i/n} \\
  \vdots & \vdots & \vdots & \vdots & \vdots & \vdots \\
  1 & e^{-(n-1)2\pi i/n} & e^{-2(n-1)2\pi i/n} & e^{-3(n-1)2\pi i/n} & \cdots & e^{-(n-1)^2 2\pi i/n}
\end{bmatrix}
$$

The matrix can be written more succinctly like so 

\\[a_{j,k} = \exp(-j\cdot k\cdot 2\pi i/n)\qquad\text{DFT} = (a_{j,k})\\]

It is symmetric, orthogonal, and unitary (up to a factor of \\(1/\sqrt n\\)) i.e. it's inverse is just \\(1/n\\) times it's hermitian conjugate.

# Fast Fourier Transform FFT
The FFT is a family of algorithms for computing the DFT of a vector. It is by far the most popular algorithm—so much so that many people use FFT and DFT interchangably.

## Cooley-Tukey Algorithms
Cooley Tukey Algorithms for the FFT are the most important class of fast fourier transform algorithms. 

### Radix-2 DIT 
*Note: DIT stands for 'Decimation In Time'.*

The Radix-2 algorithm is the simplest Cooley-Tukey algorithm to understand. It assumes that the input vector (/signal) has exactly \\(2^n\\) dimensions. Then uses recursion to break each signal in half. 

![DIT FFT butterfly diagram](https://raw.githubusercontent.com/dcxSt/dft_algos/gh-pages/assets/img/DIT-FFT-butterfly.png)

#### Python implementation 

A recursive implementation

```python
import numpy as np          # numpy library
PI = np.py                  # the constant 3.14...

def fft(arr):
  n = arr.shape[0]          # assume arr len is a power of 2
  if n == 1:
    return arr[0]
  else:
    even = fft(arr[::2])    # FFT of even indices
    odd = fft(arr[1::2])    # FFT of odd indices
    phase = np.exp(2*PI*1.j/n * np.arange(n/2))
    odd *= phase            # pointwise multiply odds by phase 
    return np.concatenate([even + odd , even - odd])
```

A for-loop implementation
(what advantages does this offer over recursive? is it faster?, more readable?)

#### Rust implementation (drop-down? how to conceal this to make article more concise?, look at raffi's website)
*code snippet here*

#### Algorithmi complexity

*Theoretical time complexity*

*Theoretical memory complexity*

*Graph of Benchmarks*

## *A more general cooley tukey*

*What if the length of the array is prime?*


## An Algorithm for All Finite Groups
[Link to paper (see page 15)](https://math.uchicago.edu/~may/REU2018/REUPapers/Dandavati.pdf)

*(add: other implementations)*

# Applications of the DFT
The DFT is perhaps the most important and basic tool in signal processing. 


# Other related transforms

## Wavelet Transforms



