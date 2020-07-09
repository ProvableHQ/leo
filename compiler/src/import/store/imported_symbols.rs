use leo_types::{Import, ImportSymbol, Package, PackageAccess};

/// Stores the the package file name and imported symbol from an import statement
#[derive(Debug)]
pub(crate) struct ImportedSymbols {
    pub symbols: Vec<(String, ImportSymbol)>,
}

impl ImportedSymbols {
    fn new() -> Self {
        Self { symbols: vec![] }
    }

    pub(crate) fn from(import: &Import) -> Self {
        let mut symbols = Self::new();

        symbols.from_package(&import.package);

        symbols
    }

    fn from_package(&mut self, package: &Package) {
        self.from_package_access(package.name.name.clone(), &package.access);
    }

    fn from_package_access(&mut self, package: String, access: &PackageAccess) {
        match access {
            PackageAccess::SubPackage(package) => self.from_package(package),
            PackageAccess::Star(span) => {
                let star = ImportSymbol::star(span);
                self.symbols.push((package, star));
            }
            PackageAccess::Symbol(symbol) => self.symbols.push((package, symbol.clone())),
            PackageAccess::Multiple(packages) => packages
                .iter()
                .for_each(|access| self.from_package_access(package.clone(), access)),
        }
    }
}
