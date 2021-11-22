# Changelog Template

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
with the exception of splitting the changelog into multiple files for readability purposes.

## [Unreleased]

### Added

- Aliases
- Arrays of Unspecified Size
- IR
- Recursion
- Error System and Backtraces
- Re-entrant parsing
- Error recovery
- WASM Parser
- leo `fetch`
- Explicit `const function` definitions
- Countdown loops

### Changed

- Access Calls Refactored
- How built ins are defined
- Global Consts
- Shadowing disallowed on top level types
- Rust 2021 Edition!
- `mut self` has been renamed to `ref-self`
- Removed loose spans from AST snapshots
- Imports now done before ASG
- AST passes now in its own directory

### Fixed

- Allowing annotations that don't exist
- Random duplicate definitions being allowed
- A few invalid proof bugs
- Incorrect field and group values
- Canonicalization Fixes

### Removed

- Removed features.
