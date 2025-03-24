use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub use self::member_config::MemberConfig;

mod member_config;

#[derive(Debug, Default, Clone)]
pub struct ManifestRegistry {
    manifests: HashMap<PathBuf, MemberConfig>,
}

impl ManifestRegistry {
    pub fn config_for_file(&self, path: &Path) -> Option<MemberConfig> {
        self.manifests.iter().find_map(|(manifest_path, config)| {
            let mut manifest_dir = (*manifest_path).to_owned();

            // Should be always true but better safe than sorry.
            if manifest_dir.ends_with("Scarb.toml") {
                manifest_dir.pop();
            }

            path.starts_with(manifest_dir).then(|| config.clone())
        })
    }

    pub fn clear(&mut self) {
        self.manifests.clear();
    }

    pub fn insert(&mut self, manifest_path: PathBuf, config: MemberConfig) {
        self.manifests.insert(manifest_path, config);
    }

    pub fn manifests_dirs(&self) -> impl Iterator<Item = PathBuf> {
        self.manifests.keys().map(|manifest_path| {
            let mut manifest_dir = (*manifest_path).to_owned();

            // Should be always true but better safe than sorry.
            if manifest_dir.ends_with("Scarb.toml") {
                manifest_dir.pop();
            }

            manifest_dir
        })
    }
}
