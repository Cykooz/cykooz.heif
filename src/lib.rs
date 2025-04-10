use std::io::BufReader;
use std::sync::{Arc, Mutex};

use libheif_rs::{
    ColorSpace, DecodingOptions, FileTypeResult, HeifContext, ItemId, LibHeif, Reader, RgbChroma,
    StreamReader,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::{exceptions, wrap_pyfunction};

use crate::stream::StreamFromPy;

mod stream;

fn result2pyresult<T>(res: libheif_rs::Result<T>) -> PyResult<T> {
    res.map_err(|heif_error| PyErr::new::<exceptions::PyRuntimeError, _>(heif_error.to_string()))
}

#[pyclass]
struct HeifImage {
    heif_context: Arc<Mutex<HeifContext<'static>>>,
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
    /// get_data(ignore_transformations: bool) -> Optional[Tuple[bytes, int, int]]
    /// --
    ///
    /// Returns tuple with image pixels data, stride and bits per pixel.
    ///
    /// :rtype: Optional[Tuple[bytes, int, int]]
    fn get_data<'py>(
        &self,
        py: Python<'py>,
        ignore_transformations: bool,
    ) -> PyResult<Bound<'py, PyTuple>> {
        let lib_hef = LibHeif::new();
        let context_mutex = self.heif_context.clone();
        let image = py.allow_threads(move || {
            let context = context_mutex.lock().unwrap();
            let handle = context.primary_image_handle()?;
            let chroma = if handle.has_alpha_channel() {
                RgbChroma::Rgba
            } else {
                RgbChroma::Rgb
            };
            let decoding_options = DecodingOptions::new().map(|mut options| {
                options.set_ignore_transformations(ignore_transformations);
                options
            });
            lib_hef.decode(&handle, ColorSpace::Rgb(chroma), decoding_options)
        });

        let image = result2pyresult(image)?;
        let planes = image.planes();

        let data: PyObject;
        let stride: PyObject;
        let bits_pre_pixel: PyObject;
        match planes.interleaved {
            Some(plane) => {
                data = PyBytes::new(py, plane.data).into();
                stride = plane.stride.into_pyobject(py)?.into();
                bits_pre_pixel = plane.bits_per_pixel.into_pyobject(py)?.into();
            }
            None => {
                data = py.None();
                stride = py.None();
                bits_pre_pixel = py.None();
            }
        };
        PyTuple::new(py, &[data, stride, bits_pre_pixel])
    }

    fn get_exif(&self, py: Python) -> PyResult<PyObject> {
        let context = self.heif_context.lock().unwrap();
        let handle = result2pyresult(context.primary_image_handle())?;
        let mut meta_ids: [ItemId; 1] = [0];
        let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
        if count == 0 {
            Ok(py.None())
        } else {
            if handle.metadata_size(meta_ids[0]) == 0 {
                // Invalid Exif block. It may have been incorrectly removed from the file.
                return Ok(py.None());
            }
            let exif = result2pyresult(handle.metadata(meta_ids[0]))?;
            Ok(PyBytes::new(py, &exif[4..]).into())
        }
    }
}

/// open_heif_from_path(path: str) -> HeifImage
/// --
///
/// This function opens HEIF file form given path and returns
/// instance of HeifImage.
///
/// :type path: str
/// :rtype: HeifImage
#[pyfunction]
fn open_heif_from_path(py: Python, path: &str) -> PyResult<HeifImage> {
    result2pyresult(py.allow_threads(move || open_heif_from_path_impl(path)))
}

fn open_heif_from_path_impl(path: &str) -> libheif_rs::Result<HeifImage> {
    let context = HeifContext::read_from_file(path)?;
    py_image_from_context(context)
}

/// open_heif_from_reader(reader, total_size: int) -> HeifImage
/// --
///
/// This function opens HEIF file form given reader instance and returns
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

fn open_heif_context_from_reader_impl(reader: Box<dyn Reader>) -> libheif_rs::Result<HeifImage> {
    let context = HeifContext::read_from_reader(reader)?;
    py_image_from_context(context)
}

fn py_image_from_context(context: HeifContext<'static>) -> libheif_rs::Result<HeifImage> {
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

/// check_file_type(data: bytes) -> str
/// --
///
/// Check a file type by it first bytes.
/// Input data should be at least 12 bytes.
///
/// :type data: bytes
/// :rtype: str
#[pyfunction]
fn check_file_type(py: Python, data: PyObject) -> PyResult<String> {
    let py_bytes = data.downcast_bound::<PyBytes>(py)?;
    let bytes = py_bytes.as_bytes();
    let res = libheif_rs::check_file_type(bytes);
    Ok(match res {
        FileTypeResult::No => "no".into(),
        FileTypeResult::Supported => "supported".into(),
        FileTypeResult::Unsupported => "unsupported".into(),
        FileTypeResult::MayBe => "maybe".into(),
    })
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rust_lib(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(open_heif_from_path))?;
    m.add_wrapped(wrap_pyfunction!(open_heif_from_reader))?;
    m.add_wrapped(wrap_pyfunction!(check_file_type))?;
    m.add_class::<HeifImage>()?;

    Ok(())
}
