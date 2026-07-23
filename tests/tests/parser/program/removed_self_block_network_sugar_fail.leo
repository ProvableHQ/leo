program test.aleo {
    fn caller_sugar() -> address {
        return self.caller;
    }
    fn signer_sugar() -> address {
        return self.signer;
    }
    fn address_sugar() -> address {
        return self.address;
    }
    fn id_sugar() -> address {
        return self.id;
    }
    fn checksum_sugar() -> [u8; 32] {
        return self.checksum;
    }
    fn edition_sugar() -> u16 {
        return self.edition;
    }
    fn owner_sugar() -> address {
        return self.program_owner;
    }
    final fn block_height_sugar() -> u32 {
        return block.height;
    }
    final fn block_timestamp_sugar() -> i64 {
        return block.timestamp;
    }
    final fn network_id_sugar() -> u16 {
        return network.id;
    }
    fn bare_self() {
        let s = self;
        return;
    }
    fn bare_block() {
        let b = block;
        return;
    }
    fn bare_network() {
        let n = network;
        return;
    }
}
