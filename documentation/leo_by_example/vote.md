---
id: vote
title: A Voting Program using Leo
---

[general tags]: # "example, vote, record, program, mapping"

**[Source Code](https://github.com/ProvableHQ/leo-examples/tree/main/vote)**

## Summary

`vote.leo` is a general vote program.

Anyone can issue new proposals, proposers can issue tickets to the voters, and voters can vote without exposing their identity.

This example is inspired by the [aleo-vote](https://github.com/zkprivacy/aleo-vote) example written by the Aleo community.

## Noteworthy Features

Voter identity is concealed by privately passing a voter's ballot into a function.
Proposal information and voting results are revealed using the public `mapping` datatype in Leo.

## How to Run

Follow the [Leo Installation Instructions](https://docs.leo-lang.org/getting_started/installation).

This vote program can be run using the following bash script. Locally, it will execute Leo program functions to create proposals, create tickets, and make votes.

```bash
cd leo/examples/vote
./run.sh
```

The `.env` file contains a private key and network type. This is the account that will be used to sign transactions and is checked for record ownership. When executing programs as different parties, be sure to set the `private_key` field in `.env` to the appropriate value. You can check out how we've set things up in `./run.sh` for a full example of how to run the program as different parties.

## Walkthrough

- [Functions](#functions)
- [Step 0: Create a Proposal](#step0)
- [Step 1: Voter 1 issues a ticket and makes a vote](#step1)
- [Step 2: Voter 2 issues a ticket and makes a vote](#step2)
- [Step 3: How votes are tallied](#step3)

## <a id="functions"></a> Functions

### Propose

Anyone can issue a new proposal publicly by calling the `propose` function.

### Create Ticket

Proposers can create new tickets for proposals.

A ticket is a record with an `owner` and a proposal id `pid`. A ticket can be used to vote for a proposal identified by `pid`; it **can only be used by the ticket owner**. That is, **only the owner can use that `ticket` to cast a vote**.

### Vote

A ticket owner can use their ticket record to vote `agree` / `disagree` with the specific proposal - `pid`. Since the ticket record can be used as an input privately, the voter's privacy is protected.

## <a id="step0"></a> Create a Proposal

We will be playing the role of three parties.

```bash
The private key and address of the proposer.
private_key: APrivateKey1zkp8wKHF9zFX1j4YJrK3JhxtyKDmPbRu9LrnEW8Ki56UQ3G
address: aleo1rfez44epy0m7nv4pskvjy6vex64tnt0xy90fyhrg49cwe0t9ws8sh6nhhr

The private key and address of voter 1.
private_key: APrivateKey1zkpHmSu9zuhyuCJqVfQE8p82HXpCTLVa8Z2HUNaiy9mrug2
address: aleo1c45etea8czkyscyqawxs7auqjz08daaagp2zq4qjydkhxt997q9s77rsp2

The private key and address of voter 2.
private_key: APrivateKey1zkp6NHwbT7PkpnEFeBidz5ZkZ14W8WXZmJ6kjKbEHYdMmf2
address: aleo1uc6jphye8y9gfqtezrz240ak963sdgugd7s96qpuw6k7jz9axs8q2qnhxc
```

Let's propose a new ballot. Take on the role of the proposer and run the `propose` function. We've provided the necessary information as inputs to the `propose` function.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp8wKHF9zFX1j4YJrK3JhxtyKDmPbRu9LrnEW8Ki56UQ3G
" > .env

leo run propose "{
  title: 2077160157502449938194577302446444field,
  content: 1452374294790018907888397545906607852827800436field,
  proposer: aleo1rfez44epy0m7nv4pskvjy6vex64tnt0xy90fyhrg49cwe0t9ws8sh6nhhr
}"
```

Output

```bash

 • {
  owner: aleo1rfez44epy0m7nv4pskvjy6vex64tnt0xy90fyhrg49cwe0t9ws8sh6nhhr.private,
  id: 2805252584833208809872967597325381727971256629741137995614832105537063464740field.private,
  info: {
    title: 2077160157502449938194577302446444field.private,
    content: 1452374294790018907888397545906607852827800436field.private,
    proposer: aleo1rfez44epy0m7nv4pskvjy6vex64tnt0xy90fyhrg49cwe0t9ws8sh6nhhr.private
  },
  _nonce: 7270749279509948287724447377218313625054368902761257869085068499107406906985group.public
}
 • {
  program_id: vote.aleo,
  function_name: propose,
  arguments: [
    2805252584833208809872967597325381727971256629741137995614832105537063464740field
  ]
}
```

You'll see that the output generates a new record with the proposal information and sets a public mapping with the proposal id as an argument input. The public mapping will be queryable on-chain.

## <a id="step1"></a> Voter 1 makes a vote

Let's create a new private ticket to make a vote. Take on the role of voter 1 and run the `new_ticket` function. The inputs take a unique ticket ID and the voter's public address.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkpHmSu9zuhyuCJqVfQE8p82HXpCTLVa8Z2HUNaiy9mrug2
" > .env

leo run new_ticket 2264670486490520844857553240576860973319410481267184439818180411609250173817field aleo1c45etea8czkyscyqawxs7auqjz08daaagp2zq4qjydkhxt997q9s77rsp2
```

Output

```bash
 • {
  owner: aleo1c45etea8czkyscyqawxs7auqjz08daaagp2zq4qjydkhxt997q9s77rsp2.private,
  pid: 2264670486490520844857553240576860973319410481267184439818180411609250173817field.private,
  _nonce: 3111099913449740827888947259874663727415985369111767658428258317443300847451group.public
}
 • {
  program_id: vote.aleo,
  function_name: new_ticket,
  arguments: [
    2264670486490520844857553240576860973319410481267184439818180411609250173817field
  ]
}
```

You'll see a new private ticket created belonging to the owner, and a public mapping in the vote program to track the ID of that ticket.

Voter 1 can now vote privately on their ticket. Call the `agree` or `disagree` function, which takes the voter's ticket output as the input.

```bash
leo run agree "{
  owner: aleo1c45etea8czkyscyqawxs7auqjz08daaagp2zq4qjydkhxt997q9s77rsp2.private,
  pid: 2264670486490520844857553240576860973319410481267184439818180411609250173817field.private,
  _nonce: 1738483341280375163846743812193292672860569105378494043894154684192972730518group.public
}"
```

Output

```bash
 • {
  program_id: vote.aleo,
  function_name: agree,
  arguments: [
    2264670486490520844857553240576860973319410481267184439818180411609250173817field
  ]
}

```

## <a id="step2"></a> Voter 2 makes a vote

Let's create a new private ticket for voter 2. Take on the role of voter 2 and run the `new_ticket` function. The inputs take a unique ticket ID and the voter's public address.

```bash
echo "
NETWORK=testnet
PRIVATE_KEY=APrivateKey1zkp6NHwbT7PkpnEFeBidz5ZkZ14W8WXZmJ6kjKbEHYdMmf2
" > .env

leo run new_ticket 2158670485494560943857353240576760973319410481267184429818180411607250143681field aleo1uc6jphye8y9gfqtezrz240ak963sdgugd7s96qpuw6k7jz9axs8q2qnhxc
```

Output

```bash
 • {
  owner: aleo1uc6jphye8y9gfqtezrz240ak963sdgugd7s96qpuw6k7jz9axs8q2qnhxc.private,
  pid: 2158670485494560943857353240576760973319410481267184429818180411607250143681field.private,
  _nonce: 7213678168429828883374086447188635180072431460350128753904256765278199909612group.public
}
 • {
  program_id: vote.aleo,
  function_name: new_ticket,
  arguments: [
    2158670485494560943857353240576760973319410481267184429818180411607250143681field
  ]
}
```

Voter 2 can now vote privately on their ticket. Call the `agree` or `disagree` function, which takes the voter's ticket output as the input.

```bash
leo run disagree "{
  owner: aleo1uc6jphye8y9gfqtezrz240ak963sdgugd7s96qpuw6k7jz9axs8q2qnhxc.private,
  pid: 2158670485494560943857353240576760973319410481267184429818180411607250143681field.private,
  _nonce: 6511154004161574129036815174288926693337549214513234790975047364416273541105group.public
}"
```

Output

```bash
 • {
  program_id: vote.aleo,
  function_name: disagree,
  arguments: [
    2158670485494560943857353240576760973319410481267184429818180411607250143681field
  ]
}
```

## <a id="step3"></a> How votes are tallied

Votes on the ticket are private. But the sum total of the agreements and disagreements are shown on-chain in the public mapping. You can query this data on-chain.
