---
id: basic_bank
title: A Basic Bank using Leo
---

[general tags]: # "example, bank, record, program, assert, hash, loops, mappings"

**[Source Code](https://github.com/ProvableHQ/leo-examples/tree/main/basic_bank)**

## Summary

This program implements a bank that issues tokens to users and allows users to deposit tokens to accrue simple interest on their deposits.

### User Flow

1. The bank issues users tokens via the `issue` function.
2. A user deposits tokens via the `deposit` function.
3. When a user requests a withdrawal, the bank calculates the compound interest.
   The `withdraw` function pays the principal and interest to the user.

You can extend the program with more features.
For example, a `transfer` function can let users send tokens to other users.

## Bugs

You may have already guessed that this program has a few bugs. We list some of them below:

- `withdraw` can only be invoked by the bank. A malicious bank could lock users' tokens by not invoking `withdraw`.
- `withdraw` fails if the sum of the interest and principal is greater than the user's balance.
- Users can increase their principal by depositing tokens multiple times, including immediately before withdrawal.
- Integer division rounds down. If the calculated interest is too small, then it will be rounded down to zero.

Can you find any others?

There are, of course, ways to write a version of this application without these bugs. This could be a good exercise for the reader.

## Language Features and Concepts

- `record` declarations
- `assert_eq`
- core functions, for example `BHP256::hash`
- record ownership
- loops and bounded iteration
- mappings
- `final` blocks

## How to Run

Follow the [Leo Installation Instructions](https://docs.leo-lang.org/getting_started/installation).

This basic bank program can be run using the following bash script. Locally, it will execute Leo program functions to issue, deposit, and withdraw tokens between a bank and a user.

```bash
cd leo/examples/basic_bank
./run.sh
```

The `.env` file contains a private key and address. This is the account that will be used to sign transactions and is checked for record ownership. When executing programs as different parties, be sure to set the `private_key` field in `.env` to the appropriate value. You can check out how we have set things up in `./run.sh` for a full example of how to run the program as different parties.

## Walkthrough

- [Step 0: Issue Tokens](#issue)
- [Step 1: Deposit Tokens](#deposit)
- [Step 2: Wait](#wait)
- [Step 3: Withdraw Tokens](#withdraw)

## <a id="issue"></a> Issue Tokens

We will be playing the role of two parties.

```bash
The private key and address of the bank.
private_key: APrivateKey1zkpHtqVWT6fSHgUMNxsuVf7eaR6id2cj7TieKY1Z8CP5rCD
address: aleo1t0uer3jgtsgmx5tq6x6f9ecu8tr57rzzfnc2dgmcqldceal0ls9qf6st7a

The private key and address of the user.
private_key: APrivateKey1zkp75cpr5NNQpVWc5mfsD9Uf2wg6XvHknf82iwB636q3rtc
address: aleo1zeklp6dd8e764spe74xez6f8w27dlua3w7hl4z2uln03re52egpsv46ngg
```

Make some bank transactions. First, act as the bank. Issue 100 tokens to the user.
Put the bank's private key in `.env`. Run the `issue` function. Specify the recipient and the amount.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpHtqVWT6fSHgUMNxsuVf7eaR6id2cj7TieKY1Z8CP5rCD
" > .env

leo run issue aleo1zeklp6dd8e764spe74xez6f8w27dlua3w7hl4z2uln03re52egpsv46ngg 100u64
```

Output

```bash
 • {
  owner: aleo1zeklp6dd8e764spe74xez6f8w27dlua3w7hl4z2uln03re52egpsv46ngg.private,
  amount: 100u64.private,
  _nonce: 5747158428808897699391969939084459370750993398871840192272007071865455893612group.public
}
```

## <a id="deposit"></a> Deposit Tokens

Now, deposit 50 of the user's tokens with the bank. Act as the user. Call the `deposit` function.
Use the output record from the `issue` function. Specify the deposit amount.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp75cpr5NNQpVWc5mfsD9Uf2wg6XvHknf82iwB636q3rtc
" > .env

leo run deposit "{
    owner: aleo1zeklp6dd8e764spe74xez6f8w27dlua3w7hl4z2uln03re52egpsv46ngg.private,
    amount: 100u64.private,
    _nonce: 4668394794828730542675887906815309351994017139223602571716627453741502624516group.public
}"  50u64
```

Output

```bash
 • {
  owner: aleo1zeklp6dd8e764spe74xez6f8w27dlua3w7hl4z2uln03re52egpsv46ngg.private,
  amount: 50u64.private,
  _nonce: 832449386206374072274231152033740843999312028336559467119808470542606777523group.public
}
 • {
  program_id: basic_bank.aleo,
  function_name: deposit,
  arguments: [
    1197470102489602745811042362685620817855019264965533852603090875444599354527field,
    50u64
  ]
}
```

The output contains a new private record with 50 credits that belongs to the user.
It also contains the on-chain finalization code and its inputs.

## <a id="wait"></a> Wait

Assume that 15 periods pass after the 50-token deposit. The principal has a compound interest rate of 12.34 percent.

You can run the calculation yourself, it comes out to 266 tokens accrued using those numbers.

## <a id="withdraw"></a> Withdraw Tokens

After 15 periods, withdraw all tokens. Act as the bank. Call the `withdraw` function.
Specify the recipient's address, amount, rate, and number of periods.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpHtqVWT6fSHgUMNxsuVf7eaR6id2cj7TieKY1Z8CP5rCD
" > .env

leo run withdraw aleo1zeklp6dd8e764spe74xez6f8w27dlua3w7hl4z2uln03re52egpsv46ngg 50u64 1234u64 15u64
```

Output

```bash
 • {
  owner: aleo1zeklp6dd8e764spe74xez6f8w27dlua3w7hl4z2uln03re52egpsv46ngg.private,
  amount: 266u64.private,
  _nonce: 7051804730047578560256662070932795007350207323461845976313826737097831996144group.public
}
 • {
  program_id: basic_bank.aleo,
  function_name: withdraw,
  arguments: [
    1197470102489602745811042362685620817855019264965533852603090875444599354527field,
    50u64
  ]
}
```

The `withdraw` function creates a private record for the user with all 266 withdrawn tokens.
It also outputs the finalization data that runs on-chain.
