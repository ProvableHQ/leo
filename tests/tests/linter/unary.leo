
program my.aleo {
    transition main() {
        let a = !true;
        let b = !!false;
        assert(a == false);
        assert(b == false);
        return;
    }
}
