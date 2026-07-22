program test.aleo {
    final fn program_checksum_sugar() -> [u8; 32] {
        return Program::checksum::[credits.aleo]();
    }
    final fn program_edition_sugar() -> u16 {
        return Program::edition::[credits.aleo]();
    }
    final fn program_owner_sugar() -> address {
        return Program::program_owner::[credits.aleo]();
    }
    final fn function_checksum_sugar() -> [u8; 32] {
        return Program::function_checksum::[credits.aleo, transfer_public]();
    }
}
