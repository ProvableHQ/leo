# Leo RFC 003: Imports Stabilization

## Authors

- Max Bruce
- Collin Chin
- Alessandro Coglio
- Eric McCarthy
- Jon Pavlik
- Damir Shamanaev
- Damon Sicore
- Howard Wu

## Status

DRAFT

# Summary

This proposal aims to improve the import management system in Leo programs to
make program environment more reproducible, predictable and compatible. To achieve 
that we suggest few changes to Leo CLI and Manifest:

- add a "dependencies" section to Leo Manifest and add a command to pull those dependencies;
- allow custom names for imports to manually resolve name conflicts;
- add "curve" and "proving system" sections to the Manifest;
- add "include" and "exclude" parameters for "proving system" and "curve";

Later this solution can be improved by adding a lock-file which would lock
imported packages based on both their contents and version. 

# Motivation

The current design of imports does not provide any guarantees on what's stored 
in program imports and published with the program to Aleo Package Manager. 
When dependency is "added", it is stored inside imports folder, and it is possible
to manually edit and/or add packages in this folder.

Also, imports are stored under package name which makes it impossible to import
two different packages with the same name. 

Another important detail in the scope of this proposal is that in future Leo
programs will have the ability to be run with different proving systems 
and curves, possibly creating incompatibility between programs written 
for different proving systems or curves. To make a foundation for these features
imports need to be managed with include/exclude lists for allowed (compatible) 
proving systems and curves.

# Design

## Leo Manifest - target section

To lay the foundation for future of the Leo ecosystem and start integrating
information about programs compatibility we suggest adding two new fields to
the new `[target]` section of the Leo Manifest: `proving_system` and `curve`.

Currently, Leo compiler only supports `Groth16` for proving system and `Bls12_377`
for curve, they are meant to be default values in Leo Manifest.

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

These fields are meant to be used to determine whether imported program is 
compatible to the original when support for different curves and proving systems
is added.

## Leo Manifest - dependencies

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

### Parameters description

`name` field sets the name of the dependency in Leo code. That way we allow 
developer to resolve collisions in import names manually. So, for example,
if a developer is adding `howard/silly-sudoku` package to his program, he
might define its in-code name as `sudoku` and import it with that name:

```ts
import sudoku;
```

`package`, `author` and `version` are package name, package author and 
version respectively. They are already used as arguments in `leo add` 
command, so these fields are already understood by the Leo developers.

## Leo CLI 

To support updated Manifest new command should be added to Leo CLI. 

```bash
# pull imports
leo install
```

Alternatively it can be called `pull`.
```
leo pull
```

## Imports Restructurization

One of the goals of proposed changes is to allow importing packages with the
same name but different authors. This has to be solved not only on the 
language level but also on the level of storing program imports. 

We suggest using set of all 3 possible program identifiers for import
folder name: `author-package@version`. Later it can be extended to 
include hash for version, but having inital set already solves name
collisions.

So, updated imports would look like:

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

This change would also affect the way imports are being processed on ASG
level, and we'd need to add imports map as an argument to the Leo compiler.
The Leo Manifest's dependencies sections needs to be parsed and passed as 
a hashmap to the compiler:

```
first-program  => author1-program@0.1.0
second-program => author2-program2@1.0.4
```

## Recursive Dependencies

This improvement introduces recursive dependencies. To solve this case preemptively
Leo CLI needs to check dependency tree and throw an error when recursive dependency
is met. We suggest implementing simple dependency tree checking while fetching
imports - if imported dependency is met on higher level - abort the execution.

Later this solution can be improved by building a lock file containing all the
information on program dependencies, and the file itself will have enough data
to track and prevent recursion.

# Drawbacks

This change might require the update of already published programs on Aleo PM due to
Leo Manifest change. However it is possible to implement it in a backward-compatible
way.

It also introduces the danger of having recursive dependencies, this problem is addressed in the Design section above.

# Effect on Ecosystem

Proposed improvement provides safety inside Leo programs and should not affect
ecosystem except for the tools which use Leo directly (such as Aleo Studio). 

It is possible that some of the proposed features will open new features on Aleo PM. 

# Alternatives

Another approach to the stated cases is to keep everything as we have now but change
the way programs are imported and stored and make names unique. Also, current 
implementation provides some guarantees on import stablitity and consistency. 
