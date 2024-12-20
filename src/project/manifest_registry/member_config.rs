use scarb_metadata::PackageMetadata;

#[derive(Debug, Clone, Default)]
pub struct MemberConfig {}

impl MemberConfig {
    pub fn from_pkg(_pkg: &PackageMetadata) -> Self {
        Self {}
    }
}
