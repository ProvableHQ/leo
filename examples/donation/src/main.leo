// The 'donation' program.
program donation.aleo {
     mapping balances: field => u64;
    
    record user_record {
        owner: address;
        amount: u64;
        id_assigned: bool;
        user: address;
       
    }
   

record Token {
    owner: address,
    amount: u64,
    
}
 function only_owner (owner: address) -> address {
    
  assert_eq(self.caller, owner );
  return owner;
 }

 transition donate(token: Token, amount: u64) -> Token {
        let difference: u64 = token.amount - amount;

        let remaining: Token = Token {
            owner: token.owner,
            amount: difference,
        };
         let hash: field = BHP256::hash_to_field(token.owner);
         return remaining then finalize(hash, amount);


 }
 

 finalize donate(hash: field, amount: u64) {
        let current_amount: u64 = Mapping::get_or_use(balances, hash, 0u64);
        Mapping::set(balances, hash, current_amount + amount);
}
 
 transition ajo ( user: address, amount: u64) -> user_record {
        assert_eq(self.caller,user );
       
        return user_record {
            owner: aleo1fxs9s0w97lmkwlcmgn0z3nuxufdee5yck9wqrs0umevp7qs0sg9q5xxxzh,
            amount: amount,
            id_assigned: true,
            user: user,
          };
 }

}
