         // The token program.
// Implements minting and transferring of tokens.
program test.aleo {
// A token record.
   // - `owner` : The token owner.
                    // - `amount` : The token amount.
    record token {
            owner: address,
    amount: u64,
        }

        /* Mint */

        // The fn `mint_public` issues the specified token amount
 // for the token receiver publicly on the network.
    fn mint_public(  public receiver:address ,  public amount:u64  ){
    return;
    }

/* Transfer */

             // Transfers tokens from one address to another.




    // This fn is public and both the sender and receiver

      // are visible on-chain.
        fn transfer_public(  public receiver:address  ,  public amount:u64  ){
        // Compute the change amount.
    // This `sub` operation is safe, and the proof will fail
                     // if an overflow occurs.
    let change:u64=amount-1u64;
                return;
    }
}
