set -x
set -e


main() {
  if [[ "${TRAVIS_RUST_VERSION}" == "stable" ]] && [[ "${TRAVIS_OS_NAME}" == "linux" ]]; then
    # No need to run clippy or fmt --check more than once
    rustup component add clippy
    cargo clippy --all-targets -- --deny warnings

    rustup component add rustfmt
    cargo fmt -- --check
  fi

  cargo build --release

  if [[ "${TRAVIS_RUST_VERSION}" == "nightly" ]] && [[ "${TRAVIS_OS_NAME}" == "linux" ]]; then
    RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
    cargo tarpaulin --out Xml
    bash <(curl -s https://codecov.io/bash)
  else
    cargo test --release
  fi
}

main
