use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::env::var;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub profiles: HashMap<String, Profile>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = var("XDG_CONFIG_HOME")
            .or_else(|_| var("HOME").map(|home| format!("{}/.config/heimdallr.toml", home)))
            .unwrap();

        let settings = Config::builder()
            .add_source(File::with_name(config_path.as_str()))
            .build()
            .unwrap();

        settings.try_deserialize()
    }
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    /// Profile name as specified in your ~/.aws/credentials file
    pub aws_profile: String,

    /// AWS region servers exist in
    pub aws_region: String,

    /// The security group id that controls ingress to the bastion server
    pub security_group_id: String,

    /// The host name of the bastion server
    pub dns_name: String,

    /// The ssh port of the bastion server
    pub bastion_port: u16,

    /// The ssh user of the bastion server
    pub bastion_user: String,

    /// The user of the ec2 server
    pub ec2_user: String,

    /// The ssh identity file to use
    pub identity_file: String,
}
