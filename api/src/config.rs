use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ConfigEnvironment {
    #[serde(default="default_host")]
    pub host: String,
    #[serde(default="default_port")]
    pub port: u16,
    pub database_url: String,
    pub secret: String,
    pub salt: String,
    pub bucket_name: String,
    pub cdn_url: String,
}

impl ConfigEnvironment {
    pub fn from_env() -> Self {
        envy::from_env::<Self>().ok().unwrap()
    }
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    4000
}
