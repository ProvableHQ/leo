
program test.aleo {
    fn main(
        group_value: group,
    ) -> group {
        let a: group = group::GEN;

        return group_value + a;
    }
}
