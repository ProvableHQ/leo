
program test.aleo {
    @test
    transition foo1() {
    }

    @test
    script foo2() {
    }

    @test(private_key = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
    transition foo3() {
    }

    @should_fail
    transition foo4() {
    }

    @should_fail
    @test
    transition foo5() {
    }

    @test(bogus_key = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH")
    transition foo6() {
    }

    @test
    @test
    script foo7() {
    }

    @test
    script foo8(x: u8) -> u8 {
        return x;
    }
}
