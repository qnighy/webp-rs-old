#!/usr/bin/env bash
set -ue
set -o pipefail

PACKAGES=(webp)

cargo update --verbose

for package in "${PACKAGES[@]}"; do
  cargo build -p $package --examples --verbose
  cargo test -p $package --verbose
done

if [[ ${TRAVIS_RUST_VERSION:-} = stable ]]; then
  cargo fmt --all -- --write-mode check
fi
