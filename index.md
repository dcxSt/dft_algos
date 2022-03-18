---
layout: default
---

# Fourier Transforms (FT)
Broadly, the fourier transform is a way of transforming some vectors or functions into other vectors or functions. The fourier transform has a broad range of applications and is central in many mathematical fields, so depending on who you ask, the fourier transform is likely to mean different things.

If you ask a **mathematician** what a fourier transform is they are likely to reply with some gibberish like *"the fourier transform is a **linear operator** that acts on **Hilbert spaces** and **finite groups** with many applications that satisfies a variety of nice properties (such as invertibility) under various conditions."* 

A **physicist** will say *"the fourier transfrom lies at the **core of many fundamental fields** in physics such as quantum mechanics and fluid dynamics. What's more, the fourier transform is a essential to the math behind statistical mechanics, which is the most important and fundamental field in all of physics."* 

A **signal processing engineer**, i.e. an engineer in any of these fields: music; computer vision; stock market prediction; some medical fields such as EEG/EMG/ECG analysis; power grid/energy distribution—will tell you that *the discrete fourier transform is **the most fundamental tool** that they use every day.*

# Discrete Fourier Transform (DFT)
The Discrete Fourier Transform. It is often used synonymously with the Fast Fourier Transform (FFT)

# Algorithms for computing the Discrete Fourier Transform

## Cooley Tukey Algorithms
Cooley Tukey Algorithms for the FFT are the most important class of fourier transform. 

### Radix-2 DIT 
*Terminology: DIT stands for 'Decimation In Time'.*

The Radix-2 DIT algorithm is the simplest Cooley-Tukey algorithm to understand. It assumes that the input discrete signal has exactly \\(2^n\\) componants. 

#### The algorithm

#### Python implementation (drop-down?)
*code snippet here*

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



