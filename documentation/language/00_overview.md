---
id: overview
title: The Leo Language Reference
sidebar_label: Overview
---

[general tags]: # "syntax"

## Statically Typed

Leo is a **statically typed language**, which means we must know the type of each variable before executing a circuit.

Leo does not support `undefined` or `null` values. When creating a new variable, its type must be either:

- **Explicitly stated** using a type annotation, or
- **Automatically inferred** by the compiler.

<!-- The exception to this rule is when a new variable inherits its type from a previous variable. -->

## Pass by Value

Expressions in Leo are always **passed by value**, which means their values are always copied when they are used as
function inputs or in right sides of assignments.
