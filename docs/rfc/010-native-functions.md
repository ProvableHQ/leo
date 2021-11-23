# Leo RFC 010: Improved Native Functions

## Authors

The Aleo Team.

## Status

DRAFT

# Summary

This RFC proposes an improved approach to handling natively implemented functions ('native functions', for short) in Leo,
that is functions implemented not via Leo code but (in essence) via Rust code.
Currently there is just one such function, namely BLAKE2s.
The scope of this proposal is limited to native functions defined by the developers of Leo itself,
not by users of Leo (i.e. developers of applications written in Leo).

The approach proposed here is to allow (global and member) Leo functions to have no defining bodies,
in which case they are regarded as natively implemented;
this is only allowed in Leo files that contain standard/core libraries, provided with the Leo toolchain.
Most of the compiler can work essentially in the same way as it does now;
at R1CS generation time, native functions must be recognized, and turned into the known gadgets that implement them.

# Motivation

Many languages support native functions (here we generically use 'functions' to also denote methods),
where 'native' refers to the fact that the functions are implemented not in the language under consideration,
but rather in the language used to implement the language under consideration.
For instance, Java supports native methods that are implemented in C rather than Java.

There are two main reasons for native functions in programming languages:
1. The functionality cannot be expressed at all in the language under consideration,
   e.g. Java has no constructs to print on screen, making a native implementation necessary.
2. The functionality can be realized more efficiently in the native language.

The first reason above may not apply to Leo, at least currently,
as Leo's intended use is mainly for "pure computations" rather than interactions with the external world.
However, we note that console statements could be regarded as native functions (as opposed to "ad hoc" statements),
and this may be in fact the path to pursue if we extend the scope of console features (e.g. even to full GUIs),
as has been recently proposed (we emphasize that the console code is not meant to be compiled to R1CS).

The second reason above applies to Leo right now.
While there is currently just one native function supported in Leo, namely BLAKE2s,
it is conceivable that there may be other cryptographic (or non-cryptographic) functions
for which hand-crafted R1CS gadgets are available
that are more efficient than what the Leo compiler would generate if their functionality were written in Leo.
While we will continue working towards making the Leo compiler better,
and perhaps eventually capable to generate R1CS whose efficiency is competitive with hand-crafted gadgets,
this will take time, and in the meanwhile new and more native functions may be added,
resulting in a sort of arms race.
In other words, it is conceivable that Leo will need to support native functions in the foreseeable future.

Languages typically come with standard/core libraries that application developers can readily use.
Even though the Leo standard/core libraries are currently limited (perhaps just to BLAKE2s),
it seems likely that we will want to provide more extensive standard/core libraries,
not just for cryptographic functions, but also for data structures and operations on them.

The just mentioned use case of data structures brings about an important point.
Leo struct types are reasonable ways to provide library data structures,
as they support static and instance member functions that realize operations on those data structures.
Just like some Java library classes provide a mix of native and non-native methods,
we could imagine certain Leo library struct types providing a mix of native and non-native member functions, e.g.:
```ts
struct Point2D {
    x: u32;
    y: u32;
    function origin() -> Point2D { ... } // probably non-native
    function move(mut self, delta_x: u32, delta_y: u32) { ... } // probably non-native
    function distance(self, other:Point2D); // maybe native (involves square root)
}
```

Our initial motivation for naive functions is limited to Leo standard/core libraries,
not to user-defined libraries or applications.
That is, only the developers of the Leo language will be able to create native functions.
Leo users, i.e. developers of Leo applications, will be able to use the provided native functions,
but not to create their own.
If support for user-defined native functions may become desired in the future, it will be discussed in a separate RFC.

# Design

## Background

### Current Approach to Native Functions

The BLAKE2s native function is currently implemented as follows (as a high-level view):
1. Prior to type checking/inference, its declaration (without a defining body)
   is programmatically added to the program under consideration.
   This way, the input and output types of the BLAKE2s function can be used to type-check code that calls it.
2. At R1CS generation time, when the BLAKE2s function is compiled, it is recognized as native and,
   instead of translating its body to R1CS (which is not possible as the function has no Leo body),
   a known BLAKE2s gadget is used.

This approach is fine for a single native function, but may not be the best for a richer collection of native functions.
In particular, consider the `Point2D` example above, which has a mix of native and non-native functions:
presumably, we would like to write at least the non-native functions of `Point2D` directly in a Leo file,
as opposed to programmatically generating them prior to type checking/inference.

### Multi-File Compilation

Leo already supports the compilation of multiple files that form a program, via packages and importing.
This capability is independent from native functions.

