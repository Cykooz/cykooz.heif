FROM quay.io/pypa/manylinux2014_x86_64

###############
# Build tools #
###############

# setup
RUN set -ex \
    && yum install wget git perl -y \
    && mkdir /build-tools

# pkg-config
RUN set -ex \
    && cd /build-tools \
    && PKG_CONFIG_VERSION="0.29.2" \
    && wget https://pkg-config.freedesktop.org/releases/pkg-config-${PKG_CONFIG_VERSION}.tar.gz \
    && tar xvf pkg-config-${PKG_CONFIG_VERSION}.tar.gz \
    && cd pkg-config-${PKG_CONFIG_VERSION} \
    && ./configure --with-internal-glib \
    && make -j4 \
    && make install \
    && pkg-config --version

# libressl
RUN set -ex \
    && cd /build-tools \
    && LIBRESSL_VERSION="3.0.2" \
    && wget https://github.com/libressl-portable/portable/archive/v${LIBRESSL_VERSION}.tar.gz \
    && tar xvf v${LIBRESSL_VERSION}.tar.gz \
    && cd portable-${LIBRESSL_VERSION} \
    && ./autogen.sh \
    && ./configure \
    && make -j4 \
    && make install

# cmake
RUN set -ex \
    && cd /build-tools \
    && CMAKE_VERSION="3.16.4" \
    && wget https://github.com/Kitware/CMake/releases/download/v${CMAKE_VERSION}/cmake-${CMAKE_VERSION}.tar.gz \
    && tar xvf cmake-${CMAKE_VERSION}.tar.gz \
    && cd cmake-${CMAKE_VERSION} \
    && ./configure \
    && gmake -j4 \
    && make install \
    && cmake --version

# autoconf
RUN set -ex \
    && cd /build-tools \
    && AUTOCONF_VERSION="2.69" \
    && wget https://ftp.gnu.org/gnu/autoconf/autoconf-${AUTOCONF_VERSION}.tar.gz \
    && tar xvf autoconf-${AUTOCONF_VERSION}.tar.gz \
    && cd autoconf-${AUTOCONF_VERSION} \
    && ./configure \
    && make -j4 \
    && make install \
    && autoconf --version

# automake
RUN set -ex \
    && cd /build-tools \
    && AUTOMAKE_VERSION="1.16.1" \
    && wget https://ftp.gnu.org/gnu/automake/automake-${AUTOMAKE_VERSION}.tar.gz \
    && tar xvf automake-${AUTOMAKE_VERSION}.tar.gz \
    && cd automake-${AUTOMAKE_VERSION} \
    && ./configure \
    && make -j4 \
    && make install \
    && automake --version

# nasm
RUN set -ex \
    && cd /build-tools \
    && NASM_VERSION="2.14.02" \
    && wget https://www.nasm.us/pub/nasm/releasebuilds/${NASM_VERSION}/nasm-${NASM_VERSION}.tar.gz \
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
    && X265_VERSION="3.3" \
    && wget https://bitbucket.org/multicoreware/x265_git/downloads/x265_${X265_VERSION}.tar.gz \
    && tar xvf x265_${X265_VERSION}.tar.gz \
    && cd x265_${X265_VERSION} \
    && cmake -DCMAKE_INSTALL_PREFIX=/usr -G "Unix Makefiles" ./source \
    && make -j4 \
    && make install \
    && strip -s /usr/lib/libx265.so \
    && ldconfig

# libde265
RUN set -ex \
    && cd /build-deps \
    && LIBDE265_VERSION="1.0.7" \
    && wget https://github.com/strukturag/libde265/releases/download/v${LIBDE265_VERSION}/libde265-${LIBDE265_VERSION}.tar.gz \
    && tar xvf libde265-${LIBDE265_VERSION}.tar.gz \
    && cd libde265-${LIBDE265_VERSION} \
    && ./autogen.sh \
    && ./configure --prefix /usr \
    && make -j4 \
    && make install-strip \
    && ldconfig

# libaom
RUN set -ex \
    && cd /build-deps \
    && LIBOAM_VERSION="2.0.0" \
    && git clone --depth 1 --branch v${LIBOAM_VERSION} https://aomedia.googlesource.com/aom \
    && mkdir -p ./aom_build \
    && cd aom_build \
    && cmake -DBUILD_SHARED_LIBS=1 -DCMAKE_INSTALL_PREFIX=/usr ../aom \
    && make -j4 \
    && make install \
    && strip -s /usr/lib64/libaom.so \
    && cp /usr/lib64/pkgconfig/aom.pc /usr/lib/pkgconfig/aom.pc \
    && ldconfig

# Rust
RUN set -ex \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN set -ex && source $HOME/.cargo/env \
    && cargo search --quiet --limit 1 a > /dev/null \
    && cargo install --force cbindgen

# libheif: step1 - rav1e
ENV LIBHEIF_VERSION 1.9.1
RUN set -ex \
    && cd /build-deps \
    && wget https://github.com/strukturag/libheif/releases/download/v${LIBHEIF_VERSION}/libheif-${LIBHEIF_VERSION}.tar.gz \
    && tar xvf libheif-${LIBHEIF_VERSION}.tar.gz \
    && cd libheif-${LIBHEIF_VERSION} \
    && source $HOME/.cargo/env \
    && mkdir -p third-party \
    && cd third-party \
    && wget https://raw.githubusercontent.com/strukturag/libheif/master/third-party/rav1e.cmd \
    && /bin/bash rav1e.cmd

# libheif: step2 - dav1d
RUN yum install meson -y
RUN set -ex \
    && cd /build-deps/libheif-${LIBHEIF_VERSION}/third-party \
    && wget https://raw.githubusercontent.com/strukturag/libheif/master/third-party/dav1d.cmd \
    && /bin/bash dav1d.cmd

# libheif: step3 - libheif
RUN set -ex \
    && cd /build-deps/libheif-${LIBHEIF_VERSION} \
    && ./autogen.sh \
    && ./configure --prefix /usr --enable-local-rav1e --enable-local-dav1d \
    && make -j4 \
    && make install-strip \
    && ldconfig

##########################
# Build manylinux wheels #
##########################

# setup
ENV CARGO_TARGET_DIR /cargo_target
RUN mkdir /workdir "${CARGO_TARGET_DIR}"

RUN set -ex \
    && source $HOME/.cargo/env \
    && declare -a pythons=("cp35-cp35m" "cp36-cp36m" "cp37-cp37m" "cp38-cp38") \
    && for PY in "${pythons[@]}"; do \
        cd "/opt/python/${PY}/bin/" \
        && ./pip install --upgrade pip auditwheel twine setuptools setuptools-rust \
        ; \
    done

ADD build.sh /
