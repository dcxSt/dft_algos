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
Fourier transforms are one of the workhorses of physics, both computational and otherwise. On a computer, we in generaly carry out *discrete Fourier transforms* or DFTs, where we sum (rather than integrate) over a discrete set of function points, and evaluate the DFT at a discrete set of \\(k\\)'s. Because of this, the DFTs can differ in some subtle ways from analytic Fourier transforms. 

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

The Radix-2 algorithm is the simplest Cooley-Tukey algorithm. It assumes that the input vector (or 'signal') has exactly \\(2^r\\) dimensions (\\(r\\) is a positive integer). The algorithm shuffles the vector's componants around, then splits the vector into two halves and reccursively applies it's self to each half of the vector, and then re-combines the output of each. 

Let \\(x\\) be a \\(n = 2^r\\) dimensional vector with real or complex entries

\\[x = \big[ x[0] , x[1] , x[2] , ... , x[n-1] \big]\\]

And let \\(X\\) be it's fourier transform. 

\\[DFT\\{x\\}[k] = X[k] = \sum_{t=0}^{n-1} x[t] e^{-t \cdot k\cdot 2\pi i / n}\\]

The key insight is to realize that this sum is itsself the sum of two smaller sums that are each DFTs of subvectors of \\(x\\). 

$$
\sum_{t=0}^{n-1} x[t] e^{-t \cdot k\cdot 2\pi i / n} 
= \left( \sum_{t_0=0}^{n/2-1} x[2t_0] e^{-t_0\cdot k\cdot 2\pi i/(n/2)} \right) 
+ e^{- k\cdot 2\pi i/n} \left( \sum_{t_1=0}^{n/2-1} x[2t_1+1] e^{-t_1\cdot k\cdot 2\pi i/(n/2)} \right)  
$$

$$
DFT\big\{ x[0] , x[1] , ... , x[n-1] \big\} 
= DFT\big\{ x[0] , x[2] , .. ,  x[n-2] \big\}  \\
\hspace{15.0em}
+ e^{-k\cdot 2\pi i/n} DFT\big\{ x[1] , x[3] , ... , x[n-1] \big\}
$$

Now you can see that the right hand side is just the sum of two smaller fourier transforms. To solve those two, you reccursively apply the same trick until you get to two dimensional vectors where the Discrete Fourier Transform reduces to a trivial sum and difference.

$$
DFT\{x[0] , x[1]\} = \big[ x[0] + x[1] , x[0] - x[1] \big]
$$



*Below: Butterfly diagram of the Culey Tukey FFT.*

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

#### C implementation

In place FFT. 

```c


```

#### Rust implementation (drop-down? how to conceal this to make article more concise?, look at raffi's website)

*code snippet here*

#### Assembly implementation
*code snippet here*

#### Algorithmic complexity
O(n logn)

*Theoretical time complexity*

*Theoretical memory complexity*

*Graph of Benchmarks*

## *A more general cooley tukey*

*What if the length of the array is prime?*


## An Algorithm for All Finite Groups
[Link to paper (see page 15)](https://math.uchicago.edu/~may/REU2018/REUPapers/Dandavati.pdf)

Why would anyone ever want to do this?

*(add: other implementations)*

# History of the DFT

In the early 1800s Joesph Fourier was galavanting around the French empire as a military official, then Napoleon put him in charge of Grenoble. While govonour, he conducted some mathematical investigations into heat flow on the side, which lead to a very neat insight that you can often greatly simplify your life greatly by performing a change of basis of your function space that came to be known as a Fourier transform. Thus Fourier Theory was bourn. 



# Applications of the DFT
The DFT is perhaps the most important and basic tool in signal processing. 


# Other related transforms

## Wavelet Transforms


***As you can see, this note is far from complete, if you'd like to contribute, contact me or submit a PR.***




