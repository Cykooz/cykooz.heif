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
