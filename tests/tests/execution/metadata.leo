/*
seed = 123456789
min_height = 16

[case]
program = "metadata.aleo"
function = "is_block_height"
input = ["2u32"]
[case]
program = "metadata.aleo"
function = "is_block_height"
input = ["3u32"]
[case]
program = "metadata.aleo"
function = "is_block_height"
input = ["4u32"]
[case]
program = "metadata.aleo"
function = "is_block_height"
input = ["0u32"]
[case]
program = "metadata.aleo"
function = "is_network_id"
input = ["0u16"]
[case]
program = "metadata.aleo"
function = "is_network_id"
input = ["1u16"]
[case]
program = "metadata.aleo"
function = "is_network_id"
input = ["2u16"]
[case]
program = "metadata.aleo"
function = "is_network_id"
input = ["3u16"]
*/

final fn finalize_is_block_height(block_height: u32) {
    assert_eq(block_height, block.height);
}

final fn finalize_is_network_id(network_id: u16) {
    assert_eq(network_id, network.id);
}

program metadata.aleo {
    fn is_block_height(block_height: u32) -> Final {
        return final { finalize_is_block_height(block_height); };
    }

    fn is_network_id(network_id: u16) -> Final {
        return final { finalize_is_network_id(network_id); };
    }

    @custom
    constructor() {
        assert_eq(true, true);
    }
}
