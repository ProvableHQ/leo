// The 'test_deploy_leo_program' program.
program test_deploy_leo_program.aleo {
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}
