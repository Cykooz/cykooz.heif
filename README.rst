***********
cykooz.heif
***********

cykooz.heif is a python wrapper above Rust library `libheif-rs <https://crates.io/crates/libheif-rs>`_.


Examples
--------

.. code-block:: python

    from cykooz.heif.image import HeifImage

    img = HeifImage('data/test.heif')
    assert img.width == 3024
    assert img.height == 4032
    assert img.mode == 'RGB'
    assert len(img.data) == 36578304
    assert img.stride == 9072
    assert len(img.exif) == 2026

.. code-block:: python

    from PIL.Image import Image
    from cykooz.heif.utils import get_pil_image

    img = get_pil_image('data/test.heif')
    assert isinstance(img, Image)
    assert img.size == (3024, 4032)
    assert img.mode == 'RGB'
    img.save('test.jpg', 'JPEG')
