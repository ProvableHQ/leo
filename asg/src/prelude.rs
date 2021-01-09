
//todo: we should merge this with core

use crate::{ Program, AsgConvertError };

// todo: make asg deep copy so we can cache resolved core modules
// todo: figure out how to do headers without bogus returns

pub fn resolve_core_module(module: &str) -> Result<Option<Program>, AsgConvertError> {
    match module {
        "unstable.blake2s" => {
            Ok(Some(crate::load_asg(r#"
            circuit Blake2s {
                function hash(seed: [u8; 32], message: [u8; 32]) -> [u8; 32] {
                    return [0; 32]
                }
            }
            "#, &crate::NullImportResolver)?))
        },
        _ => Ok(None),
    }
}