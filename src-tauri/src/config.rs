#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters, MutGetters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ApplicationConfig {
    debug: bool,
    //server 配置
    server: ServerConfig,
}
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ServerConfig {
    ///当前服务地址
    host: String,
    port: String,
}

impl ApplicationConfig {
    pub fn new(yml_data: &str) -> Self {
        let config = match serde_yaml::from_str(yml_data) {
            Ok(e) => e,
            Err(e) => panic!("{}", e),
        };
        config
    }
}
