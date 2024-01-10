// The 'b' program.
import child.aleo;
program parent.aleo {
    transition wrapper_mint(owner: address, val: u32) ->  child.aleo/A {
        return child.aleo/mint(aleo1q30lfyggefvzzxqaaclzrn3wd94q4u8zzy8jhhfrcqrf306ayvqsdvj7s4, 1u32);
    }
}
