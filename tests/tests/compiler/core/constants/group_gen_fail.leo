
program test.aleo {
    fn main(
        group_value: group,
    ) -> group {
        let a: group = group::GENERATOR;

        return group_value + a;
    }
}
