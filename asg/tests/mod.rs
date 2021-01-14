use leo_asg::*;

mod pass;
mod fail;

fn load_asg(content: &str) -> Result<Program, AsgConvertError> {
    leo_asg::load_asg(content, &NullImportResolver)
}

fn load_asg_imports<T: ImportResolver + 'static>(content: &str, imports: &T) -> Result<Program, AsgConvertError> {
    leo_asg::load_asg(content, imports)
}

fn mocked_resolver() -> MockedImportResolver {
    let packages = indexmap::IndexMap::new();
    MockedImportResolver {
        packages,
    }
}