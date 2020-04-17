use std::fmt;
use std::path::Path;

type ImportPath<'ast> = &'ast Path;
pub(crate) type PathString<'ast> = &'ast str;

#[derive(Clone)]
pub struct Import<'ast> {
    source: ImportPath<'ast>,
    symbol: Option<PathString<'ast>>,
    alias: Option<PathString<'ast>>,
}

impl<'ast> Import<'ast> {
    pub fn new(symbol: Option<PathString<'ast>>, source: ImportPath<'ast>) -> Import<'ast> {
        Import {
            source,
            symbol,
            alias: None,
        }
    }

    pub fn new_with_alias(
        symbol: Option<PathString<'ast>>,
        source: ImportPath<'ast>,
        alias: PathString<'ast>,
    ) -> Import<'ast> {
        Import {
            source,
            symbol,
            alias: Some(alias),
        }
    }

    pub fn alias(mut self, alias: Option<PathString<'ast>>) -> Self {
        self.alias = alias;
        self
    }

    pub fn get_alias(&self) -> &Option<PathString<'ast>> {
        &self.alias
    }

    pub fn get_source(&self) -> &Path {
        &self.source
    }

    pub fn get_file(&self) -> String {
        let path = self.get_source().to_str().unwrap();
        format!("{}.program", path)
    }
}

impl<'ast> fmt::Display for Import<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.alias {
            Some(ref alias) => write!(f, "import {} as {}", self.source.display(), alias),
            None => write!(f, "import {}", self.source.display()),
        }
    }
}

impl<'ast> fmt::Debug for Import<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.alias {
            Some(ref alias) => write!(
                f,
                "import(source: {}, alias: {})",
                self.source.display(),
                alias
            ),
            None => write!(f, "import( source: {})", self.source.display()),
        }
    }
}
