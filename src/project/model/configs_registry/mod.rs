use std::collections::HashMap;
use std::path::{Path, PathBuf};

use itertools::Itertools;

pub use self::package_config::PackageConfig;

mod package_config;

#[derive(Debug, Default, Clone)]
pub struct ConfigsRegistry {
    packages_configs: HashMap<PathBuf, PackageConfig>,
}

impl ConfigsRegistry {
    pub fn config_for_file(&self, path: &Path) -> Option<PackageConfig> {
        self.entry_for_file(path).map(|(_, config)| config)
    }

    pub fn manifest_dir_for_file(&self, path: &Path) -> Option<PathBuf> {
        self.entry_for_file(path).map(|(dir, _)| dir)
    }

    pub fn clear(&mut self) {
        self.packages_configs.clear();
    }

    pub fn insert(&mut self, manifest_path: PathBuf, config: PackageConfig) {
        self.packages_configs.insert(manifest_path, config);
    }

    fn entry_for_file(&self, path: &Path) -> Option<(PathBuf, PackageConfig)> {
        self.packages_configs
            .iter()
            .filter_map(|(manifest_path, config)| {
                let mut manifest_dir = (*manifest_path).to_owned();

                // Should be always true but better safe than sorry.
                if manifest_dir.ends_with("Scarb.toml") {
                    manifest_dir.pop();
                }

                path.starts_with(&manifest_dir).then(|| (manifest_dir, config.clone()))
            })
            .sorted_by(|(p1, _), (p2, _)| p2.as_os_str().len().cmp(&p1.as_os_str().len()))
            .next()
    }
}
