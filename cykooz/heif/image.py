# -*- coding: utf-8 -*-
"""
:Authors: cykooz
:Date: 25.06.2019
"""
import os
from typing import Optional, BinaryIO

from cykooz.heif.rust2py import open_heif_from_path, open_heif_from_reader, HeifImage as _RustHeifImage

from .errors import HeifError
from .typing import PathLike


class HeifImage:

    def __init__(self, image: _RustHeifImage):
        self._image = image
        self._exif = None
        self._is_exif_loaded = False
        self._data = None
        self._stride = None
        self._bits_per_pixel = None
        self._is_data_loaded = False

    @classmethod
    def from_path(cls, path: PathLike):
        try:
            image: _RustHeifImage = open_heif_from_path(os.fspath(path))
        except RuntimeError as e:
            raise HeifError(*e.args)
        return cls(image)

    @classmethod
    def from_stream(cls, stream: BinaryIO, total_size: int = None):
        if total_size is None:
            total_size: int = stream.seek(0, os.SEEK_END)
            stream.seek(0, os.SEEK_SET)
        try:
            image: _RustHeifImage = open_heif_from_reader(stream, total_size)
        except RuntimeError as e:
            raise HeifError(*e.args)
        return cls(image)

    @property
    def width(self) -> int:
        return self._image.width

    @property
    def height(self) -> int:
        return self._image.height

    @property
    def mode(self) -> str:
        return self._image.mode

    @property
    def exif(self) -> Optional[bytes]:
        if not self._is_exif_loaded:
            try:
                self._exif = self._image.get_exif()
            except RuntimeError as e:
                raise HeifError(*e.args)
            self._is_exif_loaded = True
        return self._exif

    def _load_plane(self):
        if self._is_data_loaded:
            return
        try:
            self._data, self._stride, self._bits_per_pixel = self._image.get_data()
        except RuntimeError as e:
            raise HeifError(*e.args)
        self._is_data_loaded = True

    @property
    def data(self) -> Optional[bytes]:
        self._load_plane()
        return self._data

    @property
    def stride(self) -> Optional[int]:
        self._load_plane()
        return self._stride

    @property
    def bits_per_pixel(self) -> Optional[int]:
        self._load_plane()
        return self._bits_per_pixel
