![OpenTelemetry — An observability framework for cloud-native software.][splash]

[splash]: https://raw.githubusercontent.com/open-telemetry/opentelemetry-rust/master/assets/logo-text.png

# OpenTelemetry ZPages

ZPages server written in Rust

## Overview

zPages are an in-process alternative to external exporters. When included, they collect and aggregate tracing and metrics information in the background; this data is served on web pages or APIs when requested.

This crate is still working in progress. Please find its current limitations below.

Note that this crate is still in **experimental** state. Breaking changes can still happen. Some features may still in development.

## Tracez

Tracez shows information on tracing, including aggregation counts for latency, running, and errors for spans grouped by the span name.

