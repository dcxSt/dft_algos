todo


- [x] Figure out how if there is a native complex integer datatype in rust


Building the FFT
- [x] Make your own complex data type
- [x] Initiate array of 8 complex numbers (just a boring one)
- [x] Implement the inplace FFT algorithm on them, RADIX 2 DIT CT with an array of length 8 )(i.e. three stages)
  - [x] Init second array of 8 complex numbers for flip-flopping purposes
  - [x] Static SINE function to use in FFT
  
- [ ] modify complex struct so that it stores a maximum amount of bits
- [ ] modify SINE function so that it sores correct number of bits


## Debugging 

Output of `cargo run`

```
before fft8: 0 + i 0
idx=0, bfi=0
idx=1, bfi=4
idx=2, bfi=2
idx=3, bfi=6
idx=4, bfi=1
idx=5, bfi=5
idx=6, bfi=3
idx=7, bfi=7
DEBUG: bit-switch complete, result:
[1000+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, ]
----------------------

DEBUG: stage 1

DEBUG: chunk=0
[1999+i0, -1+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, ]

DEBUG: chunk=1
[1999+i0, -1+i0, 1999+i0, -1+i0, 1000+i0, 1000+i0, 1000+i0, 1000+i0, ]

DEBUG: chunk=2
[1999+i0, -1+i0, 1999+i0, -1+i0, 1999+i0, -1+i0, 1000+i0, 1000+i0, ]

DEBUG: chunk=3
[1999+i0, -1+i0, 1999+i0, -1+i0, 1999+i0, -1+i0, 1999+i0, -1+i0, ]
----------------------

DEBUG: stage 2

DEBUG: chunk=0
[3997+i0, -1+i-1, -1+i0, 1+i-1, 1999+i0, -1+i0, 1999+i0, -1+i0, ]

DEBUG: chunk=1
[3997+i0, -1+i-1, -1+i0, 1+i-1, 3997+i0, -1+i-1, -1+i0, 1+i-1, ]
----------------------

DEBUG: stage 3

DEBUG: chunk=0
[7993+i0, -1+i-3, -1+i-1, 1+i0, -1+i0, 1+i-1, 1+i-1, -1+i2, ]
DEBUG: #2
After fft8: 7993 + i 0
7u16 in bits: 0000000000000111
one 00000001
n_one 11111111

Out:

[7993+i0, -1+i-3, -1+i-1, 1+i0, -1+i0, 1+i-1, 1+i-1, -1+i2, ]
```


