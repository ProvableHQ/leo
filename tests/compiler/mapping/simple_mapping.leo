/*
namespace: Compile
expectation: Pass
*/

mapping balances: address => u128;

circuit Token {
    owner: address;
    balance: u128;
}

mapping tokens: address => Token;
