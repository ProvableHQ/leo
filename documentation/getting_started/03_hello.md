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
├── .env # The environment, containing the `NETWORK` and `PRIVATE_KEY` variables.
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

The program ID in `program` is the official name that other developers will be able to look up after the program has been deployed to a network. This must be the same as the name of your program in `main.leo`, or compilation will fail.

Dependencies will be added to the field of the same name, as they are imported. Dependencies that are only used during development and not in production will be added to the `dev_dependencies` field.

### The Code

The `src/main.leo` file is the entry point of a Leo project. It initially contains a function named `main`.
Let's break down the structure of a Leo file.

```leo file=../code_snippets/hello/src/main.leo title="src/main.leo" showLineNumbers
```

The keyword `program` indicates the name of the [program](./../language/02_structure.md#program) inside the Leo file. In this case, it is `hello.aleo`. As mentioned before, this program name must match the one in the `program.json` manifest file.

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

There is an additional function called a `constructor`. This is a special function that helps enable program upgradability, which allows you to modify some of the logic and contents of a program after you've already deployed it onchain.

```leo file=../code_snippets/hello/src/main.leo#constructor
```

The constructor acts as a gatekeeper for your program; the logic in the function gets run before every deployment and upgrade, and governs who and how this program can be deployed and modified.

:::note
All programs must have an explicitly declared constructor function.
:::

For now, we'll leave it as is, which will prevent upgrades from occurring. For more details on how program upgradability works, and different patterns for upgrading your programs, check out [Upgrading Programs](./../guides/09_program_upgradability.md).

Now let's compile and run the program.

## Build and Run

To compile the program, run:

```bash
leo build
```

On invoking the build command, Leo automatically creates a `build/⁠` and `output/`⁠ folder in the project directory. The compiled code is contained in the `build` directory. The `output` directory is used to store intermediate artifacts from compilation.

The `leo run` command will both compile and run the specified program.
In your terminal, run:

```bash
leo run main 1u32 2u32
```

```bash title="console output:"
       Leo     2 statements before dead code elimination.
       Leo     2 statements after dead code elimination.
       Leo     The program checksum is: '[212u8, 91u8, ... , 107u8]'.
       Leo ✅ Compiled 'hello.aleo' into Aleo instructions.

⛓  Constraints

 •  'hello.aleo::main' - 33 constraints (called 1 time)

➡️  Output

 • 3u32

       Leo ✅ Finished 'hello.aleo::main' (in "./hello/build")
```

## Deploying and Executing

Running programs locally is great, but you'll likely want to actually deploy your programs and execute functions onchain. To do this, you'll need to use `leo deploy` for deployment and `leo execute` to execute functions and generate the transaction containing the requisite metadata and zero-knowledge proofs.

We have dedicated guides for both [Deploying](./../guides/03_deploying.md) and [Executing](./../guides/04_executing.md), so please check those out for more information!

## Clean

Finally, you can remove all build files and outputs with:

```bash
leo clean
```

```bash title="console output:"
Leo 🧹 Cleaned the outputs directory ./hello/outputs
Leo 🧹 Cleaned the build directory ./hello/build
```

## Next Steps

To learn more about the Leo language and its syntax, start with the [language overview](./../language/00_overview.md).

To learn more about how to use the Leo CLI, start with the [CLI overview](./../cli/00_overview.md).

To get started with some sample projects, check out the **Leo By Example** section.
