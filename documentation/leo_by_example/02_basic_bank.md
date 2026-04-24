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
3. Upon a user's request to withdraw, the bank calculates the appropriate amount of compound interest and pays the user the principal and interest via the `withdraw` function.

Note that the program can be easily extended to include additional features such as a `transfer` function, which would allow users to transfer tokens to other users.

## Bugs

You may have already guessed that this program has a few bugs. We list some of them below:

- `withdraw` can only be invoked by the bank. A malicious bank could lock users' tokens by not invoking `withdraw`.
- `withdraw` fails if the sum of the interest and principal is greater than the user's balance.
- Users can increase their principal by depositing tokens multiple times, including immediately before withdrawal.
- Integer division rounds down; if the calculated interest is too small, then it will be rounded down to zero.

Can you find any others?

There are, of course, ways to write a version of this application without these bugs. This could be a good exercise for the reader.

## Language Features and Concepts

- `record` declarations
- `assert_eq`
- core functions, e.g. `BHP256::hash`
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

The `.env` file contains a private key and address. This is the account that will be used to sign transactions and is checked for record ownership. When executing programs as different parties, be sure to set the `private_key` field in `.env` to the appropriate value. You can check out how we've set things up in `./run.sh` for a full example of how to run the program as different parties.

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

Let's make some bank transactions. We'll take the role of the bank and issue 100 tokens to the user. We swap the private key into `.env` and run the `issue` function. The inputs are simply the recipient of the issuance and the amount.

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

Now, let's have the user deposit 50 of their tokens with the bank. We'll take the role of the user and call the deposit function, having the user use the output record that was issued to them by the bank. The inputs are the output record from the `issue` function and the amount the user wishes to deposit.

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

You'll see that the output contains a new private record belonging to the user with 50 credits, and finalization data indicating code to be run on-chain and its associated inputs.

## <a id="wait"></a> Wait

With the 50 token deposit, let's say 15 periods of time pass with compounding interest at a rate of 12.34% on the principal amount.

You can run the calculation yourself, it comes out to 266 tokens accrued using those numbers.

## <a id="withdraw"></a> Withdraw Tokens

Now, let's have the bank withdraw all tokens after 15 periods. Let's switch to the bank role, and call the `withdraw` function. The inputs are the recipient's address, amount, rate, and periods.

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

You'll see here the withdrawal function creates a new private record for the user containing all 266 withdrawn tokens, and then outputs finalization data which will be run on-chain.
