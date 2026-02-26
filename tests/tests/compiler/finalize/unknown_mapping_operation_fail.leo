
program test.aleo {

     mapping account: address => u64;

     fn mint_public(public receiver: address, public amount: u64) {
         return final { finalize_mint_public(receiver, amount); };
     }
}

 final fn finalize_mint_public (public receiver: address, public amount: u64) {
     let has_key: bool = Mapping::has_key(account, receiver);
     Mapping::set(account, receiver, amount);
 }
