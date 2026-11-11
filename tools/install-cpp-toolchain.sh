#!/usr/bin/env bash
# Installs the pinned C++ toolchain (docs/toolchains.md) on Ubuntu 24.04.
# Single source of truth for local machines and CI; bump pins here only.
set -euo pipefail

readonly UBUNTU_VERSION="24.04"
readonly LLVM_MAJOR="20"
readonly MULL_VERSION="0.37.0"
readonly MULL_LLVM_VERSION="20.1.2"
readonly MULL_URL="https://github.com/mull-project/mull/releases/download/${MULL_VERSION}"

check_os() {
  # shellcheck source=/dev/null
  . /etc/os-release
  if [[ "${ID}" != "ubuntu" || "${VERSION_ID}" != "${UBUNTU_VERSION}" ]]; then
    echo "error: expected Ubuntu ${UBUNTU_VERSION}, found ${ID} ${VERSION_ID}" >&2
    exit 1
  fi
}

install_apt_packages() {
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends \
    g++-13 cmake gcovr \
    "clang-${LLVM_MAJOR}" "clang-tidy-${LLVM_MAJOR}" \
    "clang-format-${LLVM_MAJOR}" "clangd-${LLVM_MAJOR}"
}

install_mull() {
  local download_dir
  download_dir="$(mktemp -d)"
  curl -fsSL -o "${download_dir}/mull.deb" \
    "${MULL_URL}/Mull-${MULL_VERSION}-ubuntu-amd64-${UBUNTU_VERSION}.deb"
  curl -fsSL -o "${download_dir}/mull-${LLVM_MAJOR}.deb" \
    "${MULL_URL}/Mull-${LLVM_MAJOR}-${MULL_VERSION}-LLVM-${MULL_LLVM_VERSION}-ubuntu-amd64-${UBUNTU_VERSION}.deb"
  # Both in one transaction: mull-<N> depends on mull (= MULL_VERSION).
  sudo apt-get install -y "${download_dir}/mull.deb" "${download_dir}/mull-${LLVM_MAJOR}.deb"
  rm -rf "${download_dir}"
}

print_versions() {
  "clang++-${LLVM_MAJOR}" --version | head -n1
  "clang-tidy-${LLVM_MAJOR}" --version | grep -m1 version
  "clang-format-${LLVM_MAJOR}" --version
  "mull-runner-${LLVM_MAJOR}" --version | head -n1
}

check_os
install_apt_packages
install_mull
print_versions
