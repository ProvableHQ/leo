use indexmap::IndexMap;
use crate::Program;

pub trait ImportResolver {
    fn resolve_package(&self, package_segments: &[&str]) -> Option<crate::Program>;
}

pub struct NullImportResolver;

impl ImportResolver for NullImportResolver {
    fn resolve_package(&self, _package_segments: &[&str]) -> Option<crate::Program> {
        None
    }
}

pub struct StandardImportResolver;

//todo: move compiler ImportParser here
impl ImportResolver for StandardImportResolver {
    fn resolve_package(&self, _package_segments: &[&str]) -> Option<crate::Program> {
        None
    }
}

pub struct MockedImportResolver {
    pub packages: IndexMap<String, Program>,
}

impl ImportResolver for MockedImportResolver {
    fn resolve_package(&self, package_segments: &[&str]) -> Option<crate::Program> {
        self.packages.get(&package_segments.join(".")).cloned()
    }
}