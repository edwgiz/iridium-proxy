use time::macros::format_description;
use time::UtcOffset;
use tracing::{debug, Level};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::FmtSubscriber;

pub fn init() {
    let local_time = OffsetTime::new(
        UtcOffset::current_local_offset().unwrap(),
        format_description!("[hour repr:24]:[minute]:[second].[subsecond digits:3]"),
    );

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .with_thread_names(true)
        .with_timer(local_time)
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    debug!("Logging initialized");
}