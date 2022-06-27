use async_std::fs::read_to_string;

use crate::{config::ApplicationConfig, APPLICATION_CONTEXT};

pub async fn init_config() {
    let content = read_to_string("application.yml").await.unwrap();
    let config = ApplicationConfig::new(content.as_str());
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}
