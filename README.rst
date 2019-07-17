***********
cykooz.heif
***********

cykooz.heif is simple python wrapper for the library `libheif-rs <https://crates.io/crates/libheif-rs>`_.

``RawHeifImage`` is a simple wrapper around low level HEIF-decoder.

.. code-block:: python

    from cykooz.heif.image import RawHeifImage

    img = RawHeifImage('data/test.heif')
    assert img.width == 3024
    assert img.height == 4032
    assert img.mode == 'RGB'
    assert len(img.data) == 36578304
    assert img.stride == 9072
    assert len(img.exif) == 2026


Also package provides an opener plugin for ``Pillow``:

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
