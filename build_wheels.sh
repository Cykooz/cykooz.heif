#!/bin/bash

TWINE_REPOSITORY=${1:-pypi}
CUR_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "${CUR_DIR}"

U_ID=$(id -u)
G_ID=$(id -g)
OWNER="${U_ID}:${G_ID}"

cd ./docker
docker build -t cykooz_heif_build .

cd "${CUR_DIR}"
docker run -it --rm \
  -v "${CUR_DIR}:/src" \
  -v "${HOME}/.pypirc:/root/.pypirc" \
  cykooz_heif_build \
  /build.sh "${TWINE_REPOSITORY}" "${OWNER}"
