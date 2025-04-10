use std::io;

use pyo3::prelude::*;
use pyo3::types::PyBytes;

pub(crate) struct StreamFromPy {
    pub(crate) py_stream: PyObject,
}

impl io::Read for StreamFromPy {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Python::with_gil(
            |py| match self.py_stream.call_method1(py, "read", (buf.len(),)) {
                Ok(v) => {
                    let py_bytes: &Bound<PyBytes> = v.downcast_bound(py).map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            "Error during casting PyObject into PyBytes \
                         ('read' method returns not a bytes)",
                        )
                    })?;
                    let bytes = py_bytes.as_bytes();
                    if !bytes.is_empty() {
                        buf[..bytes.len()].copy_from_slice(bytes);
                    }
                    Ok(bytes.len())
                }
                Err(e) => {
                    let err_str: String = e
                        .into_pyobject(py)
                        .unwrap()
                        .call_method0("__str__")
                        .unwrap()
                        .extract()
                        .unwrap();
                    Err(io::Error::new(io::ErrorKind::Other, err_str))
                }
            },
        )
    }
}

impl io::Seek for StreamFromPy {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let (whence, offset) = match pos {
            io::SeekFrom::Start(offset) => (0, offset as i64),
            io::SeekFrom::Current(offset) => (1, offset),
            io::SeekFrom::End(offset) => (2, offset),
        };

        Python::with_gil(
            |py| match self.py_stream.call_method1(py, "seek", (offset, whence)) {
                Ok(v) => {
                    let pos: u64 = v.extract(py).map_err(|_| {
                        io::Error::new(io::ErrorKind::Other, "Method 'seek' returns not u64")
                    })?;
                    Ok(pos)
                }
                Err(e) => {
                    let err_str: String = e
                        .into_pyobject(py)
                        .unwrap()
                        .call_method0("__str__")
                        .unwrap()
                        .extract()
                        .unwrap();
                    Err(io::Error::new(io::ErrorKind::Other, err_str))
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek};

    use pyo3::ffi::c_str;
    use pyo3::types::IntoPyDict;

    use super::*;

    #[test]
    fn test_read() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let locals = [("io", py.import("io")?)].into_py_dict(py)?;
            let code = c_str!("io.BytesIO(b'a' * 100 + b'b' * 50)");
            let result = py.eval(code, None, Some(&locals))?;
            let mut stream_from_py = StreamFromPy {
                py_stream: result.into_pyobject(py)?.into(),
            };
            let mut buf = vec![0u8; 100];

            let size = stream_from_py.read(&mut buf)?;
            assert_eq!(size, 100);
            assert_eq!(buf, vec![b'a'; 100]);

            let size = stream_from_py.read(&mut buf)?;
            assert_eq!(size, 50);
            let mut expect = vec![b'b'; 50];
            expect.append(&mut vec![b'a'; 50]);
            assert_eq!(buf, expect);

            let size = stream_from_py.read(&mut buf)?;
            assert_eq!(size, 0);
            assert_eq!(buf, expect);

            Ok(())
        })
    }

    #[test]
    fn test_seek() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let locals = [("io", py.import("io")?)].into_py_dict(py)?;
            let code = c_str!("io.BytesIO(b'a' * 100 + b'b' * 50)");
            let result = py.eval(code, None, Some(&locals))?;
            let mut stream_from_py = StreamFromPy {
                py_stream: result.into_pyobject(py)?.into(),
            };

            let pos = stream_from_py.seek(io::SeekFrom::Start(0))?;
            assert_eq!(pos, 0);

            let pos = stream_from_py.seek(io::SeekFrom::End(0))?;
            assert_eq!(pos, 150);

            let pos = stream_from_py.seek(io::SeekFrom::Start(100))?;
            assert_eq!(pos, 100);

            let pos = stream_from_py.stream_position()?;
            assert_eq!(pos, 100);

            let mut buf = vec![0u8; 100];
            let size = stream_from_py.read(&mut buf)?;
            assert_eq!(size, 50);
            let mut expect = vec![b'b'; 50];
            expect.append(&mut vec![0u8; 50]);
            assert_eq!(buf, expect);

            let pos = stream_from_py.stream_position()?;
            assert_eq!(pos, 150);

            Ok(())
        })
    }
}
