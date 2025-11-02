# http

An async HTTP client library for Rust, forked from [isahc](https://github.com/sagebind/isahc).

## Overview

This crate is a fork of the isahc HTTP client library, adapted for Bridge's specific needs. The fork has been streamlined by removing unused functionality, upgrading to the latest dependencies, and making modifications to better integrate with Bridge's architecture.

## Key Changes from Original isahc

- Removed unused features and dependencies
- Upgraded all dependencies to their latest versions
- Adapted APIs and internals to align with Bridge's requirements
- Maintained compatibility with the curl backend for robust HTTP operations

## Future Plans

We are planning to migrate the implementation from curl to [hyper](https://github.com/hyperium/hyper) and refine the APIs to be more idiomatic and aligned with modern Rust async ecosystem practices.

## License

MIT
