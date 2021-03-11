use anyhow::Result;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::infra::cli::CLIOpts;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub ldap_port: u16,
    pub ldaps_port: u16,
    pub http_port: u16,
    pub secret_pepper: String,
    pub admin_dn: String,
    pub admin_password: String,
    pub verbose: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            ldap_port: 3890,
            ldaps_port: 6360,
            http_port: 17170,
            secret_pepper: String::from("secretsecretpepper"),
            admin_dn: String::new(),
            admin_password: String::new(),
            verbose: false,
        }
    }
}

impl Configuration {
    fn merge_with_cli(mut self: Configuration, cli_opts: CLIOpts) -> Configuration {
        if cli_opts.verbose {
            self.verbose = true;
        }

        if let Some(port) = cli_opts.ldap_port {
            self.ldap_port = port;
        }

        if let Some(port) = cli_opts.ldaps_port {
            self.ldaps_port = port;
        }

        self
    }
}

pub fn init(cli_opts: CLIOpts) -> Result<Configuration> {
    let config_file = cli_opts.config_file.clone();

    let config: Configuration = Figment::from(Serialized::defaults(Configuration::default()))
        .merge(Toml::file(config_file))
        .merge(Env::prefixed("LLDAP_"))
        .extract()?;

    let config = config.merge_with_cli(cli_opts);
    Ok(config)
}
