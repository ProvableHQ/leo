/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    inline get_bit(number: u32, i: u8) -> bool {
        let mask: u32 = 1u32 << i;
        return number.and(mask) > 0u32;
    }

    inline get_cell_occupant(board: u32, i: u8) -> u8 {
        return get_bit(board, i) ? 2u8 : 1u8;
    }

    transition make_move(board: u32, i: u8) -> u8 {
        return get_cell_occupant(board, i);
    }
}
