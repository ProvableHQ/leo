/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Returns the number of flipped bits.
    // E.g. 17870283321406128128u64, in binary 11111000 00000000 00000000 00000000 00000000 00000000 00000000 00000000,
    // returns 5u64;
    function bitcount(bits: u64) -> u64 {
        let r1: u64 = bits / 2u64;
        let r2: u64 = bits / 4u64;
        let r3: u64 = bits / 8u64;

        let r4: u64 = r1 & 8608480567731124087u64;
        let r5: u64 = r2 & 3689348814741910323u64;
        let r6: u64 = r3 & 1229782938247303441u64;

        let r7: u64 = bits - r4 - r5 - r6;

        let r8: u64 = r7 / 16u64;
        let r9: u64 = r7 + r8;
        let r10: u64 = r9 & 1085102592571150095u64;
        let r11: u64 = r10 % 255u64;

        return r11;
    }

    // Returns boolean of whether all the flipped bits in location are "adjacent". Horizontally, this means all flipped bits are
    // directly next to each other (111). Vertically, this means all flipped bits are separated by 7 unflipped bits
    // (10000000100000001).
    function adjacency_check(
        // The u64 representation of a ship's placement in an 8x8 grid.
        ship: u64,
        // The u64 representation of a ship's bitstring, either horizontally or vertically.
        // E.g. a ship of length 3's bit string horizontally would be: 000111 = 7u64. Vertically, the bit string would be:
        // 10000000100000001 = 65793u64.
        orientation: u64,
    ) -> bool {
        // This may result in 0.
        let division: u64 = ship / orientation;

        // subtracting 1 from 0 will cause an underflow, so we should check for this edge case.
        let is_eq: bool = division == 0u64;

        // if the above division resulted in 0, we know the adjacency check should return false.
        // Setting to r4 to 3 (11) will guarantee failure here.
        let ternary: u64 = is_eq ? 3u64 : division;
        let subtraction: u64 = ternary - 1u64;
        let and: u64 = subtraction & ternary;

        let bits_are_adjacent: bool = and == 0u64;

        return bits_are_adjacent;
    }

    // Returns boolean of whether adjacent flipped bits don't split a row of size 8.
    // E.g. 111000000 has adjacent flipped bits but splits a row: 00000001 11000000
    function horizontal_check(
        // The u64 representation of a ship's placement in an 8x8 grid.
        ship: u64,
        // The u64 representation of a ship's bitstring horizontally.
        horizontal: u64,
    ) -> bool {
        let remainder: u64 = ship % 255u64;
        // This may result in 0.
        let division: u64 = remainder / horizontal;

        // Subtracting 1 from 0 will cause an underflow.
        let is_eq: bool = division == 0u64;

        // Setting to 3 will guarantee failure.
        let ternary: u64 = is_eq ? 3u64 : division;
        let subtraction: u64 = ternary - 1u64;
        let and: u64 = subtraction & ternary;

        let bits_split_row: bool = and == 0u64;

        return bits_split_row;
    }

    // Returns `true` if the ship placement is valid.
    transition validate_ship(
        // The u64 representation of a ship's placement in an 8x8 grid.
        ship: u64,
        // The length of the placed ship.
        length: u64,
        // The u64 equivalent of a ship's horizontal bitstring representation.
        horizontal: u64,
        // The u64 equivalent of a ship's vertical bitstring representation.
        vertical: u64,
    ) -> bool {
        // Check bitcount -- all other validations depend on the bitcount being correct.
        let num_bits: u64 = bitcount(ship);
        assert_eq(num_bits, length);

        // Check horizontal bits of ship.
        let is_adjacent: bool = adjacency_check(ship, horizontal); // True if bits are adjacent horizontally.
        let is_horizontal: bool = horizontal_check(ship, horizontal); // True if those horizontal bits are not split across rows.
        let valid_horizontal: bool = is_adjacent && is_horizontal; // True if bits are adjacent horizontally and not split across rows.

        // Check vertical bits of ship.
        let valid_vertical: bool = adjacency_check(ship, vertical); // True if bits are adjacent vertically.

        let ship_is_valid: bool = valid_horizontal || valid_vertical; // Ship is valid if it is vertically or horizontally valid.

        return ship_is_valid;
    }

    // Returns the u64 representation of all the ships' placements in an 8x8 grid. This function will fail
    // if any of the ship placements overlap each other.
    transition create_board(
        // The u64 representation of a carrier's placement in an 8x8 grid. Length = 5.
        carrier: u64,
        // The u64 representation of a battleship's placement in an 8x8 grid. Length = 4.
        battleship: u64,
        // The u64 representation of a cruiser's placement in an 8x8 grid. Length = 3.
        cruiser: u64,
        // The u64 representation of a destroyer's placement in an 8x8 grid. Length = 2.
        destroyer: u64,
    ) -> u64 {
        // Bitwise combine the ship placements together
        let ships: u64 = carrier | battleship | cruiser | destroyer;

        let num_bits: u64 = bitcount(ships);
        assert_eq(num_bits, 14u64); // Given 4 individually-valid ships, a valid combination should yield exactly 14 flipped bits.

        return ships;
    }
}
