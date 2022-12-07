/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(id_type: u8) -> bool {
        if (id_type == 1u8) {
            return true;
        }
        if (id_type == 2u8) {
            assert(0u8 > id_type);
            return false;
        }
        return false;
    }
}
