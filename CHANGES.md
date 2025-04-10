# CHANGELOG

## 1.2.0 (2025-04-08)

### Changes

- Updated version of ``libheif-rs`` to 2.0.0 (improved support HEIC-files from iOS 18+)
- Updated version of ``pyo3`` to 0.24.

## 1.1.0 (2025-02-04)

### Changes

- Updated version of ``pyo3`` to 0.23.

### Bug Fixes

- Updated version of ``libheif-rs`` to 1.1.0 (support HEIC-files from iOS 18+)

## 1.0.0 (2024-01-26)

### Features

- Added support for Pillow >= 10.1

### Changes

- Updated version of ``libheif-rs`` to 1.0.
- Updated version of ``pyo3`` to 0.20.

## 0.14 (2023-09-09)

### Changes

- Updated version of ``libheif-rs`` to 0.22.0.
- Updated version of ``pyo3`` to 0.19.2.

### Breaking Changes

- Dropped support of Python 3.7.
- Version of ``manylinux`` wheels updated to 2.28.

## 0.13.1 (2023-02-01)

### Changes

- Updated version of ``pyo3`` to 0.18.

### Bug Fixes

- Fixed error with receiving zero sized EXIF block.

## 0.13 (2022-10-26)

### Changes

- Fixed metadata of package.
- Added building of wheel for Python 3.11.
- Updated version of ``libheif-rs`` to 0.15.1.
- Updated version of ``pyo3`` to 0.17.2.

## 0.12 (2022-05-20)

### Breaking Changes

- Dropped support of Python 3.6.
- Version of ``manylinux`` wheels updated to 2.24.

### Changes

- Updated version of ``libheif-rs`` to 0.15.
- Updated version of ``pyo3`` to 0.16.5.
- Added building of wheel for Python 3.10.

## 0.11 (2021-05-12)

### Changes

- Updated version of ``libheif-rs`` to 0.14.

### Wheels

- Fixed building of ``libheif`` with ``dav1d`` and ``rav1e`` libraries.

## 0.10 (2021-01-14)

### Changes

- Updated version of ``libheif-rs`` to 0.12.
- Updated version of ``pyo3`` to 0.13.1 (dropped support of Python 3.5).

## 0.9 (2020-09-26)

### Changes

- Updated version of ``libheif-rs`` to 0.11.
- Updated version of ``pyo3`` to 0.12.1.

### Wheels

- Updated version of ``libheif`` to 1.9.1.
- Added ``dav1d`` decoder for faster decoding of AVIF images.

## 0.8.3 (2020-08-30)

### Bug Fixes

- Fixed building of wheels:
    - added ``libaom``;
    - added ``rav1e`` encoder for AV1;
    - added stripping of libraries to reduce size of wheels.

## 0.8 (2020-08-29)

### Changes

- Updated version of ``libheif-rs`` to 0.10:
    - updated version of ``libheif-sys`` to 1.8.1;
    - added support of new compression format AV1.
- Updated version of ``pyo3`` to 0.11.1.

## 0.7.2 (2020-03-20)

### Changes

- Updated version of ``pyo3`` to 0.9.

### Bug Fixes

- Fixed namespace declaration.

## 0.7 (2020-03-01)

### Changes

- Updated version of ``libheif-rs`` to 0.9.

## 0.6 (2019-10-03)

### Changes

- Updated version of ``pyo3`` to 0.8.
- Updated version of ``libheif-rs`` to 0.8.

## 0.5 (2019-08-28)

### Changes

- Updated version of ``libheif-rs`` to 0.6.

## 0.4.2 (2019-07-17)

### Bug Fixes

- Added checking of image type inside of ``HeifImageFile._open()``.

## 0.4 (2019-07-17)

### Features

- Added ``RawHeifImage.check_file_type`` to check by first bytes of file
  what it file is supported by ``libheif``.
- Added opener plugin for ``Pillow``.

## 0.3 (2019-06-28)

### Features

- Added method for creating ``HeifImage`` from any file-like object.

## 0.2 (2019-06-25)

### Changes

- Added exception ``HeifError``.

## 0.1 (2019-06-25)

- Initial version.
