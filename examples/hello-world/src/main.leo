// The 'hello-world' main function.
type str = [char; _];

function main(y: bool) -> bool {
    let s = "abc";
    let a: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j9;
    return (first_el(s) == 'a') == y;
}

function first_el(s: str) -> char {
    return s[0];
}
