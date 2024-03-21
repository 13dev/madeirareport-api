use std::sync::Arc;
use tokio::sync::Notify;
use tracing::info;

use crate::client::{
  database::{DatabaseClient, DatabaseClientExt},
  email::EmailClient,
  http::HttpClient,
  redis::RedisClient,
  ClientBuilder,
};
use crate::configure::AppConfig;
use crate::error::AppResult;

#[derive(Clone)]
pub struct AppState {
  pub config: Arc<AppConfig>,
  pub redis: Arc<RedisClient>,
  pub db: Arc<DatabaseClient>,
  pub email: Arc<EmailClient>,
  pub messenger_notify: Arc<Notify>,
  pub http: HttpClient,
}

impl AppState {
  pub async fn new(config: AppConfig) -> AppResult<Self> {
    info!("Connecting to redis: {}!", &config.redis.get_url());
    let redis = Arc::new(RedisClient::build_from_config(&config)?);

    info!("Connecting to email: {}!", &config.email.get_addr());
    let email = Arc::new(EmailClient::build_from_config(&config)?);
    
    info!("Connecting to db: {}!", &config.db.get_url());
    let db = Arc::new(DatabaseClient::build_from_config(&config).await?);
    
    let http = HttpClient::build_from_config(&config)?;
    Ok(Self {
      config: Arc::new(config),
      db,
      redis,
      email,
      messenger_notify: Default::default(),
      http,
    })
  }
}
