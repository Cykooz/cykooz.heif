# -*- coding: utf-8 -*-
"""
:Authors: cykooz
:Date: 23.06.2019
"""
from pathlib import Path

import piexif
import pytest
from PIL import Image

from cykooz.heif.errors import HeifError
from cykooz.heif.image import RawHeifImage
from cykooz.heif.pil import register_heif_opener


@pytest.fixture(scope='session', autouse=True)
def reg_pil_opener():
    register_heif_opener()


@pytest.fixture(name='data_path')
def data_path_fixture() -> Path:
    return Path(__file__).parent / 'data'


def test_raw_heif_image_form_path(data_path):
    img = RawHeifImage.from_path(data_path / 'test.heic')
    assert img.width == 3024
    assert img.height == 4032
    assert img.mode == 'RGB'
    assert len(img.data) == 36578304
    assert img.stride == 9072
    assert len(img.exif) == 2026


def test_raw_heif_image_form_reader(data_path):
    img_path = data_path / 'test.heic'
    with img_path.open('rb') as f:
        img = RawHeifImage.from_stream(f)
        assert img.width == 3024
        assert img.height == 4032
        assert img.mode == 'RGB'
        assert len(img.data) == 36578304
        assert img.stride == 9072
        assert len(img.exif) == 2026


def test_raw_heif_image_form_reader_errors(data_path):
    img_path = data_path / 'test.heic'
    with img_path.open('rb') as f:
        img = RawHeifImage.from_stream(f)
        assert img.width == 3024
        assert img.height == 4032
    # File is closed
    with pytest.raises(HeifError):
        _ = img.data


@pytest.mark.parametrize(
    ['source_type'],
    [
        ('path',),
        ('stream',),
    ]
)
@pytest.mark.parametrize(
    ['file_name'],
    [
        ('test.heic',),
        ('heic_as.jpg',),
    ]
)
def test_open_pillow_image(data_path, source_type, file_name):
    fp = data_path / file_name
    if source_type == 'stream':
        fp = open(str(fp), 'rb')

    img: Image.Image = Image.open(fp)
    assert img.size == (3024, 4032)
    assert img.mode == 'RGB'

    assert 'exif' in img.info
    exif = piexif.load(img.info['exif'])
    assert exif['Exif'][42035] == b'Apple'
    assert exif['Exif'][42036] == b'iPhone 7 Plus back dual camera 6.6mm f/2.8'

    pixel = img.getpixel((100, 100))
    assert pixel == (73, 73, 69)


def test_open_png_as_heif(data_path):
    fp = data_path / 'png_as.heif'
    img: Image.Image = Image.open(fp)

    assert img.size == (1280, 720)
    assert img.mode == 'RGB'
    assert 'exif' not in img.info

    pixel = img.getpixel((100, 100))
    assert pixel == (132, 185, 255)