We note that, for the purpose of type checking code that calls a function `f`,
the defining body of `f` is irrelevant: only the input and output types of `f` are relevant.
The defining body is of course type-checked when `f` itself is type-checked,
and furthermore the defining body is obviously needed to generate R1CS,
but the point here is that only the input and output types of `f` are needed to type-check code that calls `f`.
In particular, this means that, if a Leo file imports a package,
only the type information from the package is needed to type-check the file that imports the package.
Conceptually, each package exports a symbol table, used (and sufficient) to type-check files that import that package.

## Proposal

we propose to:
1. Allow declarations of (global and member) functions to have no defining body, signaling that the function is native.
2. Extend the AST and ASG to allow functions to have no bodies.
3. Have the compiler allow empty function bodies only in standard/core library files, which should be known.
4. Have type checking/inference "skip over" absent function bodies.
5. At R1CS generation time, when a function without body is encountered, find and use the known gadget for it.

Currently the ABNF grammar requires function declarations to have a defining body (a block), i.e. to be implemented in Leo:
```
function-declaration = *annotation %s"function" identifier
                       "(" [ function-parameters ] ")" [ "->" type ]
                       block
```
We propose to relax the rule as follows:
```
function-declaration = *annotation %s"function" identifier
                       "(" [ function-parameters ] ")" [ "->" type ]
                       ( block / ";" )
```
This allows a function declaration to have a terminating semicolon instead of a block.

Since we do not have anything like abstract methods in Leo, this is a workable way to indicate native functions.
However, it is easy, if desired, to have a more promiment indication, such as a `native` keyword, or an annotation.

It may be necessary to extend the AST and ASG to accommodate function bodies to be optional,
although this may be already the case for representing BLAKE2s in its current form described above.

The compiler should know which files are part of the Leo standard/core libraries and which ones are not.
Functions without bodies will be only allowed to appear in those files.
It will be an error if any other file (e.g. user-defined) contains functions without bodies.
Type checking/inference may be where we make this check, or perhaps in some other phase.

Because of the already existing support for multi-file compilation described above,
no essential change is needed in the compiler's type checking/inference.
We just need to make sure that functions without bodies are expected and properly handled
(i.e. their input and output types must be checked and added to the proper symbol tables,
but their absent bodies must be skipped);
this may already be the case, for the treatment of BLAKE2s described above.

The main change is in R1CS generation.
Normally, when a function definition is encountered, its Leo body is translated to R1CS.
For a native function, we need to find and use a known gadget instead.
The compiler must know a mapping from native functions in the standard/core libraries
to the R1CS gadgets that implement them, so it should be just a matter of selecting the appropriate one.
Some of this logic must be already present, in order to detect and select the BLAkE2s gadget.

This approach is used in Java, where Java files may declare certain methods as `native`,
without a body but with a declaration of input and output types.
The actual native implementations, i.e. the native method bodies live in different files, as they are written in C.

# Drawbacks

This does not seem to bring any drawbacks.
A capability for native functions (for BLAKE2s) already exists;
this RFC proposes a way to make it more flexible,
with mild (and likely simplifying) changes to the compiler.

# Effect on Ecosystem

This should help support richer standard/core libraries for Leo.

# Alternatives

## Programmatic Generation

Instead of storing declarations of native functions in standard/core files as proposed above,
we could programmatically generate them as currently done for BLAKE2s.
Macros may be used to generate families of similar function declarations.

However, consider `Point2D` above, which has a mix or native and non-native functions.
One approach is to programmatically generate the whole `Point2D` declarative,
with both native and non-native functions.
But it seems that a Leo file would be clearer and more maintainable than a Rust string in the compiler.
We could think of splitting the non-native and native functions of `Point2D`:
the former in a Leo file, and the latter programmatically added.
Again, this looks more complicated than just declaring native funcions in Leo files.

## Leo Code in Rust Files

It has been pointed out that it would be beneficial to have
both the Leo code (for the non-native functions)
and the Rust code (for the native functions)
in the same place (i.e. file).
This is not possible if the non-native code is in a Leo file, because Leo files cannot contain Rust code
(and there is no plan to allow that, i.e. no inline Rust code).

However, we can turn things around and leverage Rust's macro system to accommodate Leo code in Rust files.
That is, we can have Rust files that include both the non-native Leo code,
written as Leo code (with some surrounding macro call or something like that),
along with the Rust code that implements the naive functions.

This may turn out to be in fact the preferred design in the end,
as it combines the advantage of writing non-native code in Leo
with the advantage of having native and non-native code in the same place.
In that case, we will revise this RFC to swap this design proposal with the one in the main section,
moving the proposal for Leo files to this section as an alternative.