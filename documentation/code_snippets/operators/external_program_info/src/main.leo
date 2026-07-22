import credits.aleo;

program external_program_info.aleo {
    fn read_external_info() -> Final {
        return final {
            // ANCHOR: external_checksum
            let ext_checksum: [u8; 32] = std::prog::checksum::[credits.aleo]();
            // ANCHOR_END: external_checksum

            // ANCHOR: external_edition
            let ext_edition: u16 = std::prog::edition::[credits.aleo]();
            // ANCHOR_END: external_edition

            // ANCHOR: external_program_owner
            let ext_owner: address = std::prog::program_owner::[credits.aleo]();
            // ANCHOR_END: external_program_owner
        };
    }

    @noupgrade
    constructor() {}
}
