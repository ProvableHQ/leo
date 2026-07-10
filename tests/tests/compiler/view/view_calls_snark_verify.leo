// `Snark::verify` and `Snark::verify_batch` are pure instructions (no state mutation), so they are
// allowed inside a `view fn`, exactly as they are inside a `final fn`. Executing them requires real
// proof bytes (see the compile-only `core/algorithms/snark_verify` test), so this only checks that
// the type checker accepts them in a view and that codegen emits valid Aleo.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn check(
        vk: [u8; 128],
        varuna_version: u8,
        inputs: [field; 4],
        proof: [u8; 256],
        vks: [[u8; 128]; 2],
        batch_inputs: [[[field; 4]; 3]; 2],
    ) -> bool {
        let single: bool = Snark::verify(vk, varuna_version, inputs, proof);
        let batch: bool = Snark::verify_batch(vks, varuna_version, batch_inputs, proof);
        return single && batch;
    }

    @noupgrade
    constructor() {}
}
