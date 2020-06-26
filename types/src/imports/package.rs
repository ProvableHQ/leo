use crate::{common::Identifier, PackageAccess, Span};
use leo_ast::imports::Package as AstPackage;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: Identifier,
    pub access: PackageAccess,
    pub span: Span,
}

impl<'ast> From<AstPackage<'ast>> for Package {
    fn from(package: AstPackage<'ast>) -> Self {
        Package {
            name: Identifier::from(package.name),
            access: PackageAccess::from(package.access),
            span: Span::from(package.span),
        }
    }
}

impl Package {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.name, self.access)
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
