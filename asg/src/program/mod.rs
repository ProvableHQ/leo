
mod circuit;
pub use circuit::*;

mod function;
pub use function::*;

use indexmap::{ IndexMap, IndexSet };

use std::sync::{ Arc };
use crate::{ AsgConvertError, InnerScope, Scope, ImportResolver, Input };
use std::cell::RefCell;
use leo_ast::{Package, PackageAccess, Span};

#[derive(Clone)]
pub struct Program {
    pub name: String,
    pub imported_modules: Vec<Program>, // these should generally not be accessed directly, but through scoped imports
    pub test_functions: IndexMap<String, (Arc<FunctionBody>, Option<leo_ast::Identifier>)>, // identifier = test input file
    pub functions: IndexMap<String, Arc<FunctionBody>>,
    pub circuits: IndexMap<String, Arc<CircuitBody>>,
    pub scope: Scope,
}

enum ImportSymbol {
    Direct(String),
    Alias(String, String), // from remote -> to local
    All,
}

fn resolve_import_package(output: &mut Vec<(Vec<String>, ImportSymbol, Span)>, mut package_segments: Vec<String>, package: &Package) {
    package_segments.push(package.name.name.clone());

    resolve_import_package_access(output, &package_segments, &package.access);
}

fn resolve_import_package_access(output: &mut Vec<(Vec<String>, ImportSymbol, Span)>, package_segments: &Vec<String>, package: &PackageAccess) {
    match package {
        PackageAccess::Star(span) => {
            output.push((package_segments.clone(), ImportSymbol::All, span.clone()));
        },
        PackageAccess::SubPackage(subpackage) => {
            resolve_import_package(output, package_segments.clone(), &*subpackage);
        },
        PackageAccess::Symbol(symbol) => {
            let span = symbol.symbol.span.clone();
            let symbol = if let Some(alias) = symbol.alias.as_ref() {
                ImportSymbol::Alias(symbol.symbol.name.clone(), alias.name.clone())
            } else {
                ImportSymbol::Direct(symbol.symbol.name.clone())
            };
            output.push((package_segments.clone(), symbol, span));
        },
        PackageAccess::Multiple(subaccesses) => {
            for subaccess in subaccesses.iter() {
                resolve_import_package_access(output, &package_segments, &subaccess);
            }
        },
        
    }
}

impl Program {

    /*
    stages:
    1. resolve imports into super scope
    2. finalize declared types
    3. finalize declared functions
    4. resolve all asg nodes
    */
    pub fn new<'a, T: ImportResolver + 'static>(value: &leo_ast::Program, import_resolver: &'a T) -> Result<Program, AsgConvertError> {
        // TODO: right now if some program A is imported from programs B and C, it will be copied to both, which is not optimal -- needs fixed

        // recursively extract our imported symbols
        let mut imported_symbols: Vec<(Vec<String>, ImportSymbol, Span)> = vec![];
        for import in value.imports.iter() {
            resolve_import_package(&mut imported_symbols, vec![], &import.package);
        }

        // package list
        let mut deduplicated_imports: IndexSet<Vec<String>> = IndexSet::new();
        for (package, _symbol, _span) in imported_symbols.iter() {
            deduplicated_imports.insert(package.clone());
        }

        let wrapped_resolver = crate::CoreImportResolver(import_resolver);
        // load imported programs
        let mut resolved_packages: IndexMap<Vec<String>, Program> = IndexMap::new(); 
        for package in deduplicated_imports.iter() {
            let pretty_package = package.join(".");

            let resolved_package = match wrapped_resolver.resolve_package(&package.iter().map(|x| &**x).collect::<Vec<_>>()[..])? {
                Some(x) => x,
                None => return Err(AsgConvertError::unresolved_import(&*pretty_package, &Span::default())), // todo: better span
            };

            resolved_packages.insert(package.clone(), resolved_package);
        }

        let mut imported_functions: IndexMap<String, Arc<FunctionBody>> = IndexMap::new();
        let mut imported_circuits: IndexMap<String, Arc<CircuitBody>> = IndexMap::new();

        // prepare locally relevant scope of imports
        for (package, symbol, span) in imported_symbols.into_iter() {
            let pretty_package = package.join(".");

            let resolved_package = resolved_packages.get(&package).expect("could not find preloaded package");

            match symbol {
                ImportSymbol::All => {
                    imported_functions.extend(resolved_package.functions.clone().into_iter());
                    imported_circuits.extend(resolved_package.circuits.clone().into_iter());
                },
                ImportSymbol::Direct(name) => {
                    if let Some(function) = resolved_package.functions.get(&name) {
                        imported_functions.insert(name.clone(), function.clone());
                    } else if let Some(function) = resolved_package.circuits.get(&name) {
                        imported_circuits.insert(name.clone(), function.clone());
                    } else {
                        return Err(AsgConvertError::unresolved_import(&*format!("{}.{}", pretty_package, name), &span));
                    }
                },
                ImportSymbol::Alias(name, alias) => {
                    if let Some(function) = resolved_package.functions.get(&name) {
                        imported_functions.insert(alias.clone(), function.clone());
                    } else if let Some(function) = resolved_package.circuits.get(&name) {
                        imported_circuits.insert(alias.clone(), function.clone());
                    } else {
                        return Err(AsgConvertError::unresolved_import(&*format!("{}.{}", pretty_package, name), &span));
                    }
                },
            }
        }

        let import_scope = Arc::new(RefCell::new(InnerScope {
            id: uuid::Uuid::new_v4(),
            parent_scope: None,
            circuit_self: None,
            variables: IndexMap::new(),
            functions: imported_functions.iter().map(|(name, func)| (name.clone(), func.function.clone())).collect(),
            circuits: imported_circuits.iter().map(|(name, circuit)| (name.clone(), circuit.circuit.clone())).collect(),
            function: None,
            input: None,
        }));

        // prepare header-like scope entries
        let mut proto_circuits = IndexMap::new();
        for (name, circuit) in value.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = Circuit::init(circuit)?;
            
            proto_circuits.insert(name.name.clone(), asg_circuit);
        }

