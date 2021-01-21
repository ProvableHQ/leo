use indexmap::IndexMap;
use crate::{ Program, AsgConvertError, Span };

pub trait ImportResolver {
    fn resolve_package(&mut self, package_segments: &[&str], span: &Span) -> Result<Option<Program>, AsgConvertError>;
}

pub struct NullImportResolver;

impl ImportResolver for NullImportResolver {
    fn resolve_package(&mut self, _package_segments: &[&str], _span: &Span) -> Result<Option<Program>, AsgConvertError> {
        Ok(None)
    }
}

pub struct CoreImportResolver<'a, T: ImportResolver + 'static>(pub &'a mut T);

impl<'a, T: ImportResolver + 'static> ImportResolver for CoreImportResolver<'a, T> {
    fn resolve_package(&mut self, package_segments: &[&str], span: &Span) -> Result<Option<Program>, AsgConvertError> {
        if package_segments.len() > 0 && package_segments.get(0).unwrap() == &"core" {
            Ok(crate::resolve_core_module(&*package_segments[1..].join("."))?)
        } else {
            self.0.resolve_package(package_segments, span)
        }
    }
}

pub struct StandardImportResolver;

//todo: move compiler ImportParser here
impl ImportResolver for StandardImportResolver {
    fn resolve_package(&mut self, _package_segments: &[&str], _span: &Span) -> Result<Option<Program>, AsgConvertError> {
        Ok(None)
    }
}

pub struct MockedImportResolver {
    pub packages: IndexMap<String, Program>,
}

impl ImportResolver for MockedImportResolver {
    fn resolve_package(&mut self, package_segments: &[&str], _span: &Span) -> Result<Option<Program>, AsgConvertError> {
        Ok(self.packages.get(&package_segments.join(".")).cloned())
    }
}