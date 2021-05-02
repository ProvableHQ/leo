import test-import.foo as bar;
import bar.baz.ONE as UNO;

function main() {
    console.assert(bar() == 1u32);
    console.assert(UNO == 1u8);
}
