# How to use

This program performs a 2048 point integer FFT on 32 bit integer data in numpy serialized arrays, files with the `.npy` extention. If you'd like to add flexibility, feel free to play with the code, I did my best to make it easy to adapt. 

Clone this repository

```
git clone https://github.com/dcxSt/dft_algos.git
```

Change directory 

```
cd dft_algos/fftrs
```

Build the binaries with cargo (install instructions for cargo [here](https://doc.rust-lang.org/cargo/getting-started/installation.html))

```
cargo build --release
```

Copy the binary program that you just compiled into, wherever you want it to be (perhaps the same place as your simulated data)

```
cp target/release/fftrs /any/directory/you/want/
```

Go into the directory you just copied the binary into. Run the program, supplying three arguments

```
./fftrs <name_of_npy_file.npy> <nbitshift> <number_of_bits_for_data> <number_of_sine_bits>
```

The number of bits to shift the input (so that the inter-butterfly stage scaling doesn't kill the signal) is the first input after the name of the npy file. The number of sine bits can be at most 16, and the number of data bits must be even and at most 18. 

For instance

```
./fftrs dc100.npy 18 16
```

It will output the DFT info files `<input_file_basename>_out_real.npy` and `<output_file_basename>_out_imag.npy`. Have fun. 



## Dev stuff

*Reminder:* The optimal STD to select for the FT of an 8-bit quantized input is 35. I.e. when generating simulated data scale your gaussian noise by 35 before throwing converting to int and throwing it into the integer FFT. 

*Remark:* if you'd like to display trace, debug or info logging statements, run `RUST_LOG=trace cargo run`

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


