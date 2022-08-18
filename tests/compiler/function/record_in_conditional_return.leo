/*
namespace: Compile
expectation: Pass
*/

record Token {
    owner: address,
    gates: u64,
    amount: u64
}

@program
function mint_token(owner: address, amount: u64, password: field) -> Token {
    // check if the password is correct
    let password_hash: field = 7202470996857839225873911078012225723419856133099120809866608931983814353616field;
    let password_hash_from_input: field = Poseidon2::hash(password);

    if(password_hash == password_hash_from_input){
        return Token {
            owner: owner,
            gates: 0u64,
            amount: amount
        };
    }
    else {
        return Token {
            owner: owner,
            gates: 0u64,
            amount: 0u64
        };
    }
}
