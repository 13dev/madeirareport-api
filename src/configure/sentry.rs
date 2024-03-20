use sentry::ClientInitGuard;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SentryConfig {
  key: String,
  traces_sample_rate: f32
}

pub fn init(config: &SentryConfig) -> ClientInitGuard {
  sentry::init((
    config.key.clone(),
    sentry::ClientOptions {
      release: sentry::release_name!(),
      traces_sample_rate: config.traces_sample_rate,
      ..Default::default()
    },
  ))
}
