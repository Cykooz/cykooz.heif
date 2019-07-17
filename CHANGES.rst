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

Next release
============

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
