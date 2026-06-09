---
id: token
title: A Custom Token in Leo
---

[general tags]: # "example, token, record, program"

**[Source Code](https://github.com/ProvableHQ/leo-examples/tree/main/token)**

## Summary

A transparent & shielded custom token in Leo.

## How to Run

Follow the [Leo Installation Instructions](https://docs.leo-lang.org/getting_started/installation).

This token program can be run using the following bash script. Locally, it will execute Leo program functions to mint and transfer tokens publicly and privately.

```bash
cd leo/examples/token
./run.sh
```

The `.env` file contains a private key and network type. This is the account that will be used to sign transactions and is checked for record ownership. When executing programs as different parties, be sure to set the `private_key` field in `.env` to the appropriate value. You can check out how we've set things up in `./run.sh` for a full example of how to run the program as different parties.

## Walkthrough

- [Step 0: Public Mint](#step0)
- [Step 1: Private Mint](#step1)
- [Step 2: Public Transfer](#step2)
- [Step 3: Private Transfer](#step3)
- [Step 4: Public to Private Transfer](#step4)
- [Step 5: Private to Public Transfer](#step5)

We'll be conducting a transfer between two parties.

```bash
The private key and address of Alice.
private_key: APrivateKey1zkp1w8PTxrRgGfAtfKUSq43iQyVbdQHfhGbiNPEg2LVSEXR
address: aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q

The private key and address of Bob.
private_key: APrivateKey1zkpFo72g7N9iFt3JzzeG8CqsS5doAiXyFvNCgk2oHvjRCzF
address: aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z
```

## <a id="step0"></a> Public Mint

Let's play Alice. Swap in her private key and publicly mint 100 tokens.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp1w8PTxrRgGfAtfKUSq43iQyVbdQHfhGbiNPEg2LVSEXR
" > .env

leo run mint_public aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q 100u64
```

Output

```bash
 • {
  program_id: token.aleo,
  function_name: mint_public,
  arguments: [
    aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q,
    100u64
  ]
}
```

You can see the output of `mint_public`, which takes the arguments Alice's address and the amount of tokens to mint publicly. This information is shown on-chain and can be queried on a network.

## <a id="step1"></a> Private Mint

Now let's privately mint 100 tokens for Bob. Switch to Bob's private key and privately mint 100 tokens for Bob.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpFo72g7N9iFt3JzzeG8CqsS5doAiXyFvNCgk2oHvjRCzF
" > .env

leo run mint_private aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z 100u64
```

Output

```bash
 • {
  owner: aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z.private,
  amount: 100u64.private,
  _nonce: 4719474923967087502681846187174640869781874305919806595754990568074403149805group.public
}
```

The output is a private record.

## <a id="step2"></a> Public Transfer

Let's publicly transfer 10 tokens from Alice to Bob. Swap the private key back to Alice and call the public transfer function.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp1w8PTxrRgGfAtfKUSq43iQyVbdQHfhGbiNPEg2LVSEXR
" > .env

leo run transfer_public aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z 10u64
```

Output

```bash
 • {
  program_id: token.aleo,
  function_name: transfer_public,
  arguments: [
    aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q,
    aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z,
    10u64
  ]
}
```

Again, we see the arguments used for the `final` block of `transfer_public` - Alice's address, Bob's address, and the amount to transfer. The public mapping will be queryable on-chain.

## <a id="step3"></a> Private Transfer

Let's privately transfer 20 tokens from Bob to Alice. Switch to Bob's private key and call the private transfer function.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpFo72g7N9iFt3JzzeG8CqsS5doAiXyFvNCgk2oHvjRCzF
" > .env

leo run transfer_private "{
    owner: aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z.private,
    amount: 100u64.private,
    _nonce: 6586771265379155927089644749305420610382723873232320906747954786091923851913group.public
}" aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q 20u64
```

Output

```bash
 • {
  owner: aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z.private,
  amount: 80u64.private,
  _nonce: 7402942372117092417133095075129616994719981532373540395650657400913787695842group.public
}
 • {
  owner: aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q.private,
  amount: 20u64.private,
  _nonce: 2444690320093734417295179000152972034731859256625211879727315719617371330248group.public
}
```

The output of `transfer_private` is a record owned by Bob less the 20 tokens he privately transferred to Alice, and a record owned by Alice with the 20 tokens Bob transferred to Alice.

## <a id="step4"></a> Public to Private Transfer

Let's convert 30 of Alice's public tokens into 30 private tokens for Bob. Switch the private key back to Alice.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp1w8PTxrRgGfAtfKUSq43iQyVbdQHfhGbiNPEg2LVSEXR
" > .env

leo run transfer_public_to_private aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z 30u64
```

Output

```bash
 • {
  owner: aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z.private,
  amount: 30u64.private,
  _nonce: 2372167793514585424629802909684994302673167688345985265672131682042636755887group.public
}
 • {
  program_id: token.aleo,
  function_name: transfer_public_to_private,
  arguments: [
    aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q,
    30u64
  ]
}
```

Calling `transfer_public_to_private` outputs finalization data, which indicates code to be run on-chain, along with its associated inputs.

## <a id="step5"></a> Private to Public Transfer

Let's convert 40 of Bob's private tokens into 40 public tokens for Alice. Switch the private key back to Bob.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpFo72g7N9iFt3JzzeG8CqsS5doAiXyFvNCgk2oHvjRCzF
" > .env

leo run transfer_private_to_public "{
    owner: aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z.private,
    amount: 80u64.private,
    _nonce: 1852830456042139988098466781381363679605019151318121788109768539956661608520group.public
}" aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q 40u64
```

Output

```bash
 • {
  owner: aleo17vy26rpdhqx4598y5gp7nvaa9rk7tnvl6ufhvvf4calsrrqdaqyshdsf5z.private,
  amount: 40u64.private,
  _nonce: 2233440107615267344685761424001099994586652279869516904008515754794838882197group.public
}
 • {
  program_id: token.aleo,
  function_name: transfer_private_to_public,
  arguments: [
    aleo13ssze66adjjkt795z9u5wpq8h6kn0y2657726h4h3e3wfnez4vqsm3008q,
    40u64
  ]
}
```

When we call `transfer_private_to_public`, we take Bob's private record that contains 110 tokens, and outputs a record owned by Bob with 70 tokens, and then outputs finalization data which will be run on-chain.
