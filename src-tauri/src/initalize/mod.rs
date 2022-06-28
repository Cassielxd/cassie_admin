use crate::{config::ApplicationConfig, APPLICATION_CONTEXT};

pub async fn init_config() {
    let content = include_str!("../application.yml");
    let config = ApplicationConfig::new(content);
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}
