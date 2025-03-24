use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub use self::package_config::PackageConfig;

mod package_config;

#[derive(Debug, Default, Clone)]
pub struct ConfigsRegistry {
    packages_configs: HashMap<PathBuf, PackageConfig>,
}

impl ConfigsRegistry {
    pub fn config_for_file(&self, path: &Path) -> Option<PackageConfig> {
        self.packages_configs.iter().find_map(|(manifest_path, config)| {
            let mut manifest_dir = (*manifest_path).to_owned();

            // Should be always true but better safe than sorry.
            if manifest_dir.ends_with("Scarb.toml") {
                manifest_dir.pop();
            }

            path.starts_with(manifest_dir).then(|| config.clone())
        })
    }

    pub fn clear(&mut self) {
        self.packages_configs.clear();
    }

    pub fn insert(&mut self, manifest_path: PathBuf, config: PackageConfig) {
        self.packages_configs.insert(manifest_path, config);
    }

    pub fn manifests_dirs(&self) -> impl Iterator<Item = PathBuf> {
        self.packages_configs.keys().map(|manifest_path| {
            let mut manifest_dir = (*manifest_path).to_owned();

            // Should be always true but better safe than sorry.
            if manifest_dir.ends_with("Scarb.toml") {
                manifest_dir.pop();
            }

            manifest_dir
        })
    }
}
