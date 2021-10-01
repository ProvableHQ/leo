# Leo RFC 003: Imports Stabilization

## Authors

The Aleo Team.

## Status

IMPLEMENTED

## Summary

This proposal aims to improve the import management system in Leo programs to
make the program environment more reproducible, predictable and compatible. To achieve
that we suggest a few changes to the Leo CLI and Manifest:

- add a "dependencies" section to the Leo Manifest and add a command to pull those dependencies;
- allow custom names for imports to manually resolve name conflicts;
- add "curve" and "proving system" sections to the Manifest;
- add "include" and "exclude" parameters for "proving system" and "curve";
- add a lock file to store imported dependencies and their relations;

## Motivation

The current design of imports does not provide any guarantees on what's stored
in program imports and published with the program to Aleo Package Manager.
When a dependency is "added," it is stored inside the imports folder, and it is possible
to manually edit and/or add packages in this folder.

Also, imports are stored under the package name, which makes it impossible to import
two different packages with the same name.

Another important detail in the scope of this proposal is that, in the future, Leo
programs will have the ability to run with different proving systems
and curves, possibly creating incompatibility between programs written
for different proving systems or curves. To make a foundation for these features,
imports need to be managed with include/exclude lists for allowed (compatible)
proving systems and curves.

## Background

Leo supports the importing of packages, similarly to other languages.

A Leo program consists of a main file,
which may import additional packages (files),
which may in turn import other packages.

Imported packages may be local, i.e. found in the same project directory where the main file is.
In that case, their package names match the directory structure.

Packages may be also imported from the Aleo Package Manager (APM),
which is a repository of Aleo packages, similar to `crates.io` in Rust.
This RFC is focused on importing packages from the APM.

Each package in the APM is uniquely identified by:
* The author, who must have a registered account on the APM, with a unique username.
* The package name, which is unique within each author's account.
* The package version, which allows different versions of the same package to be treated like different packages.

## Design

### Leo Manifest - target section

To lay the foundation for the future of the Leo ecosystem and to start integrating
information about programs compatibility, we suggest adding two new fields to
the new `[target]` section of the Leo Manifest: `proving_system` and `curve`.

Currently, the Leo compiler only supports `Groth16` for the proving system and `Bls12_377`
for the curve, which are meant to be default values in Leo Manifest.

```toml
[project]
name = "first"
version = "0.1.0"
description = "The first package"
license = "MIT"

[target]
curve = "Bls12_377"
proving_system = "Groth16"
```

These fields are meant to be used to determine whether the imported program is
compatible to the original when support for different curves and proving systems
is added.

### Leo Manifest - dependencies

Dependencies section:

```toml
[dependencies]
name = { author = "author", package = "package", version = "version" }

# alternative way of adding dependency record
[dependencies.name]
author = "author"
package = "package"
version = "1.0"
```

#### Parameters description

The `name` field sets the name of the dependency in Leo code. That way we allow
developer to resolve collisions in import names manually. So, for example,
if a developer is adding the `howard/silly-sudoku` package to his program, he
might define its in-code name as `sudoku` and import it with that name:

```ts
import sudoku;
```

`package`, `author` and `version` are package name, package author and
version respectively. They are already used as arguments in `leo add`
command, so these fields are already understood by the Leo developers.

### Leo CLI

To support updating the Manifest, a new command should be added to Leo CLI.

```bash
# pull imports
leo fetch
```

### Imports Restructurization

One of the goals of the proposed changes is to allow importing packages with the
same name but different authors. This has to be solved not only on the
language level but also on the level of storing program imports.

We suggest using the set of all 3 possible program identifiers for the import
folder name: `author-package@version`. Later it can be extended to
include a hash for the version, but having the inital set already solves name
collisions.

So, the updated imports would look like:

```
leo-program
├── Leo.toml
├── README.md
├── imports
│   ├── author1-program@0.1.0
│   │    └── ...
│   ├── author2-program2@1.0.4
│        └── ...
├── inputs
│   └── ...
└── src
    └── main.leo
```

This change also affects the way imports are being processed on the ASG
level, and we need to add an imports map as an argument to the Leo compiler.
The Leo Manifest's dependencies sections needs to be parsed and passed as
a hashmap to the compiler:

```
first-program  => author1-program@0.1.0
second-program => author2-program2@1.0.4
```

### Leo.lock

For the imports map to be generated and read by the Leo binary and then by the Leo compiler,
a lock file needs to be created. The lock file should be generated by the `leo fetch` command,
which will pull the dependencies, process their manifests, and put the required information
to the file in the root directory of the program called `Leo.lock`.

The suggested structure of this file is similar to the Cargo.lock file:

```
[[package]]
name = "suit-mk2"
version = "0.2.0"
author = "ironman"
import_name = "suit-mk2"

[package.dependencies]
garbage = "ironman-suit@0.1.0"

[[package]]
name = "suit"
version = "0.1.0"
author = "ironman"
import_name = "garbage"
```

In the example above, you can see that all program dependencies are defined as an
array called `package`. Each of the dependencies contains main information about
it, including the `import_name` field which is the imported package's name in
the Leo program. Also, it stores relationships between these dependencies in the
field `dependencies`.

The format described here allows the Leo binary to form an imports map which can be
passed to the compiler.

It is important to note that the Leo.lock file is created only when a package has dependencies.
For programs with no dependencies, a lock file is not required and not created.

### Recursive Dependencies

This improvement introduces recursive dependencies. To solve this case preemptively
Leo CLI needs to check the dependency tree and throw an error when a recursive dependency
is met. We suggest implementing simple dependency tree checking while fetching
imports - if imported dependency is met on a higher level - abort the execution.

Later this solution can be improved by building a lock file containing all the
information on program dependencies, and the file itself will have enough data
to track and prevent recursion.

## Drawbacks

This change might require the update of already published programs on Aleo PM due to
Leo Manifest change. However it is possible to implement it in a backward-compatible
way.

It also introduces the danger of having recursive dependencies, but this problem is addressed in the Design section above.

## Effect on Ecosystem

The proposed improvement provides safety inside Leo programs and should not affect
the ecosystem except for the tools which use Leo directly (such as Aleo Studio).

It is possible that some of the proposed features will open new features on Aleo PM.

## Alternatives

Another approach to the stated cases is to keep everything as we have now but change
the way programs are imported and stored and make names unique. Also, the current
implementation provides some guarantees on import stablitity and consistency.
