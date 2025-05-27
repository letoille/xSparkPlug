use tracing::Level;
use tracing::{info, level_filters::LevelFilter};
use tracing_appender;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::{Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let filter = Targets::new()
        .with_target("xSparkplug", Level::INFO)
        .with_target("application", Level::INFO)
        .with_target("edge", Level::INFO);

    let file_appender = tracing_appender::rolling::daily("logs", "xsp.log");
    let (nb, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(nb)
        .with_filter(filter.clone())
        .with_filter(LevelFilter::INFO);

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(filter.clone())
        .with_filter(LevelFilter::INFO);

    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    info!("Hello, xspb!");
}
