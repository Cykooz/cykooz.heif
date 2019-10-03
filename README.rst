***********
cykooz.heif
***********

``cykooz.heif`` is simple python wrapper for the library `libheif-rs <https://crates.io/crates/libheif-rs>`_.

``RawHeifImage`` is a simple wrapper around low level HEIF-decoder.

Usage Examples
==============

Read HEIF-image from file:

.. code-block:: python

    from cykooz.heif.image import RawHeifImage

    img = RawHeifImage.from_path('data/test.heif')
    assert img.width == 3024
    assert img.height == 4032
    assert img.mode == 'RGB'
    assert len(img.data) == 36578304
    assert img.stride == 9072
    assert len(img.exif) == 2026

Read HEIF-image from file-like object:

.. code-block:: python

    from cykooz.heif.image import RawHeifImage

    with open('data/test.heif') as fp
        img = RawHeifImage.from_stream(fp)
        assert img.width == 3024
        assert img.height == 4032

Also package provides an opener plugin for ``PIL`` (``Pillow``):

.. code-block:: python

    from PIL import Image
    from cykooz.heif.pil import register_heif_opener

    register_heif_opener()
    img = Image.open('data/test.heif')
    assert isinstance(img, Image.Image)
    assert img.size == (3024, 4032)
    assert img.mode == 'RGB'
    assert img.getpixel((100, 100)) == (73, 73, 69)
    img.save('test.jpg', 'JPEG')

Installation from source
========================

System requirements:

- Nightly Rust (https://www.rust-lang.org/)
- libheif-dev >= 1.5 (https://github.com/strukturag/libheif)
- python3-dev (tested with Python 3.6 and 3.7)

Ubuntu 18.04
------------

.. code-block:: console

    $ sudo add-apt-repository ppa:strukturag/libheif
    $ sudo apt-get install build-essential python3.7-dev libheif-dev curl
    $ curl https://sh.rustup.rs -sSf | sh
    $ source $HOME/.cargo/env
    $ rustup toolchain install nightly
    $ pip3 install -U setuptools setuptools-rust
    $ PYTHON_SYS_EXECUTABLE=python3 pip3 install cykooz.heif
