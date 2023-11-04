import credits.aleo;

program super_simple.aleo {
    transition main(arg1: credits.aleo/credits, arg2: address, arg3: u64) -> (credits.aleo/credits.record, credits.aleo/credits.record) {
        return credits.aleo/transfer_private(arg1, arg2, arg3);
    }
}