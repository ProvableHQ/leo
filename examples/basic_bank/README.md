<!-- # 🏦 Basic Bank -->

[//]: # (<img alt="workshop/basic_bank" width="1412" src="../.resources/basic_bank.png">)

A simple-interest yielding bank account in Leo.

## Summary

This program implements a bank that issues tokens to users and allows users to deposit tokens to accrue simple interest on their deposits.

### User Flow
1. The bank issues users tokens via the `issue` function.
2. A user deposits tokens via the `deposit` function.
3. Upon a user's request to withdraw, the bank calculates the appropriate amount of compound interest and pays the user the principal and interest via the `withdraw` function.

Note that the program can be easily extended to include addition features such as a `transfer` function, which would allow users to transfer tokens to other users.

## Bugs

You may have already guessed that this program has a few bugs. We list some of them below: 
- `withdraw` can only be invoked by the bank. A malicious bank could lock users' tokens by not invoking `withdraw`.
- `withdraw` fails if the sum of the interest and principal is greater than the user's balance. 
- User's can increase their principal by depositing tokens multiple times, including immediately before withdrawl.
- Integer division rounds down; if the calculated interest is too small, then it will be rounded down to zero.

Can you find any others?

## Language Features and Concepts
- `record` declarations
- `assert_eq`
- core functions, e.g. `BHP256::hash_to_field`
- record ownership
- loops and bounded iteration
- mappings
- finalize

## Running the Program

Leo provides users with a command line interface for compiling and running Leo programs.
Users may either specify input values via the command line or provide an input file in `inputs/`.

### Configuring Accounts
The `.env` file contains a private key.
This is the account that will be used to sign transactions and is checked for record ownership.
When executing programs as different parties, be sure to set the `PRIVATE_KEY` field in `.env` to the appropriate values.
See `./run.sh` for an example of how to run the program as different parties.


The [Aleo SDK](https://github.com/AleoHQ/leo/tree/testnet3) provides an interface for generating new accounts.
To generate a new account, navigate to [aleo.tools](https://aleo.tools).

### Providing inputs via the command line.
1. Run
```bash
leo run <function_name> <input_1> <input_2> ...
```
See `./run.sh` for an example.


### Using an input file.
1. Modify `inputs/auction.in` with the desired inputs.
2. Run
```bash
leo run <function_name>
```
For example,
```bash
leo run issue
leo run deposit
leo run withdraw
```
