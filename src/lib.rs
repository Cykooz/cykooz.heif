use std::sync::{Arc, Mutex};

use crate::stream::StreamFromPy;
use libheif_rs::{Chroma, ColorSpace, HeifContext, HeifError, Reader, StreamReader};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::wrap_pyfunction;
use std::io::BufReader;

mod stream;

fn result2pyresult<T>(res: Result<T, HeifError>) -> PyResult<T> {
    match res {
        Ok(res) => Ok(res),
        Err(heif_error) => Err(PyErr::new::<exceptions::RuntimeError, _>(
            heif_error.to_string(),
        )),
    }
}

#[pyclass]
struct HeifImage {
    heif_context: Arc<Mutex<HeifContext>>,
    /// Image mode.
    #[pyo3(get)]
    mode: String,
    /// Image width.
    #[pyo3(get)]
    width: u32,
    /// Image height.
    #[pyo3(get)]
    height: u32,
}

#[pymethods]
impl HeifImage {
    /// get_data() -> Optional[Tuple[bytes, int, int]]
    /// --
    ///
    /// Returns tuple with image pixels data, stride and bits per pixel.
    ///
    /// :rtype: Optional[Tuple[bytes, int, int]]
    fn get_data(&self, py: Python) -> PyResult<PyObject> {
        let context_mutex = self.heif_context.clone();
        let image = py.allow_threads(move || {
            let context = context_mutex.lock().unwrap();
            let handle = context.primary_image_handle()?;
            let chroma = if handle.has_alpha_channel() {
                Chroma::InterleavedRgba
            } else {
                Chroma::InterleavedRgb
            };
            handle.decode(ColorSpace::RGB, chroma)
        });

        let image = result2pyresult(image)?;
        let planes = image.planes();

        let data: PyObject;
        let stride: PyObject;
        let bits_pre_pixel: PyObject;
        match planes.interleaved {
            Some(plane) => {
                data = PyBytes::new(py, plane.data).into();
                stride = plane.stride.to_object(py);
                bits_pre_pixel = plane.bits_pre_pixel.to_object(py);
            }
            None => {
                data = py.None();
                stride = py.None();
                bits_pre_pixel = py.None();
            }
        };
        let res: PyObject = PyTuple::new(py, &[data, stride, bits_pre_pixel]).into();
        Ok(res)
    }

    fn get_exif(&self, py: Python) -> PyResult<PyObject> {
        let context = self.heif_context.lock().unwrap();
        let handle = result2pyresult(context.primary_image_handle())?;
        let meta_ids = handle.list_of_metadata_block_ids("Exif", 1);
        if meta_ids.is_empty() {
            Ok(py.None())
        } else {
            let exif = result2pyresult(handle.metadata(meta_ids[0]))?;
            Ok(PyBytes::new(py, &exif[4..]).into())
        }
    }
}

/// open_heif_from_path(path: str) -> HeifImage
/// --
///
/// This function open HEIF file form given path and returns
/// instance of HeifImage.
///
/// :type path: str
/// :rtype: HeifImage
#[pyfunction]
fn open_heif_from_path(py: Python, path: &str) -> PyResult<HeifImage> {
    result2pyresult(py.allow_threads(move || open_heif_from_path_impl(path)))
}

fn open_heif_from_path_impl(path: &str) -> Result<HeifImage, HeifError> {
    let context = HeifContext::read_from_file(path)?;
    py_image_from_context(context)
}

/// open_heif_from_reader(reader, total_size: int) -> HeifImage
/// --
///
/// This function open HEIF file form given reader instance and returns
/// instance of HeifImage.
///
/// :type reader: typing.BinaryIO
/// :type total_size: int
/// :rtype: HeifImage
#[pyfunction]
fn open_heif_from_reader(py: Python, reader: PyObject, total_size: u64) -> PyResult<HeifImage> {
    let stream_from_py = StreamFromPy {
        py_stream: reader.clone_ref(py),
    };
    let stream_from_py = BufReader::new(stream_from_py);
    result2pyresult(py.allow_threads(move || {
        open_heif_context_from_reader_impl(Box::new(StreamReader::new(stream_from_py, total_size)))
    }))
}

fn open_heif_context_from_reader_impl(reader: Box<dyn Reader>) -> Result<HeifImage, HeifError> {
    let context = HeifContext::read_from_reader(reader)?;
    py_image_from_context(context)
}

fn py_image_from_context(context: HeifContext) -> Result<HeifImage, HeifError> {
    let handle = context.primary_image_handle()?;
    let width = handle.width();
    let height = handle.height();
    let mode = if handle.has_alpha_channel() {
        "RGBA"
    } else {
        "RGB"
    };

    drop(handle);

    Ok(HeifImage {
        heif_context: Arc::new(Mutex::new(context)),
        mode: mode.to_string(),
        width,
        height,
    })
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust2py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(open_heif_from_path))?;
    m.add_wrapped(wrap_pyfunction!(open_heif_from_reader))?;
    m.add_class::<HeifImage>()?;

    Ok(())
}
