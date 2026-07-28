---
id: hello
title: Hello, Leo
sidebar_label: Hello, Leo
---

[general tags]: # "hello_leo, starter_project"

## Initialize the project

Use the Leo Command Line Interface (CLI) to create a new project.
In your terminal, run:

```bash
leo new hello
cd hello
```

This creates a directory with the following structure:

```bash
hello/
├── .gitignore # A default `.gitignore` file for Leo projects
├── program.json # The manifest for the Leo project
├── tests/
  └── test_hello.leo # The Leo source code for unit tests
└── src/
  └── main.leo # The Leo source code
```

## Unpacking the Project

### The Manifest

**program.json** is the Leo manifest file that configures the package.

```json file=../code_snippets/hello/program.json title="program.json"
```

The `program` field contains the official program ID. After network deployment, other developers can use this ID to find the program. It must match the program name in `main.leo`. Otherwise, compilation fails.

Dependencies will be added to the field of the same name, as they are imported. Dependencies that are only used during development and not in production will be added to the `dev_dependencies` field.

### The Code

The `src/main.leo` file is the entry point of a Leo project. It initially contains a function named `main`. The following sections explain the structure of a Leo file.

```leo file=../code_snippets/hello/src/main.leo title="src/main.leo" showLineNumbers
```

The keyword `program` indicates the name of the [program](./../language/structure.md#program) inside the Leo file. In this case, it is `hello.aleo`. As mentioned before, this program name must match the one in the `program.json` manifest file.

The keyword `fn` indicates an entry function definition in Leo.
The `main` function takes an input `a` with type `u32` and `public` visibility, and an input `b` with type `u32` and `private` visibility (by default).
The function returns one result with type `u32`.
The function body is enclosed in curly braces `{ }`.

```leo file=../code_snippets/hello/src/main.leo#signature
```

Inside the `main` function we declare a variable `c` with type `u32` and set it equal to the addition of variables `a` and `b`.
Leo's compiler will check that the types of `a` and `b` are equal and that the result of the addition is type `u32`.

```leo file=../code_snippets/hello/src/main.leo#addition
```

:::note
Leo is designed to detect many errors at compile time, via statically checked strong types.
Try changing the type of any variable and seeing what Leo recommends with helpful error messages.
:::

Last, we return the variable `c`.
Leo will check that `c`'s type matches the function return type `u32`.

```leo file=../code_snippets/hello/src/main.leo#ret
```

The file also contains a `constructor` function. This function enables program upgrades. An upgrade can change some program logic and content after on-chain deployment.

```leo file=../code_snippets/hello/src/main.leo#constructor
```

The constructor controls program deployment and upgrades. The network runs its logic before each deployment and upgrade.

:::note
All programs must have an explicitly declared constructor function.
:::

For now, we will leave it as is, which will prevent upgrades from occurring. For more details on how program upgradability works, and different patterns for upgrading your programs, check out [Upgrading Programs](./../guides/program_upgradability.md).

Now, compile and run the program.

## Build and Run

To compile the program, run:

```bash
leo build
```

On invoking the build command, Leo automatically creates a `build/⁠` folder in the project directory. Inside it, every program - your own program and each dependency - gets its own `build/{program}/` directory containing its compiled `.aleo` bytecode and ABI.

The `leo run` command will both compile and run the specified program.
In your terminal, run:

```bash
leo run main 1u32 2u32
```

```bash title="console output:"
       Leo     2 statements before dead code elimination.
       Leo     2 statements after dead code elimination.
       Leo ✅ Compiled 'hello.aleo' into Aleo instructions.

⛓  Constraints

 •  'hello.aleo::main' - 33 constraints (called 1 time)

➡️  Output

 • 3u32

       Leo ✅ Finished 'hello.aleo::main' (in "./hello/build")
```

## Deploying and Executing

After local tests, deploy the program and execute functions on-chain. Use `leo deploy` for deployment. Use `leo execute` to execute functions and generate a transaction. The transaction contains the required metadata and zero-knowledge proofs.

We have dedicated guides for both [Deploying](./../guides/deploying.md) and [Executing](./../guides/executing.md), so please check those out for more information!

## Clean

Finally, you can remove all build artifacts with:

```bash
leo clean
```

```bash title="console output:"
Leo 🧹 Cleaned the build directory ./hello/build
```

## Next Steps

To learn more about the Leo language and its syntax, start with the [language overview](./../language/overview.md).

To learn more about how to use the Leo CLI, start with the [CLI overview](./../cli/overview.md).

To get started with some sample projects, check out the **Leo By Example** section.
