# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Do not link to non-working docs.rs documentation. This is causes by the
  rust-skia dependency: <https://github.com/rust-skia/rust-skia/issues/720>

## [0.3.0] - 2024-04-02

### Fixed

- Add `doc-scrape-examples = true` to Cargo.toml so that docs.rs can compile the
  documentation.

## [0.2.0] - 2024-04-01

### Changed

- Use egui 0.26.0 as minimal version

## [0.1.0] - 2024-04-01

Initial release