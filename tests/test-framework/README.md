# Leo Test Framework

This directory includes the code for the Leo Test Framework.

## How it works

You would first create a rust test file inside the folder of some part of the compiler, as the test framework tests are run by the rust test framework.

### Namespaces

Then you would create a `struct` that represents a `Namespace` where you have to implement the following:

#### Parse Type

Each `namespace` must have a function, `parse_type` that returns a `ParseType`. There are several kinds of `ParseTypes`:

- Line - Parses the File line one by one.
- ContinuousLines - Parses lines continuously as one item until an empty line is encountered.
- Whole - Parses the whole file.

#### Run Test

Each `namespace` must have a function, that runs and dictates how you want the tests for that namespace to work. To make running a test possible you are given information about the test file, like its name, content, path, etc. It allows you to return any type of output to be written to an expectation file as long as it's serializable.

### Runner

Then you would create a `struct` that represents a `Runner` where you have to implement the following:

#### Resolve Namespace

Each test file only needs one runner that represents the namespace resolution to which type of test should be run with a given string.

i.e.

```rust
struct ParseTestRunner;

impl Runner for ParseTestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "Parse" => Box::new(ParseNamespace),
            "ParseExpression" => Box::new(ParseExpressionNamespace),
            "ParseStatement" => Box::new(ParseStatementNamespace),
            "Serialize" => Box::new(SerializeNamespace),
            "Input" => Box::new(InputNamespace),
            "Token" => Box::new(TokenNamespace),
            _ => return None,
        })
    }
}
```

### Rust test Function

A rust test function that calls the framework over the runner, as well as the name of the directory, is the last thing necessary.

i.e.

```rust
#[test]
pub fn parser_tests() {
	// The second argument indicates the directory where tests(.leo files)
	// would be found(tests/parser).
    leo_test_framework::run_tests(&ParseTestRunner, "parser");
}

```

### Clearing Expectations

To do so you can simply remove the corresponding `.out` file in the `tests/expectations` directory. Doing so will cause the output to be regenerated. There is an easier way to mass change them as well discussed in the next section.

### Test Framework Environment Variables

To make several aspects of the test framework easier to work with there are several environment variables:

- `TEST_FILTER` - Now runs all tests in the given directory, or the exact given test.
  - `TEST_FILTER="address" cargo test -p leo-compiler` will run all tests in the located in `tests/compiler/address`.
  - `TEST_FILTER="address/branch.leo" cargo test -p leo-compiler` will run the test located in `tests/compiler/address/branch.leo`.
- `CLEAR_LEO_TEST_EXPECTATIONS` - which if set clears all current expectations for the tests being run and regenerates them all.

To set environment variables please look at your Shell(bash/powershell/cmd/fish/etc) specific implementation for doing so

**NOTE**: Don't forget to clear the environment variable after running it with that setting, or set a temporary env variable if your shell supports it.

### Benchmarking

The test-framework is now used to easily benchmark Leo, by running on all compiler tests that have the `Pass` expectation.
Additionally, you can create additional benchmark tests by adding them in the test directory by giving them the namespace of `Bench`.

#### Running

There are currently four different kinds of benchmarks to run:

- parse - benchmarks parsing of Leo files.
- symbol - benchmarks the symbol table generation pass.
- type - benchmarks the type checking pass.
- full - benchmarks all aspects of compilation.

To run the benchmarks the command is `cargo bench -p leo-test-framework`.
This by default runs all the above-mentioned benchmark suites.
To specify a specific one you would do `cargo bench -p leo-test-framework parse` or any of the above-listed benchmark suites.

**NOTE** Benchmarks are affected by the `TEST_FILTER` environment variable.
They are also machine dependent on your pc and are impacted by other open applications.
