# -*- coding: utf-8 -*-
"""
:Authors: cykooz
:Date: 24.06.2019
"""
from PIL import Image

from .heif_image import HeifImage
from .typing import PathLike


def get_pil_image(path: PathLike) -> Image.Image:
    heif_image = HeifImage(path)
    mode = heif_image.mode
    image = Image.frombytes(
        mode, (heif_image.width, heif_image.height), heif_image.data,
        'raw', mode, heif_image.stride
    )
    exif = heif_image.exif
    if exif:
        image.info['exif'] = exif
    return image
