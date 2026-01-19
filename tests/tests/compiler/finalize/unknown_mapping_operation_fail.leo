
program test.aleo {

     mapping account: address => u64;

     async fn mint_public(public receiver: address, public amount: u64) {
         return finalize_mint_public(receiver, amount);
     }
}

 async fn finalize_mint_public (public receiver: address, public amount: u64) {
     let has_key: bool = Mapping::has_key(account, receiver);
     Mapping::set(account, receiver, amount);
 }
