# Leo Examples

This directory includes the following Leo code examples:

1. Hello World -> Basic Sum of two u32
2. Groups -> Basic operations over groups
3. Core -> Core circuits functions over a field type
4. Bubblesort -> Sorting algorithms over a tuple
5. Import point -> Import code from an other file 
6. Message -> Initialization of a circuit type 
7. Token -> Record example

## Build Guide

To compile each example, run:
```bash
leo build
```
When you run this command for the first time the snarkvm parameters (universal setup) will be downloaded, these are necessary to run the circuits.

To run each program, run:
```bash
leo run main
```
This command will look in the input file inputs/*.in where should find a section [main] and use the variables as inputs to the program.