use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{Registry};
use tracing_error::{ErrorLayer, SpanTrace};
use tracing_subscriber::layer::SubscriberExt;
use opentelemetry::api::{Provider, Tracer};
use opentelemetry::{sdk, global};
use opentelemetry::sdk::{Sampler};

pub fn get_tracer() -> thrift::Result<sdk::Tracer> {
    let exporter = opentelemetry_jaeger::Exporter::builder()
        .with_agent_endpoint("127.0.0.1:6831".parse().unwrap())
        .with_process(opentelemetry_jaeger::Process {
            service_name: "trace-demo".to_string(),
            tags: vec![
            ],
        })
        .init()?;
    let provider = sdk::Provider::builder()
        .with_simple_exporter(exporter)
        .with_config(sdk::Config {
            default_sampler: Box::new(sdk::Sampler::Always),
            ..Default::default()
        })
        .build();

    Ok(provider.get_tracer("test"))
}

fn init_telemetry() {
    let tracer =
        get_tracer().expect("Failed to build tracer.");
    let opentracing_layer = OpenTelemetryLayer::new(tracer, Sampler::Always);

    let subscriber =
        Registry::default()
            .with(opentracing_layer)
            .with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).unwrap()
}

#[derive(Debug)]
pub struct Dummy(SpanTrace);

#[tracing::instrument]
pub fn test() {
    let culprit = Dummy(SpanTrace::capture());
    tracing::warn!("It goes boom! - {:?}", culprit);
}

fn main() {
    init_telemetry();
    let tracer = global::tracer("test");
    tracer.in_span("span", move |_cx| {
        let _ = test();
    });
}