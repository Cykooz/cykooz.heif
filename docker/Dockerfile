# syntax=docker/dockerfile:1.4
FROM quay.io/pypa/manylinux_2_24_x86_64

###############
# Build tools #
###############

# setup
RUN set -ex \
    && echo "deb http://deb.debian.org/debian stretch-backports main" > /etc/apt/sources.list.d/backports.list \
    && apt-get update
RUN set -ex \
    && apt-get install wget git perl make pkg-config autoconf automake \
    m4 libtool cmake libssl-dev libjpeg-dev libpng-dev meson/stretch-backports -y \
    && mkdir /build-tools

# nasm
RUN set -ex \
    && cd /build-tools \
    && NASM_VERSION="2.15.05" \
    && wget --no-check-certificate https://www.nasm.us/pub/nasm/releasebuilds/${NASM_VERSION}/nasm-${NASM_VERSION}.tar.gz \
    && tar xvf nasm-${NASM_VERSION}.tar.gz \
    && cd nasm-${NASM_VERSION} \
    && ./configure \
    && make -j4 \
    && make install \
    && nasm --version

# cleanup
RUN set -ex \
    && cd / \
    && rm -rf /build-tools

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
    && LIBDE265_VERSION="1.0.8" \
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
    && curl  https://sh.rustup.rs -sSf | bash -s -- -y

RUN set -ex && source $HOME/.cargo/env \
    && cargo search --quiet --limit 1 a > /dev/null \
    && cargo install --force cbindgen

# libheif: step1 - rav1e
ENV LIBHEIF_VERSION 1.12.0
RUN set -ex \
    && cd /build-deps \
    && wget https://github.com/strukturag/libheif/releases/download/v${LIBHEIF_VERSION}/libheif-${LIBHEIF_VERSION}.tar.gz \
    && tar xvf libheif-${LIBHEIF_VERSION}.tar.gz \
    && cd libheif-${LIBHEIF_VERSION} \
    && source $HOME/.cargo/env \
    && mkdir -p third-party \
    && cd third-party \
    && wget https://raw.githubusercontent.com/strukturag/libheif/v${LIBHEIF_VERSION}/third-party/rav1e.cmd \
    && /bin/bash rav1e.cmd \
    && cp rav1e/dist/lib/pkgconfig/* ${PKG_CONFIG_PATH}

# libheif: step2 - dav1d
RUN set -ex \
    && cd /build-deps/libheif-${LIBHEIF_VERSION}/third-party \
    && wget https://raw.githubusercontent.com/strukturag/libheif/v${LIBHEIF_VERSION}/third-party/dav1d.cmd \
    && /bin/bash dav1d.cmd \
    && cp dav1d/dist/lib/pkgconfig/* ${PKG_CONFIG_PATH}

# libheif: step3 - libheif
RUN set -ex \
    && cd /build-deps/libheif-${LIBHEIF_VERSION} \
    && mkdir build \
    && cd build \
    && cmake --install-prefix /usr .. \
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
    && declare -a pythons=("cp37-cp37m" "cp38-cp38" "cp39-cp39" "cp310-cp310" "cp311-cp311") \
    && for PY in "${pythons[@]}"; do \
        cd "/opt/python/${PY}/bin/" \
        && ./pip install --upgrade pip auditwheel twine setuptools maturin \
        ; \
    done

ADD build.sh /
