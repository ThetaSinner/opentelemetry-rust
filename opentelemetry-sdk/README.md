![OpenTelemetry — An observability framework for cloud-native software.][splash]

[splash]: https://raw.githubusercontent.com/open-telemetry/opentelemetry-rust/main/assets/logo-text.png

# OpenTelemetry Rust SDK

A fork of the Rust [OpenTelemetry](https://opentelemetry.io/) implementation.

[![Crates.io: opentelemetry-sdk](https://img.shields.io/crates/v/opentelemetry_sdk.svg)](https://crates.io/crates/ts_opentelemetry_sdk)
[![Documentation](https://docs.rs/opentelemetry_sdk/badge.svg)](https://docs.rs/ts_opentelemetry_sdk)
[![LICENSE](https://img.shields.io/crates/l/opentelemetry_sdk)](./LICENSE)

## Overview

OpenTelemetry is a collection of tools, APIs, and SDKs used to instrument,
generate, collect, and export telemetry data (metrics, logs, and traces) for
analysis in order to understand your software's performance and behavior. You
can export and analyze them using [Prometheus], [Jaeger], and other
observability tools.

*Compiler support: [requires `rustc` 1.60+][msrv]*

[Prometheus]: https://prometheus.io
[Jaeger]: https://www.jaegertracing.io
[msrv]: #supported-rust-versions

## OpenTelemetry Benchmarks

From the root folder, run the following command:

```sh
cargo bench
```
