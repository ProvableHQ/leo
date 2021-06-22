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

This proposal aims to improve import management system in Leo programs to
make program environment more reproducible and predictable. To achieve that
we suggest few changes to Leo CLI and Manifest:

- add "dependencies" section to Leo Manifest and add command to pull those dependencies;
- allow custom names for imports to manually resolve name conflicts;
- store imports as they are called in Leo Manifest;

Later this solution can be improved by adding a lock-file which would lock
imported packages based on both their contents and version. 

# Motivation

What problems does it solve? What is the background?

Current state:
- imports are published with program to Aleo PM;
- we treat program as files with no verification of imports (they can be changed locally and published in that state);
- name collisions cannot be resolved; new import overwrites existing;

TBD

# Design

## Leo Manifest

Dependencies section:

```toml
[dependencies]
name = { author = "author", package = "package", version = "version" }

[dependencies.name]
author = "author"
package = "package"
version = "1.0"
```

TBD

## Leo CLI 

To support updated Manifest new command should be added to Leo CLI. 

```bash
# pull imports
leo pull 
```

## Imports Restructurization

One of the goals of proposed changes is to allow importing packages with the
same name but different authors. To resolve name conflict we suggest storing
imports as they are named in Leo Manifest file (Leo.toml).


<!-- Suggested change is soft. It changes only the way imports are organized 
with minimal changes to other parts of the language.

We can consider implementing imports/username-package storage, but imports 
will have to be resolved on a different level in compiler. -->

# Drawbacks

This change might require update of already published programs on Aleo PM due to
Leo Manifest change. However it is possible to implement it in a backward-compatible
way.

# Effect on Ecosystem

Proposed improvement provides safety inside Leo programs and should not affect
ecosystem except for tools which use Leo directly (such as Aleo Studio). 

It is possible that some of the proposed features open new features on Aleo PM. 

# Alternatives

Another approach to stated cases is to keep everything as we have now but change
the way programs are imported and stored and make names unique. Also, current 
implementation provides some guarantees on import stablitity and consistency. 
