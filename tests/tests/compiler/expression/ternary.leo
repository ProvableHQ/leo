/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(
        a: address, b: bool, c: field, d: i8, e: i16, f: i64, g: i128, h: u8, i: u16, j: u32, k: u64, l: u128, m: scalar, n: i32
    ) -> (
        address, bool, field, i8, i16, i64, i128, u8, u16, u32, u64, u128, scalar, i32
    ) {
        let one: address = b ? a : a;
        let two: bool = b ? b : b;
        let three: field = b ? c : c;
        let four: i8 = b ? d : d;
        let five: i16 = b ? e : e;
        let six: i64 = b ? f : f;
        let seven: i128 = b ? g : g;
        let eight: u8 = b ? h : h;
        let nine: u16 = b ? i : i;
        let ten: u32 = b ? j : j;
        let eleven: u64 = b ? k : k;
        let twelve: u128 = b ? l : l;
        let thirteen: scalar = b ? m : m;
        let fourteen: i32 = b ? n : n;
        return (one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve, thirteen, fourteen);
    }
}
