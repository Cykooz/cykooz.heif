import os
import sys

from setuptools import find_packages, setup

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print('Please install setuptools-rust package')
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension

HERE = os.path.abspath(os.path.dirname(__file__))
sys.path.append(HERE)

import version


README = open(os.path.join(HERE, 'README.rst'), 'rt').read()
CHANGES = open(os.path.join(HERE, 'CHANGES.rst'), 'rt').read()

setup(
    name='cykooz.heif',
    version=version.get_version(),
    description='A decoder of HEIF format of images',
    long_description=README + '\n\n' + CHANGES,
    long_description_content_type='text/x-rst',
    classifiers=[
        'Development Status :: 4 - Beta',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: Implementation :: CPython',
        'Topic :: Multimedia :: Graphics',
        'Topic :: Multimedia :: Graphics :: Graphics Conversion',
        'License :: OSI Approved :: MIT License',
    ],
    keywords=['heif', 'heic', 'libheif', 'pil', 'pillow'],
    author='Kirill Kuzminykh',
    author_email='cykooz@gmail.com',
    url='https://github.com/Cykooz/cykooz.heif',
    package_dir={'': '.'},
    packages=find_packages(),
    include_package_data=True,
    package_data={},
    rust_extensions=[
        RustExtension('cykooz.heif.rust_heif')
    ],
    zip_safe=False,
    extras_require={
        'test': [
            'pytest',
            'Pillow',
            'piexif',
        ],
    },
    setup_requires=[
        'setuptools-rust>=0.10.6',
        'wheel',
    ],
    install_requires=[
    ],
)
