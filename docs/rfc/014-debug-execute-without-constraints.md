# Leo RFC 014: Executing Programs Without Synthesizing Constraints

## Author(s)

The Aleo Team.

## Status

DRAFT (11/12/21)

## Summary

This RFC proposes a mechanism by which users can execute their programs without synthesizing and verifying its corresponding circuit.


## Motivation

Consider the following Leo program, which takes as input an array of one hundred thousand elements and returns a new shuffled array such that each element is in one of four bins (regions in the array).

```
// The 'round-robin-shuffle' main function.
function main(arr: [u32; 10000]) -> [u32; 10000] {
   let shuffled: [u32; 10000] = [0; 10000];
   for i in 0..10000 {
       let new_idx = ((i - ((i / 4) * 4)) * 2500) + i / 4;
       shuffled[new_idx] = arr[i];
   }
   return shuffled;
}

```
In order to run this program on some user-specified input, a user would invoke the command `leo run` which synthesizes a corresponding circuit, sets up a proving and verification key, generates a proof using the proving key and supplied inputs, and verifies the generated proof using verification key.

At the time of writing, the corresponding circuit consists of 1280000 constraints and takes ~230 seconds to produce. Setup, proving, and verification take 287, ~220, and 15 milliseconds, respectively.

The entire process must be re-run every time the program or inputs are modified, introducing significant latency in development process. Application developers cannot afford to wait for lengthy builds to test and debug their programs. This motivates an "execute" option in the Leo CLI, which allows users to run or test their programs without requiring circuit synthesis or verification.

## Design

### CLI

We propose adding a `--release` flag to `leo run` and `leo test`. If the flag is set, then the compiler synthesizes the circuit and runs the steps required for verification. Otherwise, the program is executed directly.


### Implementation

The simplest implementation would be to invoke the evaluator in `snarkVM/eval` on the intermediate representation.

## Drawbacks

This proposal does not appear to bring any drawbacks, other than introducing additional CLI options and modifying the usage of the CLI.

## Effect on Ecosystem

With a faster mechanism for executing their programs, developers can rapidly prototype and test their applications. Although they will eventually have to synthesize and verify a circuit, they can avoid this expensive step until their application is ready to be deployed.

## Alternatives

### CLI

Introduce two new commands
* `leo execute`: Directly executes the `main` function on inputs
* `leo check`: Directly executes tests



### Implementation
We discuss alternative implementations in **Future Exentions** as they provide additional value apart from direct execution.

## Future Extensions

### Performant Execution
As users build more sophisticated applications with real-time compute-intensive functions, it is likely that they would decouple the execution of the program and generating a proof of correct execution. By doing so, proof generation can be taken off the main path, allowing the application to handle greater load. Such applications would require direct execution to meet some performance baseline. The proposed implementation, while complete, is not designed for performance. Instead, a more viable alternative would be to compile Leo code to a machine executable (via LLVM) or build a performant virtual machine for the Leo lanaguage.

**Note.** This implementation should only be built if users require performant execution.

### Interactive Debugging

Intractive debugging tools allow users to step through program execution and examine intermediate values. While the design of an interactive debugger for Leo is beyond the scope of this RFC, it may inform the design of the execution mechanism.
