function main(a: bool, b: bool) {
    let c = if a ? true : false;

    assert_eq!(c, b);
}