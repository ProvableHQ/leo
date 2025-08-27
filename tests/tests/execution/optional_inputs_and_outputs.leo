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

program test.aleo {
    // === Primitive Optionals ===
    transition u64_in_out(x: u64?) -> u64? {
        if x != None {
            assert(x.unwrap() == 42u64);
        }
        return x;
    }

    transition u8_in_out(x: u8?) -> u8? {
        if x != None {
            assert(x.unwrap() == 7u8);
        }
        return x;
    }

    transition bool_in_out(x: bool?) -> bool? {
        if x != None {
            assert(x.unwrap() == true);
        }
        return x;
    }

    // === Struct Optionals ===
    struct Foo {
        x: u32?,
    }

    transition struct_optional(f: Foo?) -> Foo? {
        if f != None {
            let inner = f.unwrap();
            if inner.x != None {
                assert(inner.x.unwrap() == 99u32);
            }
        }
        return f;
    }

    // === Array Optionals ===
    transition array_optional(arr: [u8?; 2]?) -> [u8?; 2]? {
        if arr != None {
            let unwrapped = arr.unwrap();
            assert(unwrapped[0].unwrap() == 1u8);
            assert(unwrapped[1] == None);
        }
        return arr;
    }

    @noupgrade
    async constructor() {}
}
