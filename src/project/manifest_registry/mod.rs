use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct ManifestRegistry {
    manifests: HashSet<PathBuf>,
}

impl ManifestRegistry {
    pub fn contains_manifest(&self, path: &PathBuf) -> bool {
        self.manifests.contains(path)
    }

    pub fn clear(&mut self) {
        self.manifests.clear();
    }

    pub fn update(&mut self, update: ManifestRegistryUpdate) {
        self.manifests.extend(update.manifests);
    }
}

pub struct ManifestRegistryUpdate {
    manifests: HashSet<PathBuf>,
}

impl From<HashSet<PathBuf>> for ManifestRegistryUpdate {
    fn from(manifests: HashSet<PathBuf>) -> Self {
        Self { manifests }
    }
}
