program standard_ops_demo.aleo {
    fn arithmetic_demos() -> u8 {
        {
            // ANCHOR: abs
            let a: i8 = -1i8;
            let b: i8 = a.abs(); // 1i8
            // ANCHOR_END: abs
        }
        {
            // ANCHOR: abs_wrapped
            let a: i8 = -128i8;
            let b: i8 = a.abs_wrapped(); // -128i8
            // ANCHOR_END: abs_wrapped
        }
        {
            // ANCHOR: add
            let a: u8 = 1u8;
            let b: u8 = a + 1u8; // 2u8
            let c: u8 = b.add(1u8); // 3u8
            // ANCHOR_END: add
        }
        {
            // ANCHOR: add_wrapped
            let a: u8 = 255u8;
            let b: u8 = a.add_wrapped(1u8); // 0u8
            // ANCHOR_END: add_wrapped
        }
        {
            // ANCHOR: div
            let a: u8 = 4u8;
            let b: u8 = a / 2u8; // 2u8
            let c: u8 = b.div(2u8); // 1u8
            // ANCHOR_END: div
        }
        {
            // ANCHOR: div_wrapped
            let a: i8 = -128i8;
            let b: i8 = a.div_wrapped(-1i8); // -128i8
            // ANCHOR_END: div_wrapped
        }
        {
            // ANCHOR: mod_op
            let a: u8 = 3u8.mod(2u8); // 1u8
            // ANCHOR_END: mod_op
        }
        {
            // ANCHOR: mul
            let a: u8 = 2u8 * 2u8; // 4u8
            let b: u8 = a.mul(2u8); // 8u8
            // ANCHOR_END: mul
        }
        {
            // ANCHOR: mul_wrapped
            let a: u8 = 128u8.mul_wrapped(2u8); // 0u8
            // ANCHOR_END: mul_wrapped
        }
        {
            // ANCHOR: neg
            let a: i8 = (-1i8).neg(); // 1i8
            // ANCHOR_END: neg
        }
        {
            // ANCHOR: pow
            let a: u8 = 2u8 ** 2u8; // 4u8
            let b: u8 = a.pow(2u8); // 16u8
            // ANCHOR_END: pow
        }
        {
            // ANCHOR: pow_wrapped
            let a: u8 = 16u8.pow_wrapped(2u8); // 0u8
            // ANCHOR_END: pow_wrapped
        }
        {
            // ANCHOR: rem
            let a: u8 = 3u8 % 2u8; // 1u8
            let b: u8 = 4u8.rem(2u8); // 0u8
            // ANCHOR_END: rem
        }
        {
            // ANCHOR: rem_wrapped
            let a: i8 = -128i8;
            let b: i8 = a.rem_wrapped(-1i8); // 0i8
            // ANCHOR_END: rem_wrapped
        }
        {
            // ANCHOR: sub
            let a: u8 = 2u8 - 1u8; // 1u8
            let b: u8 = a.sub(1u8); // 0u8
            // ANCHOR_END: sub
        }
        {
            // ANCHOR: sub_wrapped
            let a: u8 = 0u8.sub_wrapped(1u8); // 255u8
            // ANCHOR_END: sub_wrapped
        }
        return 0u8;
    }

    fn boolean_bitwise_demos() -> bool {
        {
            // ANCHOR: and_op
            // Integer (bitwise) AND
            let a: i8 = 1i8 & 1i8;
            let b: i8 = 1i8.and(2i8);

            // Boolean (logical) AND
            let c: bool = true && true;
            let d: bool = true.and(false);
            // ANCHOR_END: and_op
        }
        {
            // ANCHOR: nand_op
            let a: bool = true.nand(false); // true
            // ANCHOR_END: nand_op
        }
        {
            // ANCHOR: nor_op
            let a: bool = false.nor(false); // true
            // ANCHOR_END: nor_op
        }
        {
            // ANCHOR: not_op
            let a: bool = true.not(); // false
            // ANCHOR_END: not_op
        }
        {
            // ANCHOR: or_op
            // Integer (bitwise) OR
            let a: i8 = 1i8 | 2i8;
            let b: i8 = 1i8.or(2i8);

            // Boolean (logical) OR
            let c: bool = true || true;
            let d: bool = true.or(false);
            // ANCHOR_END: or_op
        }
        {
            // ANCHOR: shl
            let a: u8 = 1u8 << 1u8; // 2u8
            let b: u8 = a.shl(1u8); // 4u8
            // ANCHOR_END: shl
        }
        {
            // ANCHOR: shl_wrapped
            let a: u8 = 128u8.shl_wrapped(1u8); // 0u8
            let b: i8 = 64i8.shl_wrapped(2u8); // -128i8
            // ANCHOR_END: shl_wrapped
        }
        {
            // ANCHOR: shr
            let a: u8 = 4u8 >> 1u8; // 2u8
            let b: u8 = a.shr(1u8); // 1u8
            // ANCHOR_END: shr
        }
        {
            // ANCHOR: shr_wrapped
            let a: u8 = 128u8.shr_wrapped(7u8); // 1u8
            // ANCHOR_END: shr_wrapped
        }
        {
            // ANCHOR: xor_op
            let a: bool = true.xor(false); // true
            // ANCHOR_END: xor_op
        }
        return false;
    }

    fn comparator_demos() -> bool {
        {
            // ANCHOR: gt
            let a: bool = 2u8 > 1u8; // true
            let b: bool = 1u8.gt(1u8); // false
            // ANCHOR_END: gt
        }
        {
            // ANCHOR: gte
            let a: bool = 2u8 >= 1u8; // true
            let b: bool = 1u8.gte(1u8); // true
            // ANCHOR_END: gte
        }
        {
            // ANCHOR: eq
            let a: bool = 1u8 == 1u8; // true
            let b: bool = 1u8.eq(2u8); // false
            // ANCHOR_END: eq
        }
        {
            // ANCHOR: neq
            let a: bool = 1u8 != 1u8; // false
            let b: bool = 1u8.neq(2u8); // true
            // ANCHOR_END: neq
        }
        {
            // ANCHOR: lt
            let a: bool = 1u8 < 2u8; // true
            let b: bool = 1u8.lt(1u8); // false
            // ANCHOR_END: lt
        }
        {
            // ANCHOR: lte
            let a: bool = 1u8 <= 2u8; // true
            let b: bool = 1u8.lte(1u8); // true
            // ANCHOR_END: lte
        }
        return false;
    }

    fn group_field_demos() -> field {
        {
            // ANCHOR: group_gen
            let g: group = group::GEN; // the group generator
            // ANCHOR_END: group_gen
        }
        {
            // ANCHOR: double
            let a: group = 2group;
            let b: group = a.double();
            // ANCHOR_END: double
        }
        {
            // ANCHOR: inv
            let a: field = 1field.inv();
            // ANCHOR_END: inv
        }
        {
            // ANCHOR: square
            let a: field = 1field.square(); // 1field
            // ANCHOR_END: square
        }
        {
            // ANCHOR: square_root
            let a: field = 1field.square_root(); // 1field
            // ANCHOR_END: square_root
        }
        {
            // ANCHOR: to_x_coordinate
            let x: field = 0group.to_x_coordinate(); // 0field
            // ANCHOR_END: to_x_coordinate
        }
        {
            // ANCHOR: to_y_coordinate
            let y: field = 0group.to_y_coordinate(); // 1field
            // ANCHOR_END: to_y_coordinate
        }
        {
            // ANCHOR: aleo_generator
            let g: group = Aleo::generator();
            // ANCHOR_END: aleo_generator
        }
        {
            // ANCHOR: aleo_generator_powers
            let powers: [group; 251] = Aleo::generator_powers();
            // ANCHOR_END: aleo_generator_powers
        }
        return 0field;
    }

    fn assert_demos() {
        {
            // ANCHOR: assert_op
            let a: bool = true;
            let b: bool = false;

            assert(a); // will not halt
            assert(b); // program halts
            // ANCHOR_END: assert_op
        }
        {
            // ANCHOR: assert_eq
            let a: u8 = 1u8;
            let b: u8 = 2u8;

            assert_eq(a, a); // will not halt
            assert_eq(a, b); // program halts
            // ANCHOR_END: assert_eq
        }
        {
            // ANCHOR: assert_neq
            let a: u8 = 1u8;
            let b: u8 = 2u8;

            assert_neq(a, b); // will not halt
            assert_neq(a, a); // program halts
            // ANCHOR_END: assert_neq
        }
        {
            // ANCHOR: ternary
            let a: u8 = true ? 1u8 : 2u8; // 1u8
            // ANCHOR_END: ternary
        }
    }

    fn serialize_demos() {
        {
            // ANCHOR: to_bits
            // Standard serialization (includes type metadata)
            let value: u32 = 1u32;
            let bits: [bool; 58] = Serialize::to_bits(value);

            // Raw serialization (no metadata, just raw bits)
            let bits2: [bool; 32] = Serialize::to_bits_raw(value);

            // Works with arrays too
            let bits3: [bool; 128] = Serialize::to_bits_raw([1u32, 2u32, 3u32, 4u32]);
            // ANCHOR_END: to_bits
        }
        {
            // ANCHOR: from_bits
            // Standard deserialization (includes type metadata)
            let bits1: [bool; 58] = Serialize::to_bits(1u32);
            let value1: u32 = Deserialize::from_bits::[u32](bits1);

            // Raw deserialization (no metadata, just raw bits)
            let bits2: [bool; 32] = Serialize::to_bits_raw(1u32);
            let value2: u32 = Deserialize::from_bits_raw::[u32](bits2);

            // Works with arrays too
            let bits3: [bool; 128] = Serialize::to_bits_raw([1u32, 2u32, 3u32, 4u32]);
            let arr: [u32; 4] = Deserialize::from_bits_raw::[[u32; 4]](bits3);
            // ANCHOR_END: from_bits
        }
    }

    // ANCHOR: self_address
    fn get_program_address() -> address {
        return self.address;
    }
    // ANCHOR_END: self_address

    // ANCHOR: self_caller
    fn matches_caller(addr: address) -> bool {
        return self.caller == addr;
    }
    // ANCHOR_END: self_caller

    // ANCHOR: self_signer
    fn matches_signer(addr: address) -> bool {
        return self.signer == addr;
    }
    // ANCHOR_END: self_signer

    // ANCHOR: block_height
    fn matches_height(height: u32) -> Final {
        return final {
            assert_eq(height, block.height);
        };
    }
    // ANCHOR_END: block_height

    // ANCHOR: block_timestamp
    fn matches_timestamp(timestamp: i64) -> Final {
        return final {
            assert_eq(timestamp, block.timestamp);
        };
    }
    // ANCHOR_END: block_timestamp

    // ANCHOR: self_checksum
    fn matches_checksum(checksum: [u8; 32]) -> Final {
        return final {
            assert_eq(self.checksum, checksum);
        };
    }
    // ANCHOR_END: self_checksum

    // ANCHOR: self_edition
    fn matches_edition(edition: u16) -> Final {
        return final {
            assert_eq(self.edition, edition);
        };
    }
    // ANCHOR_END: self_edition

    // ANCHOR: self_program_owner
    fn matches_owner(owner: address) -> Final {
        return final {
            assert_eq(self.program_owner, owner);
        };
    }
    // ANCHOR_END: self_program_owner

    @noupgrade
    constructor() {}
}
