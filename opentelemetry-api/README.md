![OpenTelemetry — An observability framework for cloud-native software.][splash]

[splash]: https://raw.githubusercontent.com/open-telemetry/opentelemetry-rust/main/assets/logo-text.png

# OpenTelemetry Rust API

The Rust [OpenTelemetry](https://opentelemetry.io/) implementation.

[![Crates.io: opentelemetry-api](https://img.shields.io/crates/v/opentelemetry_api.svg)](https://crates.io/crates/ts_opentelemetry_api)
[![Documentation](https://docs.rs/opentelemetry_api/badge.svg)](https://docs.rs/ts_opentelemetry_api)
[![LICENSE](https://img.shields.io/crates/l/opentelemetry_api)](./LICENSE)

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
