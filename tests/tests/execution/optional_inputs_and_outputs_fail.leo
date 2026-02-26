/*
seed = 123456789
min_height = 16

# === Primitive Optionals ===

[case]
program = "test.aleo"
function = "u64_in_out"
input = ["{is_some:true, val:42u64}"]

[case]
program = "test.aleo"
function = "u64_in_out"
input = ["{is_some:false, val:0u64}"]

[case]
program = "test.aleo"
function = "u8_in_out"
input = ["{is_some:true, val:7u8}"]

[case]
program = "test.aleo"
function = "u8_in_out"
input = ["{is_some:false, val:0u8}"]

[case]
program = "test.aleo"
function = "bool_in_out"
input = ["{is_some:true, val:true}"]

[case]
program = "test.aleo"
function = "bool_in_out"
input = ["{is_some:false, val:false}"]

# === Struct Optionals ===

[case]
program = "test.aleo"
function = "struct_optional"
input = ["{is_some:true, val:{x:{is_some:true, val:99u32}}}"]

[case]
program = "test.aleo"
function = "struct_optional"
input = ["{is_some:true, val:{x:{is_some:false, val:0u32}}}"]

[case]
program = "test.aleo"
function = "struct_optional"
input = ["{is_some:false, val:{x:{is_some:false, val:0u32}}}"]

# === Array Optionals ===

[case]
program = "test.aleo"
function = "array_optional"
input = ["{is_some:true, val:[{is_some:true, val:1u8}, {is_some:false, val:0u8}]}"]

[case]
program = "test.aleo"
function = "array_optional"
input = ["{is_some:false, val:[{is_some:false, val:0u8}, {is_some:false, val:0u8}]}"]
*/

// TODO: this test will currently fail because we don't allow inputs and outputs to have optionals in them.
// This test should pass once that restriction is removed

// === Struct Optionals ===
struct Foo {
    x: u32?,
}

program test.aleo {
    // === Primitive Optionals ===
    fn u64_in_out(x: u64?) -> u64? {
        if x != none {
            assert(x.unwrap() == 42u64);
        }
        return x;
    }

    fn u8_in_out(x: u8?) -> u8? {
        if x != none {
            assert(x.unwrap() == 7u8);
        }
        return x;
    }

    fn bool_in_out(x: bool?) -> bool? {
        if x != none {
            assert(x.unwrap() == true);
        }
        return x;
    }

    fn struct_optional(f: Foo?) -> Foo? {
        if f != none {
            let inner = f.unwrap();
            if inner.x != none {
                assert(inner.x.unwrap() == 99u32);
            }
        }
        return f;
    }

    // === Array Optionals ===
    fn array_optional(arr: [u8?; 2]?) -> [u8?; 2]? {
        if arr != none {
            let unwrapped = arr.unwrap();
            assert(unwrapped[0].unwrap() == 1u8);
            assert(unwrapped[1] == none);
        }
        return arr;
    }

    @noupgrade
    constructor() {}
}
