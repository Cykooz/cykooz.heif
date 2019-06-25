# -*- coding: utf-8 -*-
"""
:Authors: cykooz
:Date: 23.06.2019
"""
from pathlib import Path

import piexif
import pytest

from cykooz.heif.image import HeifImage
from cykooz.heif.utils import get_pil_image


@pytest.fixture(name='data_path')
def data_path_fixture() -> Path:
    return Path(__file__).parent / 'data'


def test_heif_image(data_path):
    img = HeifImage(data_path / 'test.heic')
    assert img.width == 3024
    assert img.height == 4032
    assert img.mode == 'RGB'
    assert len(img.data) == 36578304
    assert img.stride == 9072
    assert len(img.exif) == 2026


def test_get_pillow_image(data_path):
    img = get_pil_image(data_path / 'test.heic')
    assert img.size == (3024, 4032)
    assert img.mode == 'RGB'

    assert 'exif' in img.info
    exif = piexif.load(img.info['exif'])
    assert exif['Exif'][42035] == b'Apple'
    assert exif['Exif'][42036] == b'iPhone 7 Plus back dual camera 6.6mm f/2.8'


def test_get_pillow_image_errors(data_path):
    with pytest.raises(RuntimeError):
        get_pil_image(data_path / 'not_found.heic')

