struct Foo {
  x: u32
  y: u32
}

function main(a: private fe) {
    let b = a + 1fe;
    assert_eq(b, 2fe);

    let c = Foo {
        x: 4,
        y: 5,
    };

}