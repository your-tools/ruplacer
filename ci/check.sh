set -x
set -e


main() {
  rustup component add clippy
  cargo clippy --all-targets -- --deny warnings

  cargo build --release

  rustup component add rustfmt
  cargo fmt -- --check

  if [[ "${TRAVIS_RUST_VERSION}" == "nightly" ]] && [[ "${TRAVIS_OS_NAME}" == "linux" ]]; then
    RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
    cargo tarpaulin --out Xml
    bash <(curl -s https://codecov.io/bash)
  else
    cargo test --release
  fi
}

main
