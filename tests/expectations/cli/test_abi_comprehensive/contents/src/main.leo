// Comprehensive ABI generation test program.
// Exercises ABI features: primitives, arrays, structs, records, mappings, storage variables, transitions, modules.

program abi_test.aleo {

    // === Struct Definitions ===

    // This struct is NOT used in any transition, mapping, or storage.
    // It should be pruned from the ABI.
    struct Unused {
        x: u32,
    }

    // Simple struct with primitives
    struct Point {
        x: i32,
        y: i32,
    }

    // Struct with various primitive types
    struct AllPrimitives {
        f_address: address,
        f_bool: bool,
        f_field: field,
        f_group: group,
        f_scalar: scalar,
        f_i8: i8,
        f_i16: i16,
        f_i32: i32,
        f_i64: i64,
        f_i128: i128,
        f_u8: u8,
        f_u16: u16,
        f_u32: u32,
        f_u64: u64,
        f_u128: u128,
    }

    // Nested struct
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    // === Record Definitions ===

    // Record with public and private fields
    record Token {
        owner: address,
        public amount: u64,
        private data: field,
    }

    // === Mappings ===

    // Simple mapping
    mapping balances: address => u64;

    // Mapping with struct value
    mapping positions: address => Point;

    // Mapping with array value
    mapping arrays: u32 => [field; 2];

    // === Storage Variables ===

    // Simple storage variable
    storage counter: u32;

    // Storage variable with struct type
    storage last_point: Point;

    // Storage vector
    storage history: [u64];

    // === Transitions ===

    // Single primitive input/output
    transition single_primitive(a: u32) -> u32 {
        return a + 1u32;
    }

    // Multiple inputs, single output
    transition multi_input(a: u32, b: u32, c: u32) -> u32 {
        return a + b + c;
    }

    // Multiple outputs
    transition multi_output(a: u32) -> (u32, u32) {
        return (a, a + 1u32);
    }

    // All visibility modes
    transition visibility_modes(public a: u32, private b: u32, c: u32) -> public u32 {
        return a + b + c;
    }

    // Struct input/output
    transition struct_io(p: Point) -> Point {
        return Point { x: p.x + 1i32, y: p.y + 1i32 };
    }

    // Nested struct
    transition nested_struct_io(r: Rectangle) -> Rectangle {
        return r;
    }

    // Array input/output
    transition array_io(arr: [u32; 4]) -> [u32; 4] {
        return arr;
    }

    // Nested array
    transition nested_array_io(arr: [[u32; 2]; 3]) -> [[u32; 2]; 3] {
        return arr;
    }

    // Record input
    transition record_input(t: Token) -> u64 {
        return t.amount;
    }

    // Record output
    transition record_output(owner: address, amount: u64, data: field) -> Token {
        return Token { owner, amount, data };
    }

    // All primitives transition
    transition all_primitives_io(p: AllPrimitives) -> AllPrimitives {
        return p;
    }

    // Module struct input/output
    transition module_struct_io(v: utils::Vector3) -> utils::Vector3 {
        return v;
    }

    // Async transition with finalize
    async transition async_with_finalize(public amount: u64) -> Future {
        return finalize_async(amount);
    }

    async function finalize_async(amount: u64) {
        let dummy: u64 = amount + 1u64;
    }

    // Async transition that updates mapping
    async transition update_balance(public user: address, public amount: u64) -> Future {
        return finalize_update_balance(user, amount);
    }

    async function finalize_update_balance(user: address, amount: u64) {
        Mapping::set(balances, user, amount);
    }

    @noupgrade
    async constructor() {}
}
