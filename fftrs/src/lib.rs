pub mod complex;
pub mod constants;
pub mod intfft;
//pub mod iomod;

use complex::Complex;
use intfft::fft_quantized;
//use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn, PyArray};
use numpy::ndarray::{ArrayD, ArrayViewD, Ix1}; //, ArrayViewMutD};

//use complex::Complex;
//use constants::{QUART_WAV, SINE};
//use intfft::{copy_ab, fft_quantized};
//use iomod::{output_to_npy, read_npyi32};
//use log::{debug, info, trace};
//use std::env; // retrieve arguments



// // Fill a complex array with data from 
// fn fill_complex_array(xre: &[i64], xim: &[i64]) -> [Complex] {
//     assert_eq!(xre.len(),xim.len());
//     let len: usize = xre.len();
//     let mut xout: [Complex] = [Complex::new(0, 0)];
//     return xout;
// }
// 
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
        fft_quantized(&mut flip, &mut flop, 16, 18);
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
//     /// Python wrapper for integer FFT
//     #[pyfn(m)]
//     #[pyo3(name = "integerfft")]
//     fn fft_quantized_py<'py>(
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

