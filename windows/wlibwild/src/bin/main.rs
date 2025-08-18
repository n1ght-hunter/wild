use tracing_subscriber::{
    Layer as _, fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use wlibwild::run;

fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_file(true)
                .with_span_events(FmtSpan::CLOSE)
                .with_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or("wlibwild=info,warn".into()),
                ),
        )
        .init();
    run();
}
