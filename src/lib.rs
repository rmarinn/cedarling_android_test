use cedarling::{self as core, BootstrapConfig, BootstrapConfigRaw, Request};
//use std::fmt::Error;
uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct Cedarling {
    cedarling: core::Cedarling,
}

impl Cedarling {
    #[uniffi::constructor]
    pub fn new(config: String) -> Result<Self, String> {
        let config = serde_json::from_str::<BootstrapConfigRaw>(config.clone().as_str()).unwrap();
        let config = BootstrapConfig::from_raw_config(&config).map_err(|e| e.to_string())?;
        let cedarling = core::Cedarling::new(&config).map_err(|e| e.to_string())?;

        Ok(Self { cedarling })
    }
}