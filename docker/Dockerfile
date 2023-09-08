# syntax=docker/dockerfile:1.5
FROM quay.io/pypa/manylinux_2_28_x86_64

###############
# Build tools #
###############

# setup
RUN set -ex \
    && dnf install -y wget openssl-devel libjpeg-turbo-devel libpng-devel \
    meson nasm

################
# Dependencies #
################

# setup
RUN set -ex \
    && mkdir /build-deps

# x265
RUN set -ex \
    && cd /build-deps \
    && X265_VERSION="3.5" \
    && wget https://bitbucket.org/multicoreware/x265_git/downloads/x265_${X265_VERSION}.tar.gz \
    && tar xvf x265_${X265_VERSION}.tar.gz \
    && cd x265_${X265_VERSION} \
    && cmake --install-prefix /usr -G "Unix Makefiles" ./source \
    && make -j4 \
    && make install/strip \
    && ldconfig

# libde265
RUN set -ex \
    && cd /build-deps \
    && LIBDE265_VERSION="1.0.12" \
    && wget https://github.com/strukturag/libde265/releases/download/v${LIBDE265_VERSION}/libde265-${LIBDE265_VERSION}.tar.gz \
    && tar xvf libde265-${LIBDE265_VERSION}.tar.gz \
    && cd libde265-${LIBDE265_VERSION} \
    && mkdir build \
    && cd build \
    && cmake .. \
    && make -j4 \
    && make install/strip \
    && ldconfig

# Rust
SHELL ["/bin/bash", "-c"]

RUN set -ex \
    && cd /build-deps \
    && curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN set -ex && source $HOME/.cargo/env \
    && cargo search --quiet --limit 1 a > /dev/null \
    && cargo install --force cbindgen

# libheif: step1 - rav1e
ENV LIBHEIF_VERSION 1.16.2
RUN set -ex \
    && cd /build-deps \
    && wget https://github.com/strukturag/libheif/releases/download/v${LIBHEIF_VERSION}/libheif-${LIBHEIF_VERSION}.tar.gz \
    && tar xvf libheif-${LIBHEIF_VERSION}.tar.gz \
    && cd libheif-${LIBHEIF_VERSION} \
    && mkdir -p third-party \
    && cd third-party \
    && git clone -b 0.6 --depth 1 https://github.com/xiph/rav1e.git \
    && git clone -b 1.2.1 --depth 1 https://code.videolan.org/videolan/dav1d.git \
    && git clone -b v1.3.1 --depth 1 https://chromium.googlesource.com/webm/libwebp

RUN set -ex \
    && source $HOME/.cargo/env \
    && cd /build-deps/libheif-${LIBHEIF_VERSION}/third-party/rav1e \
    && cargo install cargo-c \
    && cargo cinstall --crt-static --release --prefix="$(pwd)/dist" --library-type=staticlib \
    && cp ./dist/lib/pkgconfig/* ${PKG_CONFIG_PATH}

# libheif: step2 - dav1d
RUN set -ex \
    && cd /build-deps/libheif-${LIBHEIF_VERSION}/third-party/dav1d \
    && meson build --default-library=static --buildtype release --prefix "$(pwd)/dist" $@ \
    && ninja -C build \
    && ninja -C build install \
    && cp dist/lib64/pkgconfig/* ${PKG_CONFIG_PATH}

# libheif: step3 - libsharpyuv
RUN set -ex \
    && cd /build-deps/libheif-${LIBHEIF_VERSION}/third-party/libwebp \
    && mkdir build \
    && cd build \
    && cmake -G Ninja -DCMAKE_INSTALL_PREFIX="$(pwd)/dist" -DBUILD_SHARED_LIBS=OFF -DCMAKE_BUILD_TYPE=Release .. \
    && ninja sharpyuv \
    && ninja install \
    && cp dist/lib64/pkgconfig/* ${PKG_CONFIG_PATH}

# libheif: step4 - libheif
RUN set -ex \
    && cd /build-deps/libheif-${LIBHEIF_VERSION} \
    && mkdir build \
    && cd build \
    && cmake --preset=release --install-prefix /usr .. \
    && make -j4 \
    && make install/strip \
    && ldconfig

##########################
# Build manylinux wheels #
##########################

# setup
ENV CARGO_TARGET_DIR /cargo_target
RUN mkdir /workdir "${CARGO_TARGET_DIR}"

RUN set -ex \
    && source $HOME/.cargo/env \
    && declare -a pythons=("cp38-cp38" "cp39-cp39" "cp310-cp310" "cp311-cp311") \
    && for PY in "${pythons[@]}"; do \
        cd "/opt/python/${PY}/bin/" \
        && ./pip install --upgrade pip auditwheel twine setuptools maturin \
        ; \
    done

ADD build.sh /
