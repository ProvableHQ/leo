// The 'nested' program.
import nested_example_layer_0.aleo;
program nested.aleo {
    transition example(public a: u32, b: u32) -> u32 {
        let c: u32 = nested_example_layer_0.aleo/main(a, b);
        return c;
    }
}
