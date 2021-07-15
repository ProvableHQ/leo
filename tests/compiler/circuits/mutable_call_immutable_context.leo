/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit TestMe {
    x: u8;

    function test_me(mut self) -> u8 {
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
    const t = TestMe {x: 6u8}.test_me();
    const u = my_fn().test_me();
    const v = TestMe::new().test_me();
    return (v == 2u8) == y;
}
