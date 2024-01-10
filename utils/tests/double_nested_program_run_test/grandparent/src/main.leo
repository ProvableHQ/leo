// The 'import_example' program.
import child.aleo;
import parent.aleo;
program grandparent.aleo {
    transition double_wrapper_mint(owner: address, val: u32) -> child.aleo/A {
        return parent.aleo/wrapper_mint(owner,val);
    }
}