        let scope = Arc::new(RefCell::new(InnerScope {
            id: uuid::Uuid::new_v4(),
            parent_scope: Some(import_scope),
            circuit_self: None,
            variables: IndexMap::new(),
            functions: IndexMap::new(),
            circuits: proto_circuits.iter().map(|(name, circuit)| (name.clone(), circuit.clone())).collect(),
            function: None,
            input: Some(Input::new()),
        }));

        for (name, circuit) in value.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = proto_circuits.get(&name.name).unwrap();

            asg_circuit.clone().from_ast(&scope, &circuit)?;
        }

        let mut proto_test_functions = IndexMap::new();
        for (name, test_function) in value.tests.iter() {
            assert_eq!(name.name, test_function.function.identifier.name);
            let function = Arc::new(Function::from_ast(&scope, &test_function.function)?);

            proto_test_functions.insert(name.name.clone(), function);
        }
        let mut proto_functions = IndexMap::new();
        for (name, function) in value.functions.iter() {
            assert_eq!(name.name, function.identifier.name);
            let asg_function = Arc::new(Function::from_ast(&scope, function)?);

            scope.borrow_mut().functions.insert(name.name.clone(), asg_function.clone());
            proto_functions.insert(name.name.clone(), asg_function);
        }

        // load concrete definitions
        let mut test_functions = IndexMap::new();
        for (name, test_function) in value.tests.iter() {
            assert_eq!(name.name, test_function.function.identifier.name);
            let function = proto_test_functions.get(&name.name).unwrap();

            let body = Arc::new(FunctionBody::from_ast(&scope, &test_function.function, function.clone())?);
            function.body.replace(Arc::downgrade(&body));

            test_functions.insert(name.name.clone(), (body, test_function.input_file.clone()));
        }
        let mut functions = IndexMap::new();
        for (name, function) in value.functions.iter() {
            assert_eq!(name.name, function.identifier.name);
            let asg_function = proto_functions.get(&name.name).unwrap();

            let body = Arc::new(FunctionBody::from_ast(&scope, function, asg_function.clone())?);
            asg_function.body.replace(Arc::downgrade(&body));

            functions.insert(name.name.clone(), body);
        }
        let mut circuits = IndexMap::new();
        for (name, circuit) in value.circuits.iter() {
            assert_eq!(name.name, circuit.circuit_name.name);
            let asg_circuit = proto_circuits.get(&name.name).unwrap();
            let body = Arc::new(CircuitBody::from_ast(&scope, circuit, asg_circuit.clone())?);
            asg_circuit.body.replace(Arc::downgrade(&body));

            circuits.insert(name.name.clone(), body);
        }

        Ok(Program {
            name: value.name.clone(),
            test_functions,
            functions,
            circuits,
            imported_modules: resolved_packages.into_iter().map(|(_, program)| program).collect(),
            scope,
        })
    }
}

impl Into<leo_ast::Program> for &Program {
    fn into(self) -> leo_ast::Program {
        leo_ast::Program {
            name: self.name.clone(),
            imports: vec![],
            expected_input: vec![],
            circuits: self.circuits.iter().map(|(_, circuit)| (circuit.circuit.name.clone(), circuit.circuit.as_ref().into())).collect(),
            functions: self.functions.iter().map(|(_, function)| (function.function.name.clone(), function.function.as_ref().into())).collect(),
            tests: self.test_functions.iter().map(|(_, function)| (function.0.function.name.clone(), leo_ast::TestFunction {
                function: function.0.function.as_ref().into(),
                input_file: function.1.clone(),
            })).collect(),
        }
    }
}
