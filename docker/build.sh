#!/bin/bash

set -e
WORKDIR="/workdir"
RESULTDIR="/src/wheels"
TWINE_REPOSITORY=${1:-pypi}
RES_OWNER=${2:-}

exit_handler() {
  if [[ -n "$RES_OWNER" ]]; then
    echo
    echo 'Change owner for result and cache'
    echo "================================="
    chown -R "${RES_OWNER}" ${RESULTDIR}
  fi
}
trap exit_handler EXIT

mkdir -p "${RESULTDIR}"
rm -rf ${RESULTDIR}/*
rm -rf ${WORKDIR}/*
cp -rf /src "${WORKDIR}/cykooz.heif"
cd "${WORKDIR}/cykooz.heif"
find . -name ".*" -not -path '.' -not -path '..' -exec rm -rf {} +
find . -name "*.pyc" -delete
find . -name "__pycache__" -exec rm -rf {} +
rm -rf docker tests bootstrap.py build_wheels.sh


source "$HOME/.cargo/env"
PYTHONS=("7" "8" "9" "10")
for PY_MINOR in "${PYTHONS[@]}"; do
  PY="3${PY_MINOR}"
  echo ""
  echo "Build wheel for Python 3.${PY_MINOR}"
  PY_BIN_DIR="/opt/python/cp${PY}-cp${PY}m/bin/"
  if [ ! -d "${PY_BIN_DIR}" ]; then
    PY_BIN_DIR="/opt/python/cp${PY}-cp${PY}/bin/"
  fi;
  cd "${WORKDIR}/cykooz.heif"
  PYTHON_SYS_EXECUTABLE="${PY_BIN_DIR}/python" "${PY_BIN_DIR}/maturin" build \
    --release --strip \
    --no-sdist \
    --compatibility manylinux_2_24 \
    -i "python3.${PY_MINOR}" \
    -o "${RESULTDIR}/"
  find /cargo_target/release/build/ -maxdepth 1 -name "pyo3*" -type d -print0 | xargs -0 rm -r
done

cd "${WORKDIR}/cykooz.heif"
find . -name "*.so" -delete
"${PY_BIN_DIR}/maturin" sdist -o "${RESULTDIR}/"
cd "${PY_BIN_DIR}"
TWINE_REPOSITORY=${TWINE_REPOSITORY} ./twine upload ${RESULTDIR}/*
