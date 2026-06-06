program a_dep.aleo {
    fn one() -> u8 {
        return 1u8;
    }
}

// --- Next Program --- //
program z_dep.aleo {
    fn two() -> u8 {
        return 2u8;
    }
}

// --- Next Program --- //
import z_dep.aleo;
import a_dep.aleo;

program sorted_import_bytecode.aleo {
    fn main() -> u8 {
        return a_dep.aleo::one() + z_dep.aleo::two();
    }
}
