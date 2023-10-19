use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our config reader
    let mut settings = Config::default();

    // Add configuration values from a file named "configuration".
    settings.merge(File::with_name("configuration"))?;

    // Try to convert the configuration values into Settings type
    settings.try_into()
}
