struct Foo {
    b: u32
}

function main() {
    let a = 1u8 + 1u8;
    let c = 1u16 + 1u16;
    let b = 1u32 + 1;
    let d = 1u64 + 1u64;
    let e = 1u128 + 1u128;
    let f = true && false;
    let g = 5fe + 10fe;
    let h: u32[2] = [1, 2];

    let i = Foo { b: 5 };
    let j = [1, 4, 5];
}