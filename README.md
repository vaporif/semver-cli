[<img alt="crates.io" src="https://img.shields.io/crates/v/semver-cli-check.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/semver-cli-check)
[![fmt/clippy/build](https://github.com/vaporif/semver-cli-check/actions/workflows/ci.yml/badge.svg)](https://github.com/vaporif/semver-cli-check/actions/workflows/ci.yml)

Small semver version checker cli tool

```
$ semver-cli-check
```

Usage: semver-cli-check -r <REQUESTED_VERSION> <VERSION_TO_TEST>

Arguments:
  <VERSION_TO_TEST>  SemVer to ckeck

Options:
  -r <REQUESTED_VERSION>      SemVer requirement describing the intersection of some version comparators, such as >=1.2.3, <1.8.
  -h, --help                  Print help
