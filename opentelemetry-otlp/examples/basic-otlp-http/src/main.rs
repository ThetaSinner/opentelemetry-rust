use once_cell::sync::Lazy;
use ts_opentelemetry_api::global;
use ts_opentelemetry_api::trace::TraceError;
use ts_opentelemetry_api::{
    metrics,
    trace::{TraceContextExt, Tracer},
    Context, Key, KeyValue,
};
use ts_opentelemetry_otlp::WithExportConfig;
use ts_opentelemetry_sdk::metrics as sdkmetrics;
use ts_opentelemetry_sdk::trace as sdktrace;
use std::error::Error;

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    ts_opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            ts_opentelemetry_otlp::new_exporter()
                .http()
                .with_endpoint("http://localhost:4318/v1/traces"),
        )
        .install_batch(ts_opentelemetry_sdk::runtime::Tokio)
}

fn init_metrics() -> metrics::Result<sdkmetrics::MeterProvider> {
    let export_config = ts_opentelemetry_otlp::ExportConfig {
        endpoint: "http://localhost:4318/v1/metrics".to_string(),
        ..ts_opentelemetry_otlp::ExportConfig::default()
    };
    ts_opentelemetry_otlp::new_pipeline()
        .metrics(ts_opentelemetry_sdk::runtime::Tokio)
        .with_exporter(
            ts_opentelemetry_otlp::new_exporter()
                .http()
                .with_export_config(export_config),
        )
        .build()
}

const LEMONS_KEY: Key = Key::from_static_str("ex.com/lemons");
const ANOTHER_KEY: Key = Key::from_static_str("ex.com/another");

static COMMON_ATTRIBUTES: Lazy<[KeyValue; 4]> = Lazy::new(|| {
    [
        LEMONS_KEY.i64(10),
        KeyValue::new("A", "1"),
        KeyValue::new("B", "2"),
        KeyValue::new("C", "3"),
    ]
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let _ = init_tracer()?;
    let meter_provider = init_metrics()?;

    let tracer = global::tracer("ex.com/basic");
    let meter = global::meter("ex.com/basic");

    let histogram = meter.f64_histogram("ex.com.two").init();
    let cx = Context::new();
    histogram.record(&cx, 5.5, COMMON_ATTRIBUTES.as_ref());

    tracer.in_span("operation", |cx| {
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![Key::new("bogons").i64(100)],
        );
        span.set_attribute(ANOTHER_KEY.string("yes"));

        tracer.in_span("Sub operation...", |cx| {
            let span = cx.span();
            span.set_attribute(LEMONS_KEY.string("five"));

            span.add_event("Sub span event", vec![]);
        });
    });

    meter_provider.shutdown()?;
    global::shutdown_tracer_provider();

    Ok(())
}
