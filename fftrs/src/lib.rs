pub mod complex;
pub mod constants;
pub mod intfft;
//pub mod iomod;

//use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use numpy::ndarray::{ArrayD, ArrayViewD}; //, ArrayViewMutD};

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

