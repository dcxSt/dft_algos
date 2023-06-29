pub mod complex;
pub mod constants;
pub mod intfft;
pub mod intfft_vec;
//pub mod iomod;

use complex::Complex;
use intfft::int_fft;
//use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn, PyArray};
use numpy::ndarray::{ArrayD, ArrayViewD, Ix1}; //, ArrayViewMutD};

// macros
macro_rules! log2 {
    ($val:expr) => {
        ($val as f32).log(2.0) as u32
    }
}

/// Python module
#[pymodule]
fn integer_fft(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    fn dummy_add_arrays(x: ArrayViewD<'_, i64>, y: ArrayViewD<'_, i64>) -> ArrayD<i64> {
        &x + &y
    }
    #[pyfn(m)]
    #[pyo3(name = "dummy")]
    fn dummy<'py>(
        py: Python<'py>,
        x: PyReadonlyArrayDyn<i64>,
        y: PyReadonlyArrayDyn<i64>,
        ) -> &'py PyArrayDyn<i64> {
        let x = x.as_array();
        let y = y.as_array();
        let z = dummy_add_arrays(x, y);
        z.into_pyarray(py)
    }
    #[pyfn(m)]
    #[pyo3(name = "integer_fft_2048")]
    fn integer_fft_2048<'py>(
        py: Python<'py>,
        xre: PyReadonlyArrayDyn<i64>,
        xim: PyReadonlyArrayDyn<i64>,
        ) -> (&'py PyArray<i64, Ix1>, &'py PyArray<i64, Ix1>) {
        let xre = xre.as_array();
        let xim = xim.as_array();
        assert_eq!(xre.len(), 2048);
        assert_eq!(xim.len(), 2048);
        // initiate input arrays
        let mut flip: [Complex; 2048] = [Complex::new(0, 0); 2048];
        let mut flop: [Complex; 2048] = [Complex::new(0, 0); 2048];
        // Fill flip with data
        for i in 0..2048 {
            flip[i] = Complex::new(xre[i], xim[i]);
        }
        int_fft(&mut flip, &mut flop, 16, 18);
        // Now flop has fft'd data
        // Read data back into two i64 arrays
        let mut outre = Vec::<i64>::new();
        let mut outim = Vec::<i64>::new();
        for i in 0..2048 {
            outre.push(flop[i].re);
            outim.push(flop[i].im);
        }
        let outre_pyarray = PyArray::from_vec(py, outre);
        let outim_pyarray = PyArray::from_vec(py, outim);
        return (outre_pyarray, outim_pyarray);
    }
    #[pyfn(m)]
    #[pyo3(name = "fft")]
    fn integer_fft_vec<'py>(
        py: Python<'py>,
        xre: PyReadonlyArrayDyn<i64>,
        xim: PyReadonlyArrayDyn<i64>,
        ndatabits: usize, // not more than 23 i think
        nsinebits: usize, // not more than 16
    ) -> (&'py PyArray<i64, Ix1>, &'py PyArray<i64, Ix1>) {
        assert!(ndatabits<=23, "Bit depth of data can't exceed 23.");
        assert!(nsinebits<=16, "Bit depth of twiddle factors can't exceed 16.");
        let xre = xre.as_array();
        let xim = xim.as_array();
        let lframe: usize = xre.len();
        assert!(lframe<=2048, "Maximum lengthed arrays supported up to 2048.");
        let n: u32 = log2!(lframe);
        assert_eq!(xim.len(), lframe, "Real and imaginary parts must be of equal len.");
        assert_eq!(lframe, 2usize.pow(n), "This package only FFTs powers-of-2-lengthed arrays.");
        // Declare input arrays
        let mut flip = Vec::<Complex>::with_capacity(lframe);
        let mut flop = Vec::<Complex>::with_capacity(lframe);
        // Initiate input array, 
        // Maybe exists better way than deep copy forloop?
        for i in 0..lframe {
            flip.push(Complex::new(xre[i], xim[i]));
            flop.push(Complex::new(0,0));
        }
        // Perform FFT
        intfft_vec::int_fft(&mut flip, &mut flop, n, ndatabits, nsinebits);
        // Now flop has fft'd data
        // Read data back into two i64 arrays
        let mut outre = Vec::<i64>::new();
        let mut outim = Vec::<i64>::new();
        for i in 0..lframe {
            outre.push(flop[i].re);
            outim.push(flop[i].im);
        }
        let outre_pyarray = PyArray::from_vec(py, outre);
        let outim_pyarray = PyArray::from_vec(py, outim);
        return (outre_pyarray, outim_pyarray);
    }
//     /// Python wrapper for integer FFT
//     #[pyfn(m)]
//     #[pyo3(name = "integerfft")]
//     fn int_fft_py<'py>(
//         py: Python<'py>,
//         xre: PyReadonlyArrayDyn<i64>,
//         xim: PyReadonlyArrayDyn<i64>,
//         ) -> (&'py PyArrayDyn<i64>, &'py PyArrayDyn<i64>) {
//         let xre = xre.as_array();
//         let xim = xim.as_array();
//         let len: usize = xre.len(); // length of array
//         let mut flip: [Complex] = [Complex::new(0,0)];
//         let mut flop: [Complex] = [Complex::new(0,0)];
// 
//     }
    Ok(())
}

