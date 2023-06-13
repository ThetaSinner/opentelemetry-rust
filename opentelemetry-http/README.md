![OpenTelemetry — An observability framework for cloud-native software.][splash]

[splash]: https://raw.githubusercontent.com/open-telemetry/opentelemetry-rust/main/assets/logo-text.png

# OpenTelemetry HTTP

A fork of the Rust [OpenTelemetry](https://opentelemetry.io/) implementation.

[![LICENSE](https://img.shields.io/crates/l/opentelemetry)](./LICENSE)

## Overview

[`OpenTelemetry`] is a collection of tools, APIs, and SDKs used to instrument,
generate, collect, and export telemetry data (metrics, logs, and traces) for
analysis in order to understand your software's performance and behavior. This
crate provides a HTTP client interface for use by trace exporters, as well as
helper types to inject and extract key value pairs into/from HTTP headers.

*Compiler support: [requires `rustc` 1.60+][msrv]*

[`OpenTelemetry`]: https://crates.io/crates/opentelemetry
[msrv]: #supported-rust-versions

## Supported Rust Versions

OpenTelemetry is built against the latest stable release. The minimum supported
version is 1.60. The current OpenTelemetry version is not guaranteed to build
on Rust versions earlier than the minimum supported version.

The current stable Rust compiler and the three most recent minor versions
before it will always be supported. For example, if the current stable compiler
version is 1.49, the minimum supported version will not be increased past 1.46,
three minor versions prior. Increasing the minimum supported compiler version
is not considered a semver breaking change as long as doing so complies with
this policy.
