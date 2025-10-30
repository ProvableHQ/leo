program metadata.aleo {

    async transition is_block_height(block_height: u32) -> Future {
        return finalize_is_block_height(block_height);
    }

    async function finalize_is_block_height(block_height: u32) {
        assert_eq(block_height, block.height);
    }

    async transition is_network_id(network_id: u16) -> Future {
        return finalize_is_network_id(network_id);
    }

    async function finalize_is_network_id(network_id: u16) {
        assert_eq(network_id, network.id);
    }

    @custom
    async constructor() {
        assert_eq(true, true);
    }
}

// --- Test --- //

import metadata.aleo;

program test_metadata.aleo {

    @test
    @should_fail
    async transition is_block_height_2() -> Future {
        let f = metadata.aleo/is_block_height(2u32);
        return async {
            f.await();
        };
    }

    @test
    @should_fail
    async transition is_block_height_3() -> Future {
        let f = metadata.aleo/is_block_height(3u32);
        return async {
            f.await();
        };
    }

    @test
    @should_fail
    async transition is_block_height_4() -> Future {
        let f = metadata.aleo/is_block_height(4u32);
        return async {
            f.await();
        };
    }

    @test
    @should_fail
    async transition is_block_height_0() -> Future {
        let f = metadata.aleo/is_block_height(0u32);
        return async {
            f.await();
        };
    }

    @test
    @should_fail
    async transition is_network_id_0() -> Future {
        let f = metadata.aleo/is_network_id(0u16);
        return async {
            f.await();
        };
    }

    @test
    async transition is_network_id_1() -> Future {
        let f = metadata.aleo/is_network_id(1u16);
        return async {
            f.await();
        };
    }

    @test
    @should_fail
    async transition is_network_id_2() -> Future {
        let f = metadata.aleo/is_network_id(2u16);
        return async {
            f.await();
        };
    }

    @test
    @should_fail
    async transition is_network_id_3() -> Future {
        let f = metadata.aleo/is_network_id(3u16);
        return async {
            f.await();
        };
    }

    @noupgrade
    async constructor() {}
}
