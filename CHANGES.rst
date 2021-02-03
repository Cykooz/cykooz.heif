..  Changelog format guide.
    - Before make new release of egg you MUST add here a header for new version with name "Next release".
    - After all headers and paragraphs you MUST add only ONE empty line.
    - At the end of sentence which describes some changes SHOULD be identifier of task from our task manager.
      This identifier MUST be placed in brackets. If a hot fix has not the task identifier then you
      can use the word "HOTFIX" instead of it.
    - At the end of sentence MUST stand a point.
    - List of changes in the one version MUST be grouped in the next sections:
        - Features
        - Changes
        - Bug Fixes
        - Docs

CHANGELOG
*********

0.10.4 (2021-02-03)
===================

Changes
-------

- Updated version of ``libheif-rs`` to 0.13.1.

0.10.2 (2021-01-15)
===================

Changes
-------

- Updated version of ``libheif-rs`` to 0.13.

0.10 (2021-01-14)
=================

Changes
-------

- Updated version of ``libheif-rs`` to 0.12.
- Updated version of ``pyo3`` to 0.13.1 (dropped support of Python 3.5).

0.9 (2020-09-26)
================

Changes
-------

- Updated version of ``libheif-rs`` to 0.11.
- Updated version of ``pyo3`` to 0.12.1.

Wheels
------

- Updated version of ``libheif`` to 1.9.1.
- Added ``dav1d`` decoder for faster decoding of AVIF images.

0.8.3 (2020-08-30)
==================

Bug Fixes
---------

- Fixed building of wheels:
    - added ``libaom``;
    - added ``rav1e`` encoder for AV1;
    - added stripping of libraries to reduce size of wheels.

0.8 (2020-08-29)
================

Changes
-------

- Updated version of ``libheif-rs`` to 0.10:
    - updated version of ``libheif-sys`` to 1.8.1;
    - added support of new compression format AV1.
- Updated version of ``pyo3`` to 0.11.1.

0.7.2 (2020-03-20)
==================

Changes
-------

- Updated version of ``pyo3`` to 0.9.

Bug Fixes
---------

- Fixed namespace declaration.

0.7 (2020-03-01)
================

Changes
-------

- Updated version of ``libheif-rs`` to 0.9.

0.6 (2019-10-03)
================

Changes
-------

- Updated version of ``pyo3`` to 0.8.
- Updated version of ``libheif-rs`` to 0.8.

0.5 (2019-08-28)
================

Changes
-------

- Updated version of ``libheif-rs`` to 0.6.

0.4.2 (2019-07-17)
==================

Bug Fixes
---------

- Added checking of image type inside of ``HeifImageFile._open()``.

0.4 (2019-07-17)
================

Features
--------

- Added ``RawHeifImage.check_file_type`` to check by first bytes of file
  what it file is supported by ``libheif``.
- Added opener plugin for ``Pillow``.

0.3 (2019-06-28)
================

Features
--------

- Added method for creating ``HeifImage`` from any file-like object.

0.2 (2019-06-25)
================

Changes
-------

- Added exception ``HeifError``.

0.1 (2019-06-25)
================

- Initial version.
