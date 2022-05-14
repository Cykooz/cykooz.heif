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

source "$HOME/.cargo/env"
PYTHONS=("37" "38" "39" "310")
for PY in "${PYTHONS[@]}"; do
  PY_BIN_DIR="/opt/python/cp${PY}-cp${PY}m/bin/"
  if [ ! -d "${PY_BIN_DIR}" ]; then
    PY_BIN_DIR="/opt/python/cp${PY}-cp${PY}/bin/"
  fi;
  cd "${PY_BIN_DIR}"
  mkdir "${RESULT}/wheelhouse${PY}" "${RESULT}/repaired${PY}"
  PYTHON_SYS_EXECUTABLE="${PY_BIN_DIR}/python" ./pip wheel "${WORKDIR}/cykooz.heif" \
    -w "${RESULTDIR}/wheelhouse${PY}/"
  ./auditwheel repair ${RESULTDIR}/wheelhouse${PY}/cykooz.heif*.whl \
    --plat manylinux_2_24_x86_64 \
    -w "${RESULTDIR}/repaired${PY}"
  find /cargo_target/release/build/ -maxdepth 1 -name "pyo3*" -type d -print0 | xargs -0 rm -r
  TWINE_REPOSITORY=${TWINE_REPOSITORY} ./twine upload ${RESULTDIR}/repaired${PY}/cykooz.heif*manylinux2014*.whl
done
