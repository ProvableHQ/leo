# donation.aleo

# Donation Program

The Donation program is a smart contract written in Aleo that facilitates token donations and manages user records. Users can donate tokens to the contract, and their donation amounts are recorded in a mapping.

## Features

- Token donation functionality
- User record management
- Hashing functionality for token finalization

## Smart Contract Structure

### Data Structures

- `Token`: Represents a token with an owner and an amount.
- `user_record`: Represents a user record with the owner's address, the amount donated, whether an ID has been assigned, and the user's address.

### Functions

- `donate`: Allows users to donate tokens to the contract.
- `finalize`: Finalizes the token donation by updating the token's amount in the mapping.
- `ajo`: Manages user records and assigns an ID to the user.

## Usage

1. **Deploying the Contract**: Deploy the smart contract to the Aleo blockchain.
2. **Donating Tokens**: Users can donate tokens to the contract by calling the `donate` transition and providing the token and amount.
3. **Managing User Records**: Users can manage their records by calling the `ajo` transition and providing their address and donation amount.

## Example Usage

```aleo
transition donate(token: Token, amount: u64) -> Token {
    // Implementation details
}

transition ajo(user: address, amount: u64) -> user_record {
    // Implementation details
}

## Build Guide

To compile this Aleo program, run:
```bash
snarkvm build
```

To execute this Aleo program, run:
```bash
snarkvm run hello
```
