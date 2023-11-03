import artgo_aigc_v1_3.aleo;
import credits.aleo;

program transfer.aleo {

   mapping balances: u64 => u64;

   transition main(arg1: field, arg2: u64, arg3: address) ->  artgo_aigc_v1_3.aleo/BidRecord {
       return artgo_aigc_v1_3.aleo/bid_order(arg1, arg2, arg3) then finalize();
   }
   finalize main() {
         Mapping::set(balances, 1u64, 1u64);
    }
}