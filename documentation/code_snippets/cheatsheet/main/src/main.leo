import external_program.aleo;

// ANCHOR: struct_definition
struct Message {
    sender: address,
    object: u64,
}
// ANCHOR_END: struct_definition

// ANCHOR: const_generics_struct
struct Matrix::[N: u32, M: u32] {
    data: [field; N * M],
}

// Usage
fn make_matrix() -> Matrix::[2u32, 2u32] {
    return Matrix::[2u32, 2u32] { data: [0field, 1field, 2field, 3field] };
}
// ANCHOR_END: const_generics_struct

struct Point {
    x: u32,
    y: u32,
}

// ANCHOR: helper_fn
fn compute(a: u64, b: u64) -> u64 {
    return a + b;
}
// ANCHOR_END: helper_fn

// ANCHOR: final_fn_def
final fn update_balance(receiver: address, amount: u64) {
    let current: u64 = balances.get_or_use(receiver, 0u64);
    balances.set(receiver, current + amount);
}
// ANCHOR_END: final_fn_def

program cheatsheet.aleo {
    // ANCHOR: record_definition
    record Token {
        owner: address,
        amount: u64,
    }
    // ANCHOR_END: record_definition

    record User {
        owner: address,
        balance: u64,
    }

    mapping balances: address => u64;

    storage var: u8;

    storage vec: [u8];

    fn primitives_demo() {
        // ANCHOR: primitives
        // Boolean value (true or false)
        let b: bool = false;

        // Signed 32-bit integer (also available: i8, i16, i64, i128)
        let i: i32 = -10i32;

        // Unsigned 32-bit integer (also available: u8, u16, u64, u128)
        let ui: u32 = 10u32;

        // Field element (used in cryptographic computations)
        let a: field = 1field;

        // Group element (used in elliptic curve operations)
        let g: group = 0group;

        // Scalar element (used in elliptic curve arithmetic)
        let s: scalar = 1scalar;

        // Aleo blockchain address
        let receiver: address = aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4;

        // Digital signature (used for authentication and verification)
        let sig: signature = sign1ftal5ngunk4lv9hfygl45z35vqu9cufqlecumke9jety3w2s6vqtjj4hmjulh899zqsxfxk9wm8q40w9zd9v63sqevkz8zaddugwwq35q8nghcp83tgntvyuqgk8yh0temt6gdqpleee0nwnccxfzes6pawcdwyk4f70n9ecmz6675kvrfsruehe27ppdsxrp2jnvcmy2wws6sw0egv;
        // ANCHOR_END: primitives
    }

    fn type_casting_demo() {
        // ANCHOR: type_casting
        // Casting between integer types
        let a: u8 = 255u8;
        let b: u16 = a as u16; // 255u8 to 255u16
        let c: u32 = b as u32; // 255u16 to 255u32
        let d: i32 = c as i32; // 255u32 to 255i32

        // Casting between field and integers
        let f: field = 10field;
        let i: i32 = f as i32; // Convert field to i32
        let u: u64 = f as u64; // Convert field to u64

        // Casting between scalar and field
        let s: scalar = 5scalar;
        let f_from_scalar: field = s as field; // Convert scalar to field

        // Casting between group and field
        let g: group = group::GEN;
        let f_from_group: field = g as field; // Convert group to field

        // Address casting (only valid conversions)
        let addr: address = aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4;
        let addr_field: field = addr as field; // Convert address to field
        // ANCHOR_END: type_casting
    }

    fn option_primitives_demo() {
        // ANCHOR: option_primitives
        // Boolean option value (true or false or none)
        let b_some: bool? = true;
        let b_none: bool? = none;
        // Unwrapping option values
        let b_true = b_some.unwrap();
        let b_false = b_none.unwrap_or(false);

        // Signed 32-bit integer option (also available: i8, i16, i64, i128)
        let i_some: i32? = -10i32;
        let i_none: i32? = none;

        // Unsigned 32-bit integer option (also available: u8, u16, u64, u128)
        let ui_some: u32? = 10u32;
        let ui_none: u32? = none;

        // Field element option (used in cryptographic computations)
        let a_some: field? = 1field;
        let a_none: field? = none;

        // Group element option (used in elliptic curve operations)
        let g_some: group? = 0group;
        let g_none: group? = none;

        // Scalar element option (used in elliptic curve arithmetic)
        let s_some: scalar? = 1scalar;
        let s_none: scalar? = none;
        // ANCHOR_END: option_primitives
    }

    fn record_demo() -> User {
        // ANCHOR: record_creation
        let user: User = User {
            owner: aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4,
            balance: 1000u64,
        };
        // ANCHOR_END: record_creation

        // ANCHOR: record_access
        let user_address: address = user.owner;
        let user_balance: u64 = user.balance;
        // ANCHOR_END: record_access

        return user;
    }

    fn struct_demo() {
        // ANCHOR: struct_creation
        let msg: Message = Message {
            sender: aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4,
            object: 42u64,
        };
        // ANCHOR_END: struct_creation

        // ANCHOR: struct_access
        let sender_address: address = msg.sender;
        let object_value: u64 = msg.object;
        // ANCHOR_END: struct_access
    }

    fn option_struct_demo() {
        // ANCHOR: option_struct
        let point1: Point? = Point {
            x: 8u32,
            y: 41u32,
        };
        let point2: Point? = none;

        let point1_val = point1.unwrap();
        let point2_val = point2.unwrap_or(Point { x: 0u32, y: 0u32 });
        // ANCHOR_END: option_struct
    }

    fn arrays_demo() {
        // ANCHOR: array_decl
        let arrb: [bool; 2] = [true, false];
        let arr: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
        let empty: [u8; 0] = [];
        // ANCHOR_END: array_decl

        // ANCHOR: array_access
        let first: u8 = arr[0]; // Get the first element
        let second: u8 = arr[1]; // Get the second element
        // ANCHOR_END: array_access

        // ANCHOR: array_loop
        let numbers: [u32; 3] = [5u32, 10u32, 15u32];

        let sum: u32 = 0u32;

        for i: u8 in 0u8..3u8 {
            sum += numbers[i];
        }
        // ANCHOR_END: array_loop
    }

    fn tuples_demo() {
        // ANCHOR: tuple_decl
        // NOTE: Tuples cannot be empty!
        let t: (u8, bool, field) = (42u8, true, 100field);
        // ANCHOR_END: tuple_decl

        // ANCHOR: tuple_access
        // Using de-structuring
        let (a, b, c) = t;

        //Using index-based accessing
        let first: u8 = t.0;
        let second: bool = t.1;
        let third: field = t.2;
        // ANCHOR_END: tuple_access
    }

    fn loops_demo() {
        // ANCHOR: loops
        let count: u32 = 0u32;

        for i: u32 in 0u32..5u32 {
            count += 1u32;
        }
        // ANCHOR_END: loops
    }

    fn conditionals_demo() {
        // ANCHOR: conditionals
        let a: u8 = 1u8;

        if a == 1u8 {
            a += 1u8;
        } else if a == 2u8 {
            a += 2u8;
        } else {
            a += 3u8;
        }

        a = (a == 1u8) ? a + 1u8 : ((a == 2u8) ? a + 2u8 : a + 3u8); // Ternary format
        // ANCHOR_END: conditionals
    }

    fn mappings_demo(receiver: address) -> Final {
        return final {
            // ANCHOR: mappings
            // Querying
            let contains_bal: bool = balances.contains(receiver);
            let get_bal: u64 = balances.get(receiver);
            let get_or_use_bal: u64 = balances.get_or_use(receiver, 0u64);

            // Modifying
            balances.set(receiver, 100u64);
            balances.remove(receiver);

            // External mappings (read-only)
            let ext_contains: bool = external_program.aleo::balances.contains(receiver);
            let ext_get: u64 = external_program.aleo::balances.get(receiver);
            let ext_get_or_use: u64 = external_program.aleo::balances.get_or_use(receiver, 0u64);
            // ANCHOR_END: mappings
        };
    }

    fn storage_var_demo() -> Final {
        return final {
            // ANCHOR: storage_var
            // Querying
            let unwrap_var: u8 = var.unwrap();
            let unwrap_or_var: u8 = var.unwrap_or(0u8);

            // Modifying
            var = 8u8;
            var = none;

            // External storage variables (read-only)
            let ext_var: u8 = external_program.aleo::var.unwrap();
            let ext_var_safe: u8 = external_program.aleo::var.unwrap_or(0u8);
            // ANCHOR_END: storage_var
        };
    }

    fn storage_vec_demo(idx: u32, value: u8) -> Final {
        return final {
            // ANCHOR: storage_vec
            // Querying
            let len_vec: u32 = vec.len();
            let val: u8? = vec.get(idx);

            // Modifying
            vec.set(idx, value);
            vec.push(value);
            vec.pop();
            vec.swap_remove(idx);
            vec.clear();

            // External storage vectors (read-only)
            let ext_len: u32 = external_program.aleo::vec.len();
            let ext_val: u8? = external_program.aleo::vec.get(idx);
            // ANCHOR_END: storage_vec
        };
    }

    fn standard_operators_demo(a: u64, b: u64, value: u64, boolean: bool) -> Final {
        // ANCHOR: standard_operators
        // Arithmetic Operators
        let sum: u64 = a + b; // addition (also has wrapped variant)
        let diff: u64 = a - b; // subtraction (also has wrapped variant)
        let prod: u64 = a * b; // multiplication (also has wrapped variant)
        let quot: u64 = a / b; // division (also has wrapped variant)
        let power: u64 = a ** (b as u8); // exponentiation (also has wrapped variant)
        let remainder: u64 = a % b; // remainder (also has wrapped variant)
        let neg: i64 = -(a as i64); // negation
        let abs: i64 = neg.abs(); // absolute value (also has wrapped variant)

        // Bitwise/Boolean Operators
        let logical_and: bool = boolean && boolean; // logical AND
        let logical_or: bool = boolean || boolean; // logical OR
        let bitwise_and: u64 = a & b; // bitwise AND
        let bitwise_or: u64 = a | b; // bitwise OR
        let bitwise_xor: u64 = a ^ b; // bitwise XOR
        let bitwise_not: u64 = !a; // bitwise NOT
        let bitwise_shl: u64 = a << (b as u32); // bitwise shift left (also has wrapped variant)
        let bitwise_shr: u64 = a >> (b as u32); // bitwise shift right (also has wrapped variant)

        // Comparators
        let eq: bool = a == b; // equality
        let neq: bool = a != b; // non-equality
        let lt: bool = a < b; // less than
        let lte: bool = a <= b; // less than or equal
        let gt: bool = a > b; // greater than
        let gte: bool = a >= b; // greater than or equal

        // Group & Field Operators
        let g: group = group::GEN; // the group generator
        let x: field = 0group.to_x_coordinate(); // x-coordinate of a group element
        let y: field = 0group.to_y_coordinate(); // y-coordinate of a group element
        let doubled: group = (2group).double(); // Doubles the field/group element
        let inverse: field = 1field.inv(); // Multiplicative inverse of the field/group element
        let squared: field = 1field.square(); // Square of the field/group element
        let root: field = 1field.square_root(); // Square root of the field/group element

        // Off-chain context expressions
        let this: address = self.address; // Address of program
        let caller: address = self.caller; // Address of function caller
        let signer: address = self.signer; // Address of tx signer (origin)

        // Bit Serialization/Deserialization
        let bits: [bool; 90] = Serialize::to_bits(value); // Standard serialization (includes type metadata)
        let raw_bits: [bool; 64] = Serialize::to_bits_raw(value); // Raw serialization (no metadata, just raw bits)

        // Miscellaneous
        assert(boolean); // assert the value of boolean is true
        assert_eq(a, b); // assert a and b are equal
        assert_neq(a, value); // assert a and value are not equal
        let ternary = boolean ? a : b; // Ternary expression

        return final {
            // On-chain context expressions
            let height: u32 = block.height; // Height of current block
            let now: i64 = block.timestamp; // Timestamp of current block
            let checksum: [u8; 32] = self.checksum; // Checksum of a program
            let edition: u16 = self.edition; // Edition of a program
            let owner: address = self.program_owner; // Address that deployed a program
        };
        // ANCHOR_END: standard_operators
    }

    fn crypto_operators_demo(
        sig: signature,
        addr: address,
        ecdsa_sig: [u8; 65],
        ecdsa_addr: [u8; 33],
        eth_addr: [u8; 20],
        digest: [u8; 32],
        msg: [u8; 32],
    ) -> Final {
        return final {
            // ANCHOR: crypto_operators
            // Randomization
            let rand: u32 = ChaCha::rand_u32(); // generate a random value `ChaCha::rand_<type>()`

            // Hash Functions (BHP, Pedersen, Poseidon, Keccak, SHA3)
            let hash: field = BHP256::hash_to_field(1u32); // hash any type to any type
            let hash_raw: address = Poseidon2::hash_to_address_raw(1u8); // hash any raw type to any type
            let hash_native: [bool; 256] = Keccak256::hash_to_bits(0u64); // hash any type to an array of bits (only available for Keccak and SHA3)
            let hash_native_raw: [bool; 256] = Keccak256::hash_to_bits_raw(0u64); // hash any raw type to an array of bits (only available for Keccak and SHA3)

            // Commitment Algorithms (BHP, Pedersen)
            let commit: group = Pedersen64::commit_to_group(1u32, 1scalar); // commit any type to a field, group, or address, using a scalar as blinding factor (salt)

            // Schnorr Signatures
            let schnorr: bool = signature::verify(sig, addr, 0field); // Schnorr Signature Verification

            // ECDSA Signatures (Keccak, SHA3)
            let ecdsa: bool = ECDSA::verify_keccak256(ecdsa_sig, ecdsa_addr, msg); // Verify an ECDSA signature against an ECDSA public key and the Keccak256 hash of a message
            let ecdsa_raw: bool = ECDSA::verify_keccak256_raw(ecdsa_sig, ecdsa_addr, msg); // Verify an ECDSA signature against an ECDSA public key and the Keccak256 hash of a raw message
            let ecdsa_eth: bool = ECDSA::verify_keccak256_eth(ecdsa_sig, eth_addr, msg); // Verify an ECDSA signature against an Ethereum address and the Keccak256 hash of a raw message

            let ecdsa_digest: bool = ECDSA::verify_digest(ecdsa_sig, ecdsa_addr, digest); // Verify an ECDSA signature against an ECDSA public key and a prehashed message
            let ecdsa_digest_eth: bool = ECDSA::verify_digest_eth(ecdsa_sig, eth_addr, digest); // Verify an ECDSA signature against an Ethereum address and a prehashed message
            // ANCHOR_END: crypto_operators
        };
    }

    @noupgrade
    constructor() {}
}
