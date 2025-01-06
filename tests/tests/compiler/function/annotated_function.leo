/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {    
    @test
    transition test() {
        assert(1u32 == 1u32);
    }

    @test(
      private_key = "APrivateKey1zkp9wLdmfcM57QFL3ZEzgfZwCWV52nM24ckmLSmTQcp64FL",
      batch = "0",
      seed = "1234"
    )
    transition test_other() {
        assert(1u32 == 1u32);
    }
}
