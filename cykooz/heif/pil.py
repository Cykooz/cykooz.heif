# -*- coding: utf-8 -*-
"""
:Authors: cykooz
:Date: 24.06.2019
"""
from PIL import Image, ImageFile

from .image import RawHeifImage


class HeifImageFile(ImageFile.ImageFile):
    format = 'HEIF'
    format_description = "HEIF/HEIC image"

    def _open(self):
        raw_heif_image = RawHeifImage.from_stream(self.fp)

        # size in pixels (width, height)
        self._size = raw_heif_image.width, raw_heif_image.height

        # mode setting
        self.mode = raw_heif_image.mode

        exif = raw_heif_image.exif
        if exif:
            self.info['exif'] = exif

        offset = self.fp.tell()
        self.tile = [
            ('heif', (0, 0) + self.size, offset, (raw_heif_image,))
        ]


class HeifDecoder(ImageFile.PyDecoder):

    _pulls_fd = True

    def decode(self, buffer):
        raw_heif_image: RawHeifImage = self.args[0]
        mode = raw_heif_image.mode
        raw_decoder = Image._getdecoder(mode, 'raw', (mode, raw_heif_image.stride))
        raw_decoder.setimage(self.im)
        return raw_decoder.decode(raw_heif_image.data)


def register_heif_opener():
    Image.register_open(HeifImageFile.format, HeifImageFile, RawHeifImage.check_file_type)
    Image.register_decoder('heif', HeifDecoder)
    Image.register_extensions(HeifImageFile.format, ['.heic', '.heif'])
    Image.register_mime(HeifImageFile.format, 'image/heif')
