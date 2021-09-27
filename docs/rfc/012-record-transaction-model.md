# Leo RFC 012: Improved Record and Transaction Model

## Authors

The Aleo Team.

## Status

DRAFT

## Summary

This RFC describes an improved model for how Leo programs interact with the Aleo blockchain.
The description is oriented to the Leo developer:
it does not describe the zero-knowledge details,
as the whole purpose of Leo is to enable developers
to write applications with only a very high-level understanding of zero-knowledge.

## Motivation

While Leo can be described as a regular programming language
(albeit with certain non-standard restrictions motivated by its compilation to zero-knowledge circuits),
its purpose is to build applications for the Aleo blockchain.
It is thus important to describe precisely how Leo programs operate in the Aleo blockchain.

## Background

### Zexe

The Aleo blockchain follows the Zexe model, with some variations.
It is thus useful to briefly review some aspects of Zexe first.

In Zexe, there are _records_ that contain application-specific data,
and _transactions_ that consume _n_ old records and produce _m_ new records.
The computation of the new records from the old records
is arbitrary and unknown to the blockchain;
the blockchain only enforces that the old records satisfy known _death predicates_
and that the new records satisfy known _birth predicates_.
See the [Zexe paper](https://eprint.iacr.org/2018/962.pdf) for details.

### Aleo Blockchain

In the Aleo blockchain, a transaction always consumes 2 old records and produces 2 new records.
That is, _n_ = 2 and _m_ = 2 with respect to the Zexe model.
Other choices are possible, and may be supported in the future;
the current choice of 2 old and 2 new records is motivated by
being the minimum to represent certain computations of interest, such as token exchanges,
which may involve records owned by different parties
(and therefore need to consume more than one record, since each record has a unique owner).

One or both of the old records may be dummy,
if only one old actual record is desired,
or if new records are created "from nothing".
One or both of the new records may be dummy,
if only one new actual record is desired,
or if old records just have to be consumed.

Aleo records and transactions have a well-defined structure.
They are ordered collections of typed slots.
Of particular interest is the _payload_ slot,
which contains a fixed number of bytes (currently 128)
to store application-specific data.
(Note that the developer documentation is out of date at the time of this writing.)

In the Aleo blockchain, unlike Zexe, there is no separation among
computation of new records from old records, death predicates, and birth predicates.
Instead, a Leo program plays the role of all three, as described below.

### Current Leo Program Execution Model

A Leo program is a collection of files,
with `file` as defined in the ABNF grammar,
i.e. as a sequence of declarations.
A Leo program has one main file,
which may contain import declarations,
which resolve to other files,
which may in turn contain import declarations,
and so on until a closed set of files is obtained:
that (linked) set of files is the _program_.

In order to be used in the Aleo blockchain,
a Leo program must include a function called `main`, in its aforementioned main file.
The processing of a transaction corresponds to an invocation of this `main` function.
The word 'corresponds' in the preceding sentence is important:
unlike other blockchains like Ethereum,
the processing of the transaction does not involve executing the Leo code;
rather, it involves checking a zero-knowledge proof
of the execution of the Leo program,
which was prepared when the Leo program was compiled.
This is what 'corresponds' means, in that sentence.
However, for the high-level purpose of this RFC, these are zero-knowledge details.

In general, the `main` function takes some `const` and some non-`const` inputs (declared as parameters),
and returns an output (declared as a return type), which may be a tuple to represent "multiple" outputs.
The `const` inputs are compiled into the zero-knowledge circuit,
so they can be ignored for our purpose here,
leaving only the non-`const` inputs and the output for consideration.

The execution of `main` can be described as a mathematical function
```
main : Record x Record x Inputs -> Record x Record x Output
```
where `x` is cartesian product,
`Record` is the set of possible records,
`Inputs` is the set of possible inputs to `main`, and
`Output` is the set of possible outputs from `main`.
(These sets can be thought as "types", but mathematically we talk about sets.)
That is, this mathematical function
takes three inputs (the two old records and the `main` inputs)
and returns three outputs (the two new records and the `main` output).
While `Record` is fixed, i.e. it is the same for all Leo programs,
both `Inputs` and `Output` differ based on the Leo input and output types of `main`.

In the Leo code, in `main` or in functions called by `main`,
the values in `Inputs` are accessed via the `main` parameters,
while the old records are accessed via the special `input` variable,
which provides access to the two old records and their slots,
including the payloads that contain application-specific data.
The picture for new records and values in `Output` is less clear from the documentation:
experimentation suggests that the new records are obtained
by serializing the output value in `Output` (which, recall, may be a tuple).

It is important to note that the values in `Inputs` do not come from the two old records.
Rather, they are private inputs, supplied by the developer
when they compile the Leo program and generate the zero-knowledge proof.
Indeed, as mentioned above, the processing of the transaction in the blockchain
does not execute the Leo code, and thus does not need to know the values in `Inputs`.
Rather, the blockchain has to verify a zero-knowledge proof asserting that
there exist values in `Input`, known to the creator of the transaction,
such that the execution of the Leo program's `main`
on those values and on the old records
yields the new records, along with some value in `Output`;
this is, roughly speaking, the assertion proved in zero-knowledge.

## Design

### Multiple Entry Points

We propose to generalize from one entry point (i.e. the `main` function) to multiple entry points,
in line with the smart contract paradigm.

Instead of implicitly designating `main` as the only entry point,
we need a mechanism to explicitly designate one or more Leo functions as entry points.

A simple approach could be to use an annotation like `@entrypoint` to designate _entry point functions_:
```
@entrypoint
function mint(...) -> ... { ... }

@entrypoint
function transfer(...) -> ... { ... }
```
This has a precedent, in the use of `@test` to designate Leo test functions that are not compiled to circuits.

Another approach is to use a keyword, e.g.
```
entrypoint function mint(...) -> ... { ... }

entrypoint function transfer(...) -> ... { ... }
```
Yet another approach is to group entrypoint functions inside a new block construct, e.g.
```
entrypoint {

    function mint(...) -> ... { ... }

    function transfer(...) -> ... { ... }
}
```

In the rest of this design section we assume the annotation approach (i.e. `@entrypoint`) for concreteness,
but that can be replaced as soon as we converge on a choice.

### Types for Transaction Inputs and Outputs

We propose to add types for transaction inputs and outputs to the Leo standard library,
and possibly include them in the prelude that is implicitly imported by every Leo program.

Given that records have a fixed structure with typed slots,
their format could be described by a Leo circuit type, e.g. called `Record`,
whose member variables correspond to the slots.
The types of the slots are fairly low-level,
i.e. byte arrays (e.g. `u8[128]` for the payload)
and unsigned integers (e.g. `u64` for the balance),
because they must have a clear correspondence with the serialized form of records.
This means that the Leo code may have to do
its own deserialization of the payload bytes into higher-level Leo values;
standard serialization/deserialization libraries for Leo types may be provided for this,
as an independent and more generally useful feature.

Given that a transaction input consists of two records and possibly additional information,
it makes sense to also have a circuit type `TransactionInput`,
which includes two `Record` slots and possibly additional slots.

Additionally, it makes sense to have a circuit type `TransactionOutput`
that describes the output data of a transaction that is produced by the Leo program.
This could also include two `Record` slots for the new records,
or possibly "subsets" of records if the values of some record slots are calculated
not by the Leo program but instead by the Leo CLI (i.e. build process).

All these types should be documented, as part of the standard library.
We will need to flesh out their exact definition,
but we note that this is fairly easy to change when it is in the standard library.

### Entry Point Input and Output Types

We propose that each entry point function of a Leo program
explicitly produce transaction outputs from transaction inputs
by taking a `TransactionInput` input and returning a `TransactionOutput` output:
```
@entrypoint
function ...(input: TransactionInput, ...) -> TransactionOutput { ... }
```
This way, the calculation of transaction outputs from transaction inputs is made functional and explicit.

As special cases (both of which may apply to the same entry point):
1. We could allow the `TransactionInput` input to be absent,
   when an entry point does not need access the transaction input data,
   e.g. when producing new records without consuming old records.
2. We could allow the function output to be `()` instead of `TransactionOutput`,
   when an entry point does not need to produce transaction outputs,
   e.g. when consuming old records without producing new records.

Compared to the current Leo program execution model (described earlier in the background section),
`input` is made an explicit input here, instead of being like a built-in global variable.
Furthermore, the output type is restricted to be `TransactionOutput` (or `()`),
thus eliminating the implicit serialization and the asymmetry with the treatment of transaction inputs.
There is still no restriction on the non-`TransactionInput` inputs of an entry point function;
as noted earlier, they are existentially quantified in the zero-knowledge assertion.

Thus, a Leo program entry point can be now described as a mathematical function
```
entrypoint : Record x Record x Inputs -> Record x Record
```
where `Output` is no longer present.
(If `TransactionInput` includes additional data, besides the two old records, that may affect the transaction output,
then we would need to add that to this mathematical model;
however, the model above is sufficiently accurate for the current discussion.)

We may require the `TransactionInput` input of an entry point function, if present,
to be the first input of the function, for clarity and readability.
A question is whether we should extend that requirement to non-entry-point functions
that may be passed `TransactionInput` values.
We note that none of these restrictions are necessary, though.
A necessary restriction is that each entry point function takes at most one `TransactionInput` input.

We may require the `TransactionInput` input of an entry point function, if present,
to be called `input`, or some other predefined name.
However, this is not a necessary restriction, and we may decide to demote that to a convention rather than a requirement.
(Currently `input` is a keyword and its own kind of Leo expression, which slightly complicates the language.)

### Access to Transaction Input and Output Types

Currently the member variables of Leo circuit types are always accessible for both reading and writing.
It is thus possible for a Leo program
to read from the member variables of `TransactionInput`
and to write to the member variables of `TransactionOutput`.
Therefore, for an initial implementation,
it suffices for these two circuit types to provide member variables for the needed slots.

We might want the member variables of `TransactionInput` to be read-only.
This is not necessary for the transaction model to work:
so long as `TransactionInput` is properly initialized before calling the entry point,
and that after the call the resulting `TransactionOutput` is used to create the transaction,
there is no harm in the Leo program to modify the copy of `TransactionInput` passed to the program.
Nonetheless, we may want to enforce this restriction to encourage good coding practices
(unless we find a use case to the contrary).

There is currently no mechanism in Leo to enforce that.
Designating the transaction input as `const` is not right,
as that designation normally means that the value is compiled into the circuit.

We could provide read-only access via member function (e.g. `payload()`, `balance()`),
but we still have to prohibit assignments to member variables (which is currently allowed on any circuit type).
As an orthogonal and more generally useful feature,
we could consider adding public/private access designations to Leo circuit members.
Another approach is to avoid exposing the member variables,
and just make the member functions available via an implicit import declaration.
All of this needs to be thought through more carefully, in the broader context of the Leo language design.

If `TransactionInput` has member functions, it may also be useful for `TransactionOutput` to have member functions,
presumably to create new instances and to set values of member variables.

### Proposed Leo Program Execution Model

One issue with the special `input` variable is whether it should be treated as a built-in global variable,
or whether it should be explicitly passed to the entry point functions and to the non-entry-point functions called by them.
The first approach is more concise, while the second approach is more explicit.
Note that, in the second approach, we may want to enforce certain restrictions on the use of `input`,
e.g. we may not want to allow a call `f(input, input)` even if the parameters of `f` both have the same circuit type as `input`.
There is nothing inherently wrong with `f(input, input)`, i.e. with handling `input` by value,
except that perhaps `input` is a relatively large structure,
and duplicating it generates a (relatively) large number of R1CS constraints.
Another idea is to pass `input` by (immutable) reference behind the scenes,
analogously to how we pass `self` by mutable reference to functions with `mut self`.

The treatment of output records is less clear at this point.
As mentioned above, experimentation suggests that currently the output values of `main` are serialized into new records.
This is not "symmetric" with the treatment of input records.
It may be preferable to require the Leo code to perform its own serialization of high-level data to output records,
which would often be the inverse of the deserialization from input records.
We could consider, for symmetry, to add a special `output` variable,
also with a known circuit type,
which contains (at least some of) the data in the output records, most notably the two payloads.
(It may not contain all the data of the record because some slots
have to be computed by the underlying zero-knowledge mechanisms,
outside of the Leo code.)
This `output` variable would have to be read/write, unlike `input`.
Similarly to `input`, it could be either a built-in global variable
or passed around functions by reference, in a single-threaded way.
The single-threadedness is a more important requirement here,
since the variable is read/write,
i.e. it needs to be treated like a global variable,
in the sense that there is a single instance of it.

If we go the `output` variable route, a question is what happens with the outputs of the entry point functions
(i.e. the values in `Output`, in the mathematical function described earlier).
If all the output data is explicitly written into the output record by the Leo code,
then perhaps the Leo entry point functions should always return `()`, i.e. "nothing",
or perhaps they should be predicates, i.e. return `bool`,
where `true` indicates a successful check (e.g. "yes, this private input yields this commitment when hashed")
and `false` indicates a failed check.

Another possibility is to require entry point functions to return records as outputs.
More precisely, these may be smaller structures than records,
because some of the slots of the records may only be calculated outside of Leo,
but for the present discussion we will assume that Leo can calculate the whole records.
As mentioned earlier, a transaction may generate 0, 1, or 2 new records.
Correspondingly, we could require entry point functions to return results of one of the following types:
```
@entrypoint function ...(...) -> () // when no new records are created
@entrypoint function ...(...) -> Record // when one new record is created
@entrypoint function ...(...) -> (Record, Record) // when two new records are created
// using an annotation for concreteness, but the point stands for the other options discussed
```
In other words, an entry point function can be now seen as a mathematical function
```
entrypoint : Record x Record x Inputs -> Record x Record
```
where one or both output records are dummy if the function creates less than two new records.

The above constrains each entry point to always return the same number of records.
Different entry point functions may return different numbers of records.
If we want the same entry point function
to return different numbers of records in different situations,
then it could make sense to have a more general circuit type for the output of a transaction,
which may contain 0, 1, or 2 records, and possibly other information as needed,
and require entry point functions to uniformly return values of that type:
```
@entrypoint function ...(...) -> TransactionOutput // contains 0, 1, or 2 records
```

Earlier we discussed having a known and accessible circuit type for the `input` special variable.
This type could be called `TransactionInput`, which mirrors `TransactionOutput`.
In this case, it seems more natural to treat `input` not as a global variable,
but as a parameter of entry functions;
it could be the first parameter, required for every entry function that accesses the transaction input:
```
@entrypoint function ...(input: TransactionInput, ...) -> TransactionOutput
```
We could even drop `input` as a special keyword/expression altogether,
and allow any name (but suggest a convention) for the `TransactionInput` parameter of entry point functions.

## Alternatives

The 'Design' section above already outlines several alternatives to consider.
Once we make some specific choices, we can move the other options to this section.
