import distrofund_private_transfer.aleo;
import credits.aleo;

program simple.aleo {
   transition main(arg1: credits.aleo/credits, arg2:distrofund_private_transfer.aleo/Addresses1, arg3:distrofund_private_transfer.aleo/Amount1) -> (credits.aleo/credits.record, credits.aleo/credits.record) {
       return distrofund_private_transfer.aleo/transfer_one(arg1, arg2, arg3);
   }
}