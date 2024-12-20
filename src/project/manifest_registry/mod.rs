use std::collections::HashMap;
use std::path::PathBuf;

use self::member_config::MemberConfig;

pub mod member_config;

#[derive(Debug, Default, Clone)]
pub struct ManifestRegistry {
    manifests: HashMap<PathBuf, MemberConfig>,
}

impl ManifestRegistry {
    pub fn config_for_file(&self, path: &str) -> Option<MemberConfig> {
        self.manifests.iter().find_map(|(manifest_path, config)| {
            let mut manifest_dir = (*manifest_path).to_owned();
            manifest_dir.pop(); // Remove Scarb.toml from path
            let manifest_dir = manifest_dir.to_str()?;

            path.starts_with(manifest_dir).then(|| config.clone())
        })
    }

    pub fn contains_manifest(&self, path: &PathBuf) -> bool {
        self.manifests.contains_key(path)
    }

    pub fn clear(&mut self) {
        self.manifests.clear();
    }

    pub fn update(&mut self, update: ManifestRegistryUpdate) {
        self.manifests.extend(update.manifests);
    }
}

pub struct ManifestRegistryUpdate {
    manifests: HashMap<PathBuf, MemberConfig>,
}

impl From<HashMap<PathBuf, MemberConfig>> for ManifestRegistryUpdate {
    fn from(manifests: HashMap<PathBuf, MemberConfig>) -> Self {
        Self { manifests }
    }
}
