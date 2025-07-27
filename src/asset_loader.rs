use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};

use crate::config::Config;

#[derive(Default)]
pub struct ConfigLoader;

impl AssetLoader for ConfigLoader {
    type Asset = Config;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let config: Config = toml::from_slice(&bytes)?;
        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
