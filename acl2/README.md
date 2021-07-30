# ACL2 Theorem Generation and Checking for Leo Compilation

## Setup

1. You will need the contents of the current directory, `leo-acl2/testing/bin/`.
   In step 6, replace `/path/to` by where you put these files.

2. Before running theorem generation and checking, unzip the acl2 image file:
```
    cd /path/to
    gunzip leo-acl2.lx86cl64.gz
```

## Inputs to Theorem Generation and Checking

3. To run theorem generation and checking for the canonicalization phase
   for a particular leo program, you must first generate the files
     initial_ast.json
     canonicalization_ast.json
   The way you do that is dependent on the interface to the Leo compiler.
   As of this writing you will need a standard leo program structure
   and then do `leo build --enable-all-theorems` which writes the json files
   to the "outputs/" directory.
   In the next step, we assume the current directory contains these JSON files

## Running

4. Use the tgc script to run ACL2 theorem generation and checking
   on the initial_ast.json and canonicalization_ast.json files.

   The last argument to tgc is the name of a new file that will contain the
   theorem of correctness.  It must end in ".lisp".
```
    /path/to/tgc canonicalization initial_ast.json canonicalization_ast.json canonicalization-theorem.lisp
```
   The tgc script returns an exit status of 0 if the theorem was successfully
   generated and checked (proved).

## Run with Docker

Simply build an image:

```
docker build -t acl2 .
``` 
