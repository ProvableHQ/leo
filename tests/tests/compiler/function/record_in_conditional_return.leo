/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    record Token {
        owner: address,
        amount: u64
    }
    
    transition mint_token(owner: address, amount: u64, password: field) -> Token {
        // check if the password is correct
        let password_hash: field = 7202470996857839225873911078012225723419856133099120809866608931983814353616field;
        let password_hash_from_input: field = Poseidon2::hash_to_field(password);
    
        if(password_hash == password_hash_from_input){
            return Token {
                owner: owner,
                amount: amount
            };
        }
        else {
            return Token {
                owner: owner,
                amount: 0u64
            };
        }
    }
    
    transition mint_token2(owner: address, amount: u64, password: field) -> Token {
        // check if the password is correct
        let password_hash: field = 7202470996857839225873911078012225723419856133099120809866608931983814353616field;
        let password_hash_from_input: field = Poseidon2::hash_to_field(password);
    
        if(password_hash == password_hash_from_input){
            let token: Token = Token {
                owner: owner,
                amount: amount
            };
            return token;
        }
        return Token {
            owner: owner,
            amount: 0u64
        };
    }
}
