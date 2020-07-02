use leo_types::{Import, ImportSymbol, Package, PackageAccess};

#[derive(Debug)]
pub(crate) struct ImportedSymbols {
    pub name: String,
    pub symbols: Vec<ImportSymbol>,
}

impl ImportedSymbols {
    fn new(name: String) -> Self {
        Self { name, symbols: vec![] }
    }

    pub(crate) fn from(import: &Import) -> Self {
        let mut symbols = Self::new(import.package.name.name.clone());

        symbols.from_package_access(&import.package.access);

        symbols
    }

    fn from_package(&mut self, package: &Package) {
        self.name = package.name.name.clone();

        self.from_package_access(&package.access);
    }

    fn from_package_access(&mut self, access: &PackageAccess) {
        match access {
            PackageAccess::SubPackage(package) => self.from_package(package),
            PackageAccess::Star(span) => {
                let star = ImportSymbol::star(span);
                self.symbols.push(star);
            }
            PackageAccess::Symbol(symbol) => self.symbols.push(symbol.clone()),
            PackageAccess::Multiple(packages) => packages.iter().for_each(|access| self.from_package_access(access)),
        }
    }
}
