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
cp -rf /src "${WORKDIR}/cykooz-heif"
cd "${WORKDIR}/cykooz-heif"
find . -name ".*" -not -path '.' -not -path '..' -exec rm -rf {} +
find . -name "*.pyc" -delete
find . -name "__pycache__" -exec rm -rf {} +
rm -rf docker tests build_wheels.sh


source "$HOME/.cargo/env"
PYTHONS=("9" "10" "11" "12" "13")
mkdir "${RESULT}/repaired"
SDIST_OPT="--sdist"
for PY_MINOR in "${PYTHONS[@]}"; do
  PY="3${PY_MINOR}"
  echo ""
  echo "Build wheel for Python 3.${PY_MINOR}"
  PY_BIN_DIR="/opt/python/cp${PY}-cp${PY}/bin"
  cd "${WORKDIR}/cykooz-heif"
  mkdir "${RESULT}/wheelhouse${PY}"
  PYTHON_SYS_EXECUTABLE="${PY_BIN_DIR}/python" "${PY_BIN_DIR}/maturin" build \
    ${SDIST_OPT} \
    --release --strip \
    --compatibility manylinux_2_28 \
    --skip-auditwheel \
    -i "python3.${PY_MINOR}" \
    -o "${RESULTDIR}/wheelhouse${PY}/"
  "${PY_BIN_DIR}/auditwheel" repair ${RESULTDIR}/wheelhouse${PY}/cykooz.heif*.whl \
    --plat manylinux_2_28_x86_64 \
    -w "${RESULTDIR}/repaired"
  if [[ -n "$SDIST_OPT" ]]; then
    cp ${RESULTDIR}/wheelhouse${PY}/cykooz.heif*.tar.gz "${RESULTDIR}/repaired/"
    SDIST_OPT=""
  fi
  find /cargo_target/release/build/ -maxdepth 1 -name "pyo3*" -type d -print0 | xargs -0 rm -r
done

cd "${PY_BIN_DIR}"
TWINE_REPOSITORY=${TWINE_REPOSITORY} ./twine upload ${RESULTDIR}/repaired/*
