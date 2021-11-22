/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

struct TestMe {
    x: u8;

    function test_me(&self) -> u8 {
        self.x += 1;
        return self.x;
    }

    function new() -> Self {
        return Self { x: 1u8 };
    }
}

function my_fn() -> TestMe {
  return TestMe { x: 0u8 };
}

function main(y: bool) -> bool {
    let t = TestMe {x: 6u8}.test_me();
    let u = my_fn().test_me();
    let v = TestMe::new().test_me();
    return (v == 2u8) == y;
}
