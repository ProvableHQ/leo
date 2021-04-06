circuit TestMe {
    x: u8,

    function test_me(mut self) -> u8 {
        self.x += 1;
        return self.x
    }
}

function main () {
    const t = TestMe {x: 6u8}.test_me();
}